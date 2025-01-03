use crate::container::Container;
use crate::impl_async_read;
use crate::types::CommonFutureRead;
use bollard::container::LogOutput;
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::LogsOptions::<String>)]
#[napi(object)]
pub struct LogsOptions {
    /// Return the logs as a finite stream.
    pub follow: bool,
    /// Return logs from `stdout`.
    pub stdout: bool,
    /// Return logs from `stderr`.
    pub stderr: bool,
    /// Only return logs since this time, as a UNIX timestamp.
    pub since: i64,
    /// Only return logs before this time, as a UNIX timestamp.
    // workaround for https://github.com/containers/podman/issues/10859
    pub until: i64,
    /// Add timestamps to every log line.
    pub timestamps: bool,
    /// Only return this number of log lines from the end of the logs. Specify as an integer or all
    /// to output `all` log lines.
    pub tail: String,
}

#[napi]
impl Container {
    #[napi]
    pub fn logs(&self, option: Option<LogsOptions>) -> Result<LogsResponse> {
        let stream = self.docker.logs(&self.id, option.map(Into::into));

        Ok(LogsResponse {
            inner: CommonFutureRead::new(stream),
        })
    }
}

#[napi]
pub struct LogsResponse {
    inner: CommonFutureRead<LogOutput>,
}

impl_async_read!(LogsResponse);
