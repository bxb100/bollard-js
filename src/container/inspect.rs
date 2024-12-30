use crate::container::Container;
use crate::format_err;
use crate::stubs::ContainerInspectResponse;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::InspectContainerOptions)]
#[napi(object)]
pub struct InspectContainerOptions {
    /// Return the size of container as fields `SizeRw` and `SizeRootFs`
    pub size: bool,
}

#[napi]
impl Container {
    #[napi]
    pub async fn inspect(
        &self,
        option: Option<InspectContainerOptions>,
    ) -> Result<ContainerInspectResponse> {
        let response = self
            .docker
            .inspect_container(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)?;

        Ok(response.into())
    }
}
