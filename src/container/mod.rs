mod attach;
mod changes;
mod commit;
mod create;
mod exec;
mod export;
mod get_archive;
mod inspect;
mod kill;
mod logs;
mod pause;
mod put_archive;
mod remove;
mod rename;
mod resize;
mod restart;
mod start;
mod stats;
mod stop;
mod top;
mod unpause;
mod update;
mod wait;

use crate::format_err;
use bollard::Docker;
use bytes::{Buf, Bytes};
use futures::{AsyncRead, AsyncReadExt, Stream, StreamExt};
use napi::bindgen_prelude::Buffer;
use std::pin::Pin;
use std::task::{ready, Poll};

#[napi]
#[derive(Clone)]
pub struct Container {
    docker: Docker,
    pub id: String,
}

#[napi]
pub struct ReadStream {
    inner: FutureBytesRead,
}

#[napi]
impl ReadStream {
    #[napi]
    pub async unsafe fn read(&mut self, mut buf: Buffer) -> napi::Result<usize> {
        let buf = buf.as_mut();
        let n = self.inner.read(buf).await.map_err(format_err)?;
        Ok(n)
    }
}

pub(crate) struct FutureBytesRead {
    pub(crate) inner: Pin<Box<dyn Stream<Item = Result<Bytes, bollard::errors::Error>> + Send>>,
    pub(crate) pos: usize,
    pub(crate) buf: Bytes,
}

impl AsyncRead for FutureBytesRead {
    //noinspection DuplicatedCode
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
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

            this.buf = match ready!(this.inner.poll_next_unpin(cx)) {
                Some(Ok(item)) => item,
                Some(Err(err)) => {
                    return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, err)));
                }
                None => return Poll::Ready(Ok(0)),
            }
        }
    }
}
