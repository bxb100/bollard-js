use crate::container::Container;
use crate::format_err;
use crate::types::FutureRead;
use bytes::Bytes;
use futures::AsyncReadExt;
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
            inner: FutureRead {
                stream: Box::pin(stream),
                pos: 0,
                buf: Bytes::new(),
            },
        })
    }
}

#[napi]
pub struct LogsResponse {
    inner: FutureRead,
}

#[napi]
impl LogsResponse {
    #[napi]
    pub async unsafe fn read(&mut self, mut buf: Buffer) -> Result<usize> {
        let buf = buf.as_mut();
        let n = self.inner.read(buf).await.map_err(format_err)?;
        Ok(n)
    }
}
