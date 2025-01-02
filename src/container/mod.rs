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
use crate::types::{CommonFutureRead, ToBytes};
use bollard::Docker;
use bytes::Bytes;
use futures::AsyncReadExt;
use napi::bindgen_prelude::Buffer;

#[napi]
#[derive(Clone)]
pub struct Container {
    docker: Docker,
    pub id: String,
}

#[napi]
pub struct ReadStream {
    inner: CommonFutureRead<Bytes>,
}

impl ToBytes for Bytes {
    fn with_eol() -> bool {
        false
    }

    fn to_bytes(self) -> std::io::Result<Bytes> {
        Ok(self)
    }
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
