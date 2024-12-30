mod attach;
mod create;
mod inspect;
mod remove;
mod rename;
mod start;
mod top;
mod update;

use bollard::Docker;

#[napi]
#[derive(Clone)]
pub struct Container {
    docker: Docker,
    pub id: String,
}
