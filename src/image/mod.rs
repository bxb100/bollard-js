use bollard::Docker;

mod create;
mod history;
mod inspect;

#[napi]
pub struct Image {
    docker: Docker,
    pub id: String,
}
