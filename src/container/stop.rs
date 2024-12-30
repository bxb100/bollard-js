use crate::container::Container;
use crate::format_err;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::StopContainerOptions)]
#[napi(object)]
pub struct StopContainerOptions {
    /// Number of seconds to wait before killing the container
    pub t: i64,
}

#[napi]
impl Container {
    #[napi]
    pub async fn stop(&self, option: Option<StopContainerOptions>) -> Result<()> {
        self.docker
            .stop_container(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)
    }
}
