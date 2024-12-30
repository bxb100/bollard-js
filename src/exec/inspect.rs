use crate::exec::Exec;
use crate::format_err;
use crate::stubs::ExecInspectResponse;
use napi::bindgen_prelude::*;

#[napi]
impl Exec {
    #[napi]
    pub async fn inspect(&self) -> Result<ExecInspectResponse> {
        let response = self
            .docker
            .inspect_exec(&self.id)
            .await
            .map_err(format_err)?;

        Ok(response.into())
    }
}
