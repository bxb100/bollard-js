#[macro_use]
extern crate napi_derive;
mod container;
mod converts;
mod exec;
mod stubs;
mod types;

use crate::types::*;
use bollard::ClientVersion;
use napi::bindgen_prelude::*;
use std::path::Path;

const DEFAULT_TIMEOUT: u64 = 120;
const API_DEFAULT_VERSION: &ClientVersion = bollard::API_DEFAULT_VERSION;

#[napi]
pub struct Docker(bollard::Docker);

//noinspection RsCompileErrorMacro
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
                unreachable!()
            }
        } else {
            bollard::Docker::connect_with_defaults().map_err(format_err)?
        };

        Ok(Self(docker))
    }

    #[napi(js_name = "_version")]
    pub async fn version(&self) -> Result<Buffer> {
        self.0
            .version()
            .await
            .map(|v| serde_json::to_string(&v))
            .map_err(format_err)?
            .map(Into::into)
            .map_err(format_err)
    }
}

fn format_err(error: impl std::error::Error) -> Error {
    Error::from_reason(format!("{}", error))
}
