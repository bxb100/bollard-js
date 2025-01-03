use crate::format_err;
use crate::image::Image;
use crate::stubs::HistoryResponseItem;
use napi::bindgen_prelude::*;

#[napi]
impl Image {
    #[napi]
    pub async fn history(&self) -> Result<Vec<HistoryResponseItem>> {
        let res = self
            .docker
            .image_history(&self.id)
            .await
            .map_err(format_err)?;

        Ok(res.into_iter().map(Into::into).collect())
    }
}
