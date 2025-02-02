use crate::format_err;
use crate::image::Image;
use crate::stubs::ImageInspect;
use napi::bindgen_prelude::*;

#[napi]
impl Image {
    #[napi]
    pub async fn inspect(&self) -> Result<ImageInspect> {
        let res = self
            .docker
            .inspect_image(&self.id)
            .await
            .map_err(format_err)?;

        Ok(res.into())
    }
}
