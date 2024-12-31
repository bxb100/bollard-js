use crate::container::create::CrateContainerConfig;
use crate::container::Container;
use crate::format_err;
use crate::stubs::Commit;
use napi::bindgen_prelude::*;
use o2o::o2o;
use o2o::traits::IntoExisting;

#[derive(o2o)]
#[owned_into_existing(bollard::image::CommitContainerOptions::<String>)]
#[napi(object)]
pub struct CommitContainerOptions {
    /// Repository name for the created image.
    pub repo: String,
    /// Tag name for the create image.
    pub tag: String,
    /// Commit message.
    pub comment: String,
    /// Author of the image.
    pub author: String,
    /// Whether to pause the container before committing.
    pub pause: bool,
    /// `Dockerfile` instructions to apply while committing
    pub changes: Option<String>,
}

#[napi]
impl Container {
    #[napi]
    pub async fn commit(
        &self,
        option: CommitContainerOptions,
        config: CrateContainerConfig,
    ) -> Result<Commit> {
        let mut opt = bollard::image::CommitContainerOptions::<String> {
            container: self.id.clone(),
            ..Default::default()
        };
        option.into_existing(&mut opt);

        let res = self
            .docker
            .commit_container(opt, config.into())
            .await
            .map_err(format_err)?;

        Ok(res.into())
    }
}
