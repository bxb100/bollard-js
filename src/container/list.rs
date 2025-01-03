use crate::stubs::ContainerSummary;
use crate::{format_err, Docker};
use napi::bindgen_prelude::*;
use o2o::o2o;
use std::collections::HashMap;

#[derive(o2o)]
#[owned_into(bollard::container::ListContainersOptions::<String>)]
#[napi(object)]
pub struct ListContainersOptions {
    /// Return all containers. By default, only running containers are shown
    pub all: bool,
    /// Return this number of most recently created containers, including non-running ones
    #[into(~.map(|l| l as isize))]
    pub limit: Option<i64>,
    /// Return the size of container as fields `SizeRw` and `SizeRootFs`
    pub size: bool,
    /// Filters to process on the container list, encoded as JSON. Available filters:
    ///  - `ancestor`=`(<image-name>[:<tag>]`, `<image id>`, or `<image@digest>`)
    ///  - `before`=(`<container id>` or `<container name>`)
    ///  - `expose`=(`<port>[/<proto>]`|`<startport-endport>`/`[<proto>]`)
    ///  - `exited`=`<int>` containers with exit code of `<int>`
    ///  - `health`=(`starting`|`healthy`|`unhealthy`|`none`)
    ///  - `id`=`<ID>` a container's ID
    ///  - `isolation`=(`default`|`process`|`hyperv`) (Windows daemon only)
    ///  - `is-task`=`(true`|`false`)
    ///  - `label`=`key` or `label`=`"key=value"` of a container label
    ///  - `name`=`<name>` a container's name
    ///  - `network`=(`<network id>` or `<network name>`)
    ///  - `publish`=(`<port>[/<proto>]`|`<startport-endport>`/`[<proto>]`)
    ///  - `since`=(`<container id>` or `<container name>`)
    ///  - `status`=(`created`|`restarting`|`running`|`removing`|`paused`|`exited`|`dead`)
    ///  - `volume`=(`<volume name>` or `<mount point destination>`)
    pub filters: HashMap<String, Vec<String>>,
}

#[napi]
impl Docker {
    #[napi]
    pub async fn list_containers(
        &self,
        option: Option<ListContainersOptions>,
    ) -> Result<Vec<ContainerSummary>> {
        let res = self
            .0
            .list_containers(option.map(Into::into))
            .await
            .map_err(format_err)?;

        Ok(res.into_iter().map(Into::into).collect())
    }
}
