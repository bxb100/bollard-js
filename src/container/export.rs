use crate::container::{Container, FutureBytesRead, ReadStream};
use bytes::Bytes;
use napi::bindgen_prelude::*;

#[napi]
impl Container {
    #[napi]
    pub fn export(&self) -> Result<ReadStream> {
        let stream = self.docker.export_container(&self.id);

        Ok(ReadStream {
            inner: FutureBytesRead {
                inner: Box::pin(stream),
                pos: 0,
                buf: Bytes::new(),
            },
        })
    }
}
