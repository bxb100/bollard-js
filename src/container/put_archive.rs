use crate::container::Container;
use crate::format_err;
use bytes::Bytes;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::UploadToContainerOptions::<String>)]
#[napi(object)]
pub struct UploadToContainerOptions {
    /// Path to a directory in the container to extract the archive’s contents into.
    pub path: String,
    /// If “1”, “true”, or “True” then it will be an error if unpacking the given content would
    /// cause an existing directory to be replaced with a non-directory and vice versa.
    pub no_overwrite_dir_non_dir: String,
}

#[napi]
impl Container {
    #[napi]
    pub async fn put_archive(
        &self,
        option: Option<UploadToContainerOptions>,
        archive: Buffer,
    ) -> Result<()> {
        // or using std fs direct read, which better?
        let vec: Vec<u8> = archive.into();

        self.docker
            .upload_to_container(&self.id, option.map(Into::into), Bytes::from(vec))
            .await
            .map_err(format_err)
    }
}
