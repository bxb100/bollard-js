use crate::exec::Exec;
use crate::format_err;
use crate::types::Output;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::exec::StartExecOptions)]
#[napi(object)]
pub struct StartExecOptions {
    /// Detach from the command.
    #[napi(js_name = "Detach")]
    pub detach: bool,
    /// Allocate a pseudo-TTY.
    #[napi(js_name = "Tty")]
    pub tty: bool,
    /// The maximum size for a line of output. The default is 8 * 1024 (roughly 1024 characters).
    #[into(~.map(|n| n as usize))]
    #[napi(js_name = "OutputCapacity")]
    pub output_capacity: Option<u32>,
}

#[napi]
impl Exec {
    /// Starts a previously set up exec instance. If detach is true, this endpoint returns immediately after starting the command. Otherwise, it sets up an interactive session with the command.
    #[napi]
    pub async fn start(&self, option: Option<StartExecOptions>) -> Result<Option<Output>> {
        let res = self
            .docker
            .start_exec(&self.id, option.map(Into::into))
            .await
            .map_err(format_err)?;

        match res {
            bollard::exec::StartExecResults::Attached { output, input } => {
                Ok(Some(Output::new(output, input)))
            }
            bollard::exec::StartExecResults::Detached => Ok(None),
        }
    }
}
