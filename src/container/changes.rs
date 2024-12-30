use crate::container::Container;
use crate::converts::convert_vec_to_vec;
use crate::format_err;
use crate::stubs::FilesystemChange;
use napi::bindgen_prelude::*;

#[napi]
impl Container {
    #[napi]
    pub async fn changes(&self) -> Result<Option<Vec<FilesystemChange>>> {
        let res = self
            .docker
            .container_changes(&self.id)
            .await
            .map_err(format_err)?;

        Ok(convert_vec_to_vec(res))
    }
}
