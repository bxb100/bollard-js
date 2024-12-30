mod attach;
mod create;
mod remove;
mod start;

use bollard::Docker;

#[napi]
#[derive(Clone)]
pub struct Container {
    docker: Docker,
    pub id: String,
}
