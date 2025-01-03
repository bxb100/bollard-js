use crate::format_err;
use crate::image::Image;
use crate::stubs::ImageDeleteResponseItem;
use crate::types::DockerCredentials;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::image::RemoveImageOptions)]
#[napi(object)]
pub struct RemoveImageOptions {
    /// Remove the image even if it is being used by stopped containers or has other tags.
    pub force: bool,
    /// Do not delete untagged parent images.
    pub noprune: bool,
}

#[napi]
impl Image {
    #[napi]
    pub async fn remove(
        &self,
        option: Option<RemoveImageOptions>,
        credentials: Option<DockerCredentials>,
    ) -> Result<Vec<ImageDeleteResponseItem>> {
        let res = self
            .docker
            .remove_image(
                &self.id,
                option.map(Into::into),
                credentials.map(Into::into),
            )
            .await
            .map_err(format_err)?;

        Ok(res.into_iter().map(Into::into).collect())
    }
}
