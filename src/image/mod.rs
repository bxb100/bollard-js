use bollard::Docker;

mod create;
mod history;
mod inspect;
mod push;
mod tag;

#[napi]
pub struct Image {
    docker: Docker,
    pub id: String,
}
