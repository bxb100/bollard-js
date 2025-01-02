use crate::container::{Container, ReadStream};
use crate::types::CommonFutureRead;
use bytes::Bytes;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::DownloadFromContainerOptions::<String>)]
#[napi(object)]
pub struct DownloadFromContainerOptions {
    /// Resource in the container’s filesystem to archive.
    pub path: String,
}

#[napi]
impl Container {
    #[napi]
    pub fn get_archive(&self, option: Option<DownloadFromContainerOptions>) -> Result<ReadStream> {
        let stream = self
            .docker
            .download_from_container(&self.id, option.map(Into::into));

        Ok(ReadStream {
            inner: CommonFutureRead {
                stream: Box::pin(stream),
                pos: 0,
                buf: Bytes::new(),
            },
        })
    }
}
