use crate::format_err;
use crate::image::Image;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::image::TagImageOptions::<String>)]
#[napi(object)]
pub struct TagImageOptions {
    /// The repository to tag in. For example, `someuser/someimage`.
    pub repo: String,
    /// The name of the new tag.
    pub tag: String,
}

#[napi]
impl Image {
    #[napi]
    pub async fn tag(&self, option: Option<TagImageOptions>) -> Result<()> {
        self.docker
            .tag_image(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)
    }
}
