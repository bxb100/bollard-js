use crate::container::Container;
use crate::stubs::{EndpointSettings, HealthConfig, HostConfig};
use crate::{format_err, Docker};
use napi::bindgen_prelude::*;
use o2o::o2o;
use std::collections::HashMap;

#[derive(o2o)]
#[owned_into(bollard::container::CreateContainerOptions::<String>)]
#[napi(object)]
pub struct CreateContainerOptions {
    pub name: String,
    pub platform: Option<String>,
}

#[derive(o2o)]
#[owned_into(bollard::container::Config::<String>)]
#[napi(object)]
pub struct Config {
    #[napi(js_name = "Hostname")]
    pub hostname: Option<String>,

    /// The domain name to use for the container.
    #[napi(js_name = "Domainname")]
    pub domainname: Option<String>,

    /// The user that commands are run as inside the container.
    #[napi(js_name = "User")]
    pub user: Option<String>,

    /// Whether to attach to `stdin`.
    #[napi(js_name = "AttachStdin")]
    pub attach_stdin: Option<bool>,

    /// Whether to attach to `stdout`.
    #[napi(js_name = "AttachStdout")]
    pub attach_stdout: Option<bool>,

    /// Whether to attach to `stderr`.
    #[napi(js_name = "AttachStderr")]
    pub attach_stderr: Option<bool>,

    /// An object mapping ports to an empty object in the form:  `{\'<port>/<tcp|udp|sctp>\': {}}`
    #[napi(js_name = "ExposedPorts")]
    #[into(crate::converts::convert_vec_to_map(~))]
    pub exposed_ports: Option<Vec<String>>,

    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[napi(js_name = "Tty")]
    pub tty: Option<bool>,

    /// Open `stdin`
    #[napi(js_name = "OpenStdin")]
    pub open_stdin: Option<bool>,

    /// Close `stdin` after one attached client disconnects
    #[napi(js_name = "StdinOnce")]
    pub stdin_once: Option<bool>,

    /// A list of environment variables to set inside the container in the form `[\'VAR=value\', ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[napi(js_name = "Env")]
    pub env: Option<Vec<String>>,

    /// Command to run specified as a string or an array of strings.
    #[napi(js_name = "Cmd")]
    pub cmd: Option<Vec<String>>,

    /// A TEST to perform TO Check that the container is healthy.
    #[napi(js_name = "Healthcheck")]
    #[map(~.map(Into::into))]
    pub healthcheck: Option<HealthConfig>,

    /// Command is already escaped (Windows only)
    #[napi(js_name = "ArgsEscaped")]
    pub args_escaped: Option<bool>,

    /// The name of the image to use when creating the container
    #[napi(js_name = "Image")]
    pub image: Option<String>,

    /// An object mapping mount point paths inside the container to empty objects.
    #[napi(js_name = "Volumes")]
    #[into(crate::converts::convert_vec_to_map(~))]
    pub volumes: Option<Vec<String>>,

    /// The working directory for commands to run in.
    #[napi(js_name = "WorkingDir")]
    pub working_dir: Option<String>,

    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\'\']`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[napi(js_name = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,

    /// Disable networking for the container.
    #[napi(js_name = "NetworkDisabled")]
    pub network_disabled: Option<bool>,

    /// MAC address of the container.
    #[napi(js_name = "MacAddress")]
    pub mac_address: Option<String>,

    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[napi(js_name = "OnBuild")]
    pub on_build: Option<Vec<String>>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// Signal to stop a container as a string or unsigned integer.
    #[napi(js_name = "StopSignal")]
    pub stop_signal: Option<String>,

    /// Timeout to stop a container in seconds.
    #[napi(js_name = "StopTimeout")]
    pub stop_timeout: Option<i64>,

    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[napi(js_name = "Shell")]
    pub shell: Option<Vec<String>>,

    /// Container configuration that depends on the host we are running on.
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[napi(js_name = "HostConfig")]
    #[map(~.map(Into::into))]
    pub host_config: Option<HostConfig>,

    /// This container's networking configuration.
    #[napi(js_name = "NetworkingConfig")]
    #[map(~.map(Into::into))]
    pub networking_config: Option<NetworkingConfig>,
}

#[derive(o2o)]
#[owned_into(bollard::container::NetworkingConfig::<String>)]
#[napi(object)]
pub struct NetworkingConfig {
    #[map(~.into_iter().map(|(k, v)| (k, v.into())).collect())]
    pub endpoints_config: HashMap<String, EndpointSettings>,
}

#[napi]
pub struct CreateContainerResponse {
    container: Container,
    warnings: Vec<String>,
}

#[napi]
impl CreateContainerResponse {
    #[napi(getter)]
    pub fn container(&self) -> Container {
        self.container.clone()
    }

    #[napi(getter)]
    pub fn warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }
}

#[napi]
impl Docker {
    #[napi]
    pub async fn create_container(
        &self,
        options: Option<CreateContainerOptions>,
        config: Config,
    ) -> Result<CreateContainerResponse> {
        let res = self
            .0
            .create_container(options.map(|o| o.into()), config.into())
            .await
            .map_err(format_err)?;

        Ok(CreateContainerResponse {
            container: Container {
                id: res.id,
                docker: self.0.clone(),
            },
            warnings: res.warnings,
        })
    }

    #[napi]
    pub fn get_container(&self, id: String) -> Container {
        Container {
            docker: self.0.clone(),
            id,
        }
    }
}
