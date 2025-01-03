use crate::container::Container;
use crate::impl_async_read;
use crate::types::CommonFutureRead;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::StatsOptions)]
#[napi(object)]
pub struct StatsOptions {
    /// Stream the output. If false, the stats will be output once and then it will disconnect.
    pub stream: bool,
    /// Only get a single stat instead of waiting for 2 cycles. Must be used with `stream = false`.
    pub one_shot: bool,
}

#[napi]
impl Container {
    #[napi]
    pub fn stats(&self, option: Option<StatsOptions>) -> Result<StatsStream> {
        let output = self.docker.stats(&self.id, option.map(Into::into));
        Ok(StatsStream {
            inner: CommonFutureRead::new(output),
        })
    }
}

#[napi]
pub struct StatsStream {
    inner: CommonFutureRead<bollard::container::Stats>,
}

impl_async_read!(StatsStream);
