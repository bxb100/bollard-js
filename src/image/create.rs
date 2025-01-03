use crate::types::{CommonFutureRead, DockerCredentials, ToBytes};
use crate::{impl_async_read, Docker};
use bytes::Bytes;

use crate::image::Image;
use napi::bindgen_prelude::*;

#[napi(object)]
pub struct CreateImageOptions {
    /// Name of the image to pull. The name may include a tag or digest. This parameter may only be
    /// used when pulling an image. The pull is cancelled if the HTTP connection is closed.
    pub from_image: String,
    /// Source to import. The value may be a URL from which the image can be retrieved or `-` to
    /// read the image from the request body. This parameter may only be used when importing an
    /// image.
    pub from_src: String,
    /// Repository name given to an image when it is imported. The repo may include a tag. This
    /// parameter may only be used when importing an image.
    pub repo: String,
    /// Tag or digest. If empty when pulling an image, this causes all tags for the given image to
    /// be pulled.
    pub tag: String,
    /// Platform in the format `os[/arch[/variant]]`
    pub platform: String,
    /// A list of Dockerfile instructions to be applied to the image being created. Changes must be
    /// URL-encoded! This parameter may only be used when importing an image.
    pub changes: Vec<String>,
}

#[napi]
impl Docker {
    #[napi]
    pub fn create_image(
        &self,
        option: Option<CreateImageOptions>,
        root_fs: Option<Buffer>,
        credentials: Option<DockerCredentials>,
    ) -> Result<CreateImageOutput> {
        let stream = self.0.create_image(
            option.map(|opt| bollard::image::CreateImageOptions {
                from_image: opt.from_image,
                from_src: opt.from_src,
                repo: opt.repo,
                tag: opt.tag,
                platform: opt.platform,
                // Is safe to do this?
                changes: opt.changes.into_iter().map(|s| s.leak() as &str).collect(),
            }),
            root_fs.map(|buf| Bytes::from(buf.to_vec())),
            credentials.map(Into::into),
        );

        Ok(CreateImageOutput {
            inner: CommonFutureRead {
                stream: Box::pin(stream),
                pos: 0,
                buf: Bytes::new(),
            },
        })
    }

    #[napi]
    pub fn get_image(&self, id: String) -> Image {
        Image {
            id,
            docker: self.0.to_owned(),
        }
    }
}

#[napi]
pub struct CreateImageOutput {
    inner: CommonFutureRead<bollard::models::CreateImageInfo>,
}

impl ToBytes for bollard::models::CreateImageInfo {
    fn with_eol() -> bool {
        true
    }

    fn to_bytes(self) -> std::io::Result<Bytes> {
        let bytes = serde_json::to_vec(&self).map(Bytes::from)?;
        Ok(bytes)
    }
}

impl_async_read!(CreateImageOutput);
