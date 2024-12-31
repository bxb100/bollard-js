use crate::container::{Container, DownloadStream, FutureBytesRead};
use bytes::Bytes;
use napi::bindgen_prelude::*;

#[napi]
impl Container {
    #[napi]
    pub fn export(&self) -> Result<DownloadStream> {
        let stream = self.docker.export_container(&self.id);

        Ok(DownloadStream {
            inner: FutureBytesRead {
                inner: Box::pin(stream),
                pos: 0,
                buf: Bytes::new(),
            },
        })
    }
}
