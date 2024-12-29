#[napi(object)]
pub struct DockerOptions {
    pub socket_path: Option<String>,
    pub url: Option<String>,
    pub ssl_key: Option<String>,
    pub ssl_cert: Option<String>,
    pub ssl_ca: Option<String>,
}
