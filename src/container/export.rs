use crate::container::{Container, ReadStream};
use crate::types::CommonFutureRead;
use napi::bindgen_prelude::*;

#[napi]
impl Container {
    #[napi]
    pub fn export(&self) -> Result<ReadStream> {
        let stream = self.docker.export_container(&self.id);

        Ok(ReadStream {
            inner: CommonFutureRead::new(stream),
        })
    }
}
