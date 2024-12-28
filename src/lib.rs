#[macro_use]
extern crate napi_derive;
mod types;

use crate::types::*;
use bollard::container::AttachContainerResults;
use bollard::API_DEFAULT_VERSION;
use napi::bindgen_prelude::*;
use std::path::Path;

#[napi]
pub struct Docker(bollard::Docker);

const DEFAULT_TIMEOUT: u64 = 120;

#[napi]
impl Docker {
    #[napi(constructor)]
    pub fn new(options: Option<DockerOptions>) -> Result<Self> {
        let docker = if let Some(opt) = options {
            if let Some(socket_path) = opt.socket_path {
                bollard::Docker::connect_with_socket(
                    &socket_path,
                    DEFAULT_TIMEOUT,
                    API_DEFAULT_VERSION,
                )
                .map_err(format_err)?
            } else if let Some(url) = opt.url {
                if let DockerOptions {
                    ssl_key: Some(ssk_key),
                    ssl_cert: Some(ssl_cert),
                    ssl_ca: Some(ssl_ca),
                    ..
                } = opt
                {
                    bollard::Docker::connect_with_ssl(
                        &url,
                        Path::new(&ssk_key),
                        Path::new(&ssl_cert),
                        Path::new(&ssl_ca),
                        DEFAULT_TIMEOUT,
                        API_DEFAULT_VERSION,
                    )
                    .map_err(format_err)?
                } else {
                    bollard::Docker::connect_with_http(&url, DEFAULT_TIMEOUT, API_DEFAULT_VERSION)
                        .map_err(format_err)?
                }
            } else {
                unimplemented!("")
            }
        } else {
            bollard::Docker::connect_with_defaults().map_err(format_err)?
        };

        Ok(Self(docker))
    }

    #[napi]
    pub async fn version(&self) -> Result<Buffer> {
        let v = self.0.version().await.map_err(format_err)?;
        serde_json::to_string(&v)
            .map(|s| s.into())
            .map_err(format_err)
    }

    #[napi]
    pub async fn attach(&self, id: String, option: Option<AttachOptions>) -> Result<AttachOutput> {
        let option = option.map(|opt| bollard::container::AttachContainerOptions::<String> {
            stdin: opt.stdin,
            stderr: opt.stderr,
            stdout: opt.stdout,
            stream: opt.stream,
            logs: opt.logs,
            detach_keys: None,
        });

        let AttachContainerResults { output, input } = self
            .0
            .attach_container(&id, option)
            .await
            .map_err(format_err)?;

        Ok(AttachOutput::new(output, input))
    }
}

fn format_err(error: impl std::error::Error) -> Error {
    Error::from_reason(format!("{}", error))
}
