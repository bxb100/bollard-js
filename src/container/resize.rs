use crate::container::Container;
use crate::format_err;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::ResizeContainerTtyOptions)]
#[napi(object)]
pub struct ResizeContainerTtyOptions {
    /// Width of the TTY session in characters
    #[napi(js_name = "w")]
    pub width: u16,
    /// Height of the TTY session in characters
    #[napi(js_name = "h")]
    pub height: u16,
}

#[napi]
impl Container {
    #[napi]
    pub async fn resize(&self, option: ResizeContainerTtyOptions) -> Result<()> {
        self.docker
            .resize_container_tty(&self.id, option.into())
            .await
            .map_err(format_err)
    }
}
