use crate::container::Container;
use crate::format_err;
use crate::types::Output;
use bollard::container::AttachContainerResults;
use napi::bindgen_prelude::*;
use o2o::o2o;

/// ## Notice
///
/// I can't figure out the detach-key abilities, if I set `ctrl-c`
/// which means
/// ```rust
/// self.sink.write_i8(0x03).await?;
/// ```
///
/// > 0x3 means ascii `^C`
///
/// will work?
#[derive(o2o)]
#[owned_into(bollard::container::AttachContainerOptions::<String>)]
#[ghosts(detach_keys: {None})]
#[napi(object)]
pub struct AttachOptions {
    pub stdin: Option<bool>,
    pub stdout: Option<bool>,
    pub stderr: Option<bool>,
    pub stream: Option<bool>,
    pub logs: Option<bool>,
}

#[napi]
impl Container {
    #[napi]
    pub async fn attach(&self, option: Option<AttachOptions>) -> Result<Output> {
        let AttachContainerResults { output, input } = self
            .docker
            .attach_container(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)?;

        Ok(Output::new(output, input))
    }
}
