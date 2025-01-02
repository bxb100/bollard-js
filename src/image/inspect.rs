use crate::stubs::ImageInspect;
use crate::{format_err, Docker};
use napi::bindgen_prelude::*;

#[napi]
impl Docker {
    #[napi]
    pub async fn inspect_image(&self, image_name: String) -> Result<ImageInspect> {
        let res = self
            .0
            .inspect_image(&image_name)
            .await
            .map_err(format_err)?;

        Ok(res.into())
    }
}
