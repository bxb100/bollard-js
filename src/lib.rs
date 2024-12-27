#[macro_use]
extern crate napi_derive;

use bollard::API_DEFAULT_VERSION;
use napi::bindgen_prelude::*;
use std::path::Path;

#[napi]
pub struct Docker(bollard::Docker);

#[napi(object)]
pub struct DockerOptions {
    pub socket_path: Option<String>,
    pub url: Option<String>,
    pub ssl_key: Option<String>,
    pub ssl_cert: Option<String>,
    pub ssl_ca: Option<String>,
}

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
}

fn format_err(error: impl std::error::Error) -> Error {
    Error::from_reason(format!("{}", error))
}
