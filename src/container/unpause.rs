use crate::container::Container;
use crate::format_err;
use napi::bindgen_prelude::*;

#[napi]
impl Container {
    #[napi]
    pub async fn unpause(&self) -> Result<()> {
        self.docker
            .unpause_container(&self.id)
            .await
            .map_err(format_err)
    }
}
