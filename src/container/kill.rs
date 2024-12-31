use crate::container::Container;
use crate::format_err;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::KillContainerOptions::<String>)]
#[napi(object)]
pub struct KillContainerOptions {
    /// Signal to send to the container as an integer or string (e.g. `SIGINT`)
    pub signal: String,
}

#[napi]
impl Container {
    #[napi]
    pub async fn kill(&self, option: Option<KillContainerOptions>) -> Result<()> {
        self.docker
            .kill_container(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)
    }
}
