use crate::stubs::ImageSummary;
use crate::{format_err, Docker};
use napi::bindgen_prelude::*;
use o2o::o2o;
use std::collections::HashMap;

#[derive(o2o)]
#[owned_into(bollard::image::ListImagesOptions::<String>)]
#[napi(object)]
pub struct ListImagesOptions {
    /// Show all images. Only images from a final layer (no children) are shown by default.
    pub all: bool,
    /// A JSON encoded value of the filters to process on the images list. Available filters:
    ///  - `before`=(`<image-name>[:<tag>]`, `<image id>` or `<image@digest>`)
    ///  - `dangling`=`true`
    ///  - `label`=`key` or `label`=`"key=value"` of an image label
    ///  - `reference`=(`<image-name>[:<tag>]`)
    ///  - `since`=(`<image-name>[:<tag>]`, `<image id>` or `<image@digest>`)
    pub filters: HashMap<String, Vec<String>>,
    /// Show digest information as a RepoDigests field on each image.
    pub digests: bool,
}

#[napi]
impl Docker {
    #[napi]
    pub async fn list_images(
        &self,
        option: Option<ListImagesOptions>,
    ) -> Result<Vec<ImageSummary>> {
        let res = self
            .0
            .list_images(option.map(Into::into))
            .await
            .map_err(format_err)?;

        Ok(res.into_iter().map(Into::into).collect())
    }
}
