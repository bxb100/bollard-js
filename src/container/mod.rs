mod attach;
mod changes;
mod commit;
mod create;
mod exec;
mod export;
mod get_archive;
mod inspect;
mod kill;
mod list;
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

use crate::impl_async_read;
use crate::types::CommonFutureRead;
use bollard::Docker;
use bytes::Bytes;
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

impl_async_read!(ReadStream);
