use crate::container::Container;
use crate::format_err;
use bollard::container::StartContainerOptions;
use napi::bindgen_prelude::*;

#[napi]
impl Container {
    #[napi]
    pub async fn start(&self) -> Result<()> {
        self.docker
            .start_container(&self.id, None::<StartContainerOptions<String>>)
            .await
            .map_err(format_err)
    }
}
