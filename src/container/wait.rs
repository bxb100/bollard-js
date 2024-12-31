use crate::container::Container;
use crate::stubs::ContainerWaitResponse;
use futures::StreamExt;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::WaitContainerOptions::<String>)]
#[napi(object)]
pub struct WaitContainerOptions {
    /// Wait until a container state reaches the given condition, either 'not-running' (default),
    /// 'next-exit', or 'removed'.
    pub condition: String,
}

#[napi]
impl Container {
    #[napi]
    pub async fn wait(
        &self,
        option: Option<WaitContainerOptions>,
    ) -> Option<ContainerWaitResponse> {
        let mut stream = self.docker.wait_container(&self.id, option.map(Into::into));

        match stream.next().await {
            None => None,
            Some(item) => item.map(Into::into).ok(),
        }
    }
}
