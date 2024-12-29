use crate::container::Container;
use crate::format_err;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::RemoveContainerOptions)]
#[napi(object)]
pub struct RemoveContainerOptions {
    /// Remove the volumes associated with the container.
    pub v: bool,
    /// If the container is running, kill it before removing it.
    pub force: bool,
    /// Remove the specified link associated with the container.
    pub link: bool,
}

#[napi]
impl Container {
    #[napi]
    pub async fn remove(&self, option: Option<RemoveContainerOptions>) -> Result<()> {
        self.docker
            .remove_container(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)?;

        Ok(())
    }
}
