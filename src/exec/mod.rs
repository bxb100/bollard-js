mod inspect;
mod resize;
mod start;

use bollard::Docker;

#[derive(Clone)]
#[napi]
pub struct Exec {
    docker: Docker,
    pub id: String,
}

impl Exec {
    pub fn new(docker: Docker, id: String) -> Self {
        Self { docker, id }
    }
}
