use bollard::Docker;

mod create;
mod get;
mod history;
mod inspect;
mod list;
mod push;
mod remove;
mod tag;

#[napi]
pub struct Image {
    docker: Docker,
    pub id: String,
}
