use crate::container::Container;
use crate::format_err;
use bytes::{Buf, Bytes};
use futures::{ready, AsyncRead, AsyncReadExt, Stream, StreamExt};
use napi::bindgen_prelude::*;
use o2o::o2o;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(o2o)]
#[owned_into(bollard::container::StatsOptions)]
#[napi(object)]
pub struct StatsOptions {
    /// Stream the output. If false, the stats will be output once and then it will disconnect.
    pub stream: bool,
    /// Only get a single stat instead of waiting for 2 cycles. Must be used with `stream = false`.
    pub one_shot: bool,
}

#[napi]
impl Container {
    #[napi]
    pub fn stats(&self, option: Option<StatsOptions>) -> Result<StatsStream> {
        let output = self.docker.stats(&self.id, option.map(Into::into));
        Ok(StatsStream {
            inner: Box::pin(output),
            buf: Bytes::new(),
            pos: 0,
        })
    }
}

#[napi]
pub struct StatsStream {
    inner: Pin<
        Box<
            dyn Stream<
                    Item = std::result::Result<bollard::container::Stats, bollard::errors::Error>,
                > + Send,
        >,
    >,
    buf: Bytes,
    pos: usize,
}

#[napi]
impl StatsStream {
    #[napi]
    pub async unsafe fn read(&mut self, mut buf: Buffer) -> Result<usize> {
        let buf = buf.as_mut();
        let n = AsyncReadExt::read(self, buf).await.map_err(format_err)?;
        Ok(n)
    }
}

impl AsyncRead for StatsStream {
    //noinspection DuplicatedCode
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

            this.buf = match ready!(this.inner.poll_next_unpin(cx)) {
                None => return Poll::Ready(Ok(0)),
                Some(Ok(stats)) => match serde_json::to_vec(&stats) {
                    Ok(vec) => Bytes::from(vec),
                    Err(_) => {
                        return Poll::Ready(Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Failed to serialize to bytes",
                        )))
                    }
                },
                Some(Err(err)) => {
                    return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, err)))
                }
            };

            // -, - ugly hack
            if this.pos > 0 {
                buf[0] = b'\r';
                buf[1] = b'\n';
                return Poll::Ready(Ok(2));
            }
        }
    }
}
