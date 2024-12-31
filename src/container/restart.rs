use crate::container::Container;
use crate::format_err;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::RestartContainerOptions)]
#[napi(object)]
pub struct RestartContainerOptions {
    /// Number of seconds to wait before killing the container.
    #[into(~ as isize)]
    pub t: i64,
}

#[napi]
impl Container {
    #[napi]
    pub async fn restart(&self, option: Option<RestartContainerOptions>) -> Result<()> {
        self.docker
            .restart_container(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)
    }
}
