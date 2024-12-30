use crate::container::Container;
use futures::StreamExt;
use napi::bindgen_prelude::*;
use std::fs;
use std::io::Write;

#[napi]
impl Container {
    #[napi]
    pub async fn export(&self, path: String) -> Result<()> {
        let mut stream = self.docker.export_container(&self.id);

        let mut f = fs::File::create(path)?;

        while let Some(Ok(bytes)) = stream.next().await {
            f.write_all(&bytes)?
        }

        Ok(())
    }
}
