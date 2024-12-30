use crate::container::Container;
use crate::format_err;
use bollard::container::{AttachContainerResults, LogOutput};
use bytes::{Buf, Bytes};
use futures::{AsyncRead, AsyncReadExt, Stream, StreamExt};
use napi::bindgen_prelude::*;
use napi::tokio::io::{AsyncWrite, AsyncWriteExt};
use o2o::o2o;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

type AttachSource =
    Pin<Box<dyn Stream<Item = std::result::Result<LogOutput, bollard::errors::Error>> + Send>>;
type AttachSink = Pin<Box<dyn AsyncWrite + Send>>;

#[napi]
pub struct AttachOutput {
    source: LogOutputReader,
    sink: AttachSink,
}

#[napi]
impl AttachOutput {
    pub fn new(output: AttachSource, input: AttachSink) -> Self {
        Self {
            source: LogOutputReader {
                stream: output,
                pos: 0,
                buf: Bytes::new(),
            },
            sink: input,
        }
    }

    #[napi]
    pub async unsafe fn write(&mut self, mut buf: Buffer) -> Result<usize> {
        let buf = buf.as_mut();
        let n = self.sink.write(buf).await.map_err(format_err)?;
        Ok(n)
    }

    #[napi(js_name = "close")]
    pub async unsafe fn shutdown_writer(&mut self) -> Result<()> {
        self.sink.shutdown().await?;
        Ok(())
    }

    #[napi]
    pub async unsafe fn read(&mut self, mut buf: Buffer) -> Result<usize> {
        let buf = buf.as_mut();
        let n = self.source.read(buf).await.map_err(format_err)?;
        Ok(n)
    }
}

pub struct LogOutputReader {
    pub stream: AttachSource,
    pub pos: usize,
    pub buf: Bytes,
}

impl AsyncRead for LogOutputReader {
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

/// ## Notice
///
/// I can't figure out the detach-key abilities, if I set `ctrl-c`
/// which means
/// ```rust
/// self.sink.write_i8(0x03).await?;
/// ```
///
/// > 0x3 means ascii `^C`
///
/// will work?
#[derive(o2o)]
#[owned_into(bollard::container::AttachContainerOptions::<String>)]
#[ghosts(detach_keys: {None})]
#[napi(object)]
pub struct AttachOptions {
    pub stdin: Option<bool>,
    pub stdout: Option<bool>,
    pub stderr: Option<bool>,
    pub stream: Option<bool>,
    pub logs: Option<bool>,
}

#[napi]
impl Container {
    #[napi]
    pub async fn attach(&self, option: Option<AttachOptions>) -> Result<AttachOutput> {
        let AttachContainerResults { output, input } = self
            .docker
            .attach_container(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)?;

        Ok(AttachOutput::new(output, input))
    }
}
