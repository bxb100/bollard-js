use crate::exec::Exec;
use crate::format_err;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::exec::ResizeExecOptions)]
#[napi(object)]
pub struct ResizeExecOptions {
    /// Height of the TTY session in characters
    #[napi(js_name = "h")]
    pub height: u16,
    /// Width of the TTY session in characters
    #[napi(js_name = "w")]
    pub width: u16,
}

#[napi]
impl Exec {
    #[napi]
    pub async fn resize(&self, option: ResizeExecOptions) -> Result<()> {
        self.docker
            .resize_exec(&self.id, option.into())
            .await
            .map_err(format_err)
    }
}
