use crate::container::Container;
use crate::format_err;
use crate::stubs::ContainerTopResponse;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::TopOptions::<String>)]
#[napi(object)]
pub struct TopOptions {
    /// The arguments to pass to `ps`. For example, `aux`
    pub ps_args: String,
}

#[napi]
impl Container {
    #[napi]
    pub async fn top(&self, option: Option<TopOptions>) -> Result<ContainerTopResponse> {
        let res = self
            .docker
            .top_processes(&self.id, option.map(|o| o.into()))
            .await
            .map_err(format_err)?;
        Ok(res.into())
    }
}
