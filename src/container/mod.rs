mod attach;
mod changes;
mod commit;
mod create;
mod exec;
mod export;
mod inspect;
mod pause;
mod remove;
mod rename;
mod start;
mod stop;
mod top;
mod unpause;
mod update;

use bollard::Docker;

#[napi]
#[derive(Clone)]
pub struct Container {
    docker: Docker,
    pub id: String,
}
