use crate::container::Container;
use crate::format_err;
use bollard::container::RenameContainerOptions;
use napi::bindgen_prelude::*;

#[napi]
impl Container {
    #[napi]
    pub async fn rename(&self, new_name: String) -> Result<()> {
        self.docker
            .rename_container(&self.id, RenameContainerOptions { name: new_name })
            .await
            .map_err(format_err)
    }
}
