use crate::image::Image;
use crate::impl_async_read;
use crate::types::CommonFutureRead;
use bytes::Bytes;
use napi::bindgen_prelude::*;

#[napi]
impl Image {
    #[napi]
    pub fn get(&self) -> Result<ExportImageStream> {
        let export_stream = self.docker.export_image(&self.id);

        Ok(ExportImageStream {
            inner: CommonFutureRead::new(export_stream),
        })
    }
}

#[napi]
pub struct ExportImageStream {
    inner: CommonFutureRead<Bytes>,
}

impl_async_read!(ExportImageStream);
