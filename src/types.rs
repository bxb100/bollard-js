use crate::format_err;
use bollard::container::LogOutput;
use bytes::{Buf, Bytes};
use futures::{AsyncRead, AsyncReadExt, Stream, StreamExt};
use napi::bindgen_prelude::Buffer;
use napi::tokio::io::{AsyncWrite, AsyncWriteExt};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

#[napi(object)]
pub struct DockerOptions {
    pub socket_path: Option<String>,
    pub url: Option<String>,
    pub ssl_key: Option<String>,
    pub ssl_cert: Option<String>,
    pub ssl_ca: Option<String>,
}

type Source = Pin<Box<dyn Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send>>;
type Sink = Pin<Box<dyn AsyncWrite + Send>>;

#[napi]
pub struct Output {
    source: FutureRead,
    sink: Sink,
}

#[napi]
impl Output {
    pub fn new(output: Source, input: Sink) -> Self {
        Self {
            source: FutureRead {
                stream: output,
                pos: 0,
                buf: Bytes::new(),
            },
            sink: input,
        }
    }

    #[napi]
    pub async unsafe fn write(&mut self, mut buf: Buffer) -> napi::Result<()> {
        let buf = buf.as_mut();
        self.sink.write_all(buf).await.map_err(format_err)?;
        Ok(())
    }

    #[napi(js_name = "close")]
    pub async unsafe fn shutdown_writer(&mut self) -> napi::Result<()> {
        self.sink.shutdown().await?;
        Ok(())
    }

    #[napi]
    pub async unsafe fn read(&mut self, mut buf: Buffer) -> napi::Result<usize> {
        let buf = buf.as_mut();
        let n = self.source.read(buf).await.map_err(format_err)?;
        Ok(n)
    }
}

pub struct FutureRead {
    pub stream: Source,
    pub pos: usize,
    pub buf: Bytes,
}

impl AsyncRead for FutureRead {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        loop {
            if this.buf.has_remaining() {
                let len = this.buf.remaining().min(buf.len());
                this.buf.copy_to_slice(&mut buf[..len]);
                this.pos += len;
                return Poll::Ready(Ok(len));
            }

            this.buf = match ready!(this.stream.poll_next_unpin(cx)) {
                Some(Ok(item)) => item.into_bytes(),
                Some(Err(err)) => {
                    return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, err)));
                }
                None => return Poll::Ready(Ok(0)),
            }
        }
    }
}
