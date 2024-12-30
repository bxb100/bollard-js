use crate::container::Container;
use crate::exec::Exec;
use crate::format_err;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::exec::CreateExecOptions::<String>)]
#[napi(object)]
pub struct CreateExecOptions {
    /// Attach to `stdin` of the exec command.
    #[napi(js_name = "AttachStdin")]
    pub attach_stdin: Option<bool>,
    /// Attach to stdout of the exec command.
    #[napi(js_name = "AttachStdout")]
    pub attach_stdout: Option<bool>,
    /// Attach to stderr of the exec command.
    #[napi(js_name = "AttachStderr")]
    pub attach_stderr: Option<bool>,
    /// Allocate a pseudo-TTY.
    #[napi(js_name = "Tty")]
    pub tty: Option<bool>,
    /// Override the key sequence for detaching a container. Format is a single character `[a-Z]`
    /// or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
    #[napi(js_name = "DetachKeys")]
    pub detach_keys: Option<String>,
    /// A list of environment variables in the form `["VAR=value", ...].`
    #[napi(js_name = "Env")]
    pub env: Option<Vec<String>>,
    /// Command to run, as a string or array of strings.
    #[napi(js_name = "Cmd")]
    pub cmd: Option<Vec<String>>,
    /// Runs the exec process with extended privileges.
    #[napi(js_name = "Privileged")]
    pub privileged: Option<bool>,
    /// The user, and optionally, group to run the exec process inside the container. Format is one
    /// of: `user`, `user:group`, `uid`, or `uid:gid`.
    #[napi(js_name = "User")]
    pub user: Option<String>,
    /// The working directory for the exec process inside the container.
    #[napi(js_name = "WorkingDir")]
    pub working_dir: Option<String>,
}

#[napi]
pub struct CreateExecResults {
    exec: Exec,
}

#[napi]
impl CreateExecResults {
    #[napi(getter)]
    pub fn exec(&self) -> Exec {
        self.exec.clone()
    }
}

#[napi]
impl Container {
    #[napi]
    pub async fn exec(&self, option: CreateExecOptions) -> Result<CreateExecResults> {
        let res = self
            .docker
            .create_exec(&self.id, option.into())
            .await
            .map_err(format_err)?;

        Ok(CreateExecResults {
            exec: Exec::new(self.docker.clone(), res.id),
        })
    }
}
