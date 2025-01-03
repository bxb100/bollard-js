use crate::format_err;
use bollard::container::LogOutput;
use bytes::{Buf, Bytes};
use futures::{AsyncRead, AsyncReadExt, Stream, StreamExt};
use napi::bindgen_prelude::Buffer;
use napi::tokio::io::{AsyncWrite, AsyncWriteExt};
use o2o::o2o;
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

#[macro_export]
macro_rules! impl_async_read {
    ($name:ident) => {
        #[napi]
        impl $name {
            #[napi]
            pub async unsafe fn read(&mut self, mut buf: Buffer) -> napi::Result<usize> {
                let buf = buf.as_mut();
                let n = futures::AsyncReadExt::read(&mut self.inner, buf)
                    .await
                    .map_err($crate::format_err)?;
                Ok(n)
            }
        }
    };
}

#[napi]
pub struct Output {
    source: CommonFutureRead<LogOutput>,
    sink: Sink,
}

#[napi]
impl Output {
    pub fn new(output: Source, input: Sink) -> Self {
        Self {
            source: CommonFutureRead {
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

pub struct CommonFutureRead<T: ToBytes> {
    pub(crate) stream: Pin<Box<dyn Stream<Item = Result<T, bollard::errors::Error>> + Send>>,
    pub(crate) pos: usize,
    pub(crate) buf: Bytes,
}

impl<T: ToBytes> CommonFutureRead<T> {
    pub fn new(
        stream: impl Stream<Item = Result<T, bollard::errors::Error>> + Send + 'static,
    ) -> Self {
        Self {
            stream: Box::pin(stream),
            pos: 0,
            buf: Bytes::new(),
        }
    }
}

impl<T: ToBytes> AsyncRead for CommonFutureRead<T> {
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
                Some(Ok(item)) => match item.to_bytes() {
                    Ok(bytes) => bytes,
                    Err(err) => return Poll::Ready(Err(err)),
                },
                Some(Err(err)) => {
                    return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, err)));
                }
                None => return Poll::Ready(Ok(0)),
            };

            // -, - ugly hack
            if T::with_eol() && this.pos > 0 {
                buf[0] = b'\r';
                buf[1] = b'\n';
                return Poll::Ready(Ok(2));
            }
        }
    }
}

pub trait ToBytes {
    fn with_eol() -> bool;
    fn to_bytes(self) -> std::io::Result<Bytes>;
}

impl ToBytes for Bytes {
    fn with_eol() -> bool {
        false
    }

    fn to_bytes(self) -> std::io::Result<Bytes> {
        Ok(self)
    }
}

impl ToBytes for LogOutput {
    fn with_eol() -> bool {
        false
    }

    fn to_bytes(self) -> std::io::Result<Bytes> {
        Ok(self.into_bytes())
    }
}

impl ToBytes for bollard::container::Stats {
    fn with_eol() -> bool {
        true
    }

    fn to_bytes(self) -> std::io::Result<Bytes> {
        let bytes = serde_json::to_vec(&self).map(Bytes::from)?;
        Ok(bytes)
    }
}

impl ToBytes for bollard::models::CreateImageInfo {
    fn with_eol() -> bool {
        true
    }

    fn to_bytes(self) -> std::io::Result<Bytes> {
        let bytes = serde_json::to_vec(&self).map(Bytes::from)?;
        Ok(bytes)
    }
}

impl ToBytes for bollard::models::PushImageInfo {
    fn with_eol() -> bool {
        true
    }

    fn to_bytes(self) -> std::io::Result<Bytes> {
        let bytes = serde_json::to_vec(&self).map(Bytes::from)?;
        Ok(bytes)
    }
}

#[derive(o2o)]
#[owned_into(bollard::auth::DockerCredentials)]
#[napi(object)]
pub struct DockerCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
    pub auth: Option<String>,
    pub email: Option<String>,
    pub serveraddress: Option<String>,
    pub identitytoken: Option<String>,
    pub registrytoken: Option<String>,
}
