use crate::image::Image;
use crate::impl_async_read;
use crate::types::{CommonFutureRead, DockerCredentials, ToBytes};
use bytes::Bytes;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::image::PushImageOptions::<String>)]
#[napi(object)]
pub struct PushImageOptions {
    /// The tag to associate with the image on the registry.
    pub tag: String,
}

#[napi]
impl Image {
    #[napi]
    pub fn push(
        &self,
        option: Option<PushImageOptions>,
        credentials: Option<DockerCredentials>,
    ) -> Result<PushImageInfoStream> {
        let stream = self.docker.push_image(
            &self.id,
            option.map(Into::into),
            credentials.map(Into::into),
        );

        Ok(PushImageInfoStream {
            inner: CommonFutureRead::new(stream),
        })
    }
}

#[napi]
pub struct PushImageInfoStream {
    inner: CommonFutureRead<bollard::models::PushImageInfo>,
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

impl_async_read!(PushImageInfoStream);
