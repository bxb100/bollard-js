use crate::container::Container;
use crate::format_err;
use crate::stubs::{
    DeviceMapping, DeviceRequest, ResourcesBlkioWeightDevice, ResourcesUlimits, RestartPolicy,
    ThrottleDevice,
};
use napi::bindgen_prelude::*;
use o2o::o2o;

#[derive(o2o)]
#[owned_into(bollard::container::UpdateContainerOptions::<String>)]
#[napi(object)]
pub struct UpdateContainerOptions {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[napi(js_name = "CpuShares")]
    #[map(~.map(|v| v as isize))]
    pub cpu_shares: Option<i64>,

    /// Memory limit in bytes.
    #[napi(js_name = "Memory")]
    pub memory: Option<i64>,

    /// Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist.
    #[napi(js_name = "CgroupParent")]
    pub cgroup_parent: Option<String>,

    /// Block IO weight (relative weight).
    #[napi(js_name = "BlkioWeight")]
    pub blkio_weight: Option<u16>,

    /// Block IO weight (relative device weight) in the form `[{'Path': 'device_path', 'Weight': weight}]`.
    #[napi(js_name = "BlkioWeightDevice")]
    #[map(crate::converts::convert_vec_to_vec(~))]
    pub blkio_weight_device: Option<Vec<ResourcesBlkioWeightDevice>>,

    /// Limit read rate (bytes per second) from a device, in the form `[{'Path': 'device_path', 'Rate': rate}]`.
    #[napi(js_name = "BlkioDeviceReadBps")]
    #[map(crate::converts::convert_vec_to_vec(~))]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (bytes per second) to a device, in the form `[{'Path': 'device_path', 'Rate': rate}]`.
    #[napi(js_name = "BlkioDeviceWriteBps")]
    #[map(crate::converts::convert_vec_to_vec(~))]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,

    /// Limit read rate (IO per second) from a device, in the form `[{'Path': 'device_path', 'Rate': rate}]`.
    #[napi(js_name = "BlkioDeviceReadIOps")]
    #[map(crate::converts::convert_vec_to_vec(~))]
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (IO per second) to a device, in the form `[{'Path': 'device_path', 'Rate': rate}]`.
    #[napi(js_name = "BlkioDeviceWriteIOps")]
    #[map(crate::converts::convert_vec_to_vec(~))]
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,

    /// The length of a CPU period in microseconds.
    #[napi(js_name = "CpuPeriod")]
    pub cpu_period: Option<i64>,

    /// Microseconds of CPU time that the container can get in a CPU period.
    #[napi(js_name = "CpuQuota")]
    pub cpu_quota: Option<i64>,

    /// The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[napi(js_name = "CpuRealtimePeriod")]
    pub cpu_realtime_period: Option<i64>,

    /// The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[napi(js_name = "CpuRealtimeRuntime")]
    pub cpu_realtime_runtime: Option<i64>,

    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`)
    #[napi(js_name = "CpusetCpus")]
    pub cpuset_cpus: Option<String>,

    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[napi(js_name = "CpusetMems")]
    pub cpuset_mems: Option<String>,

    /// A list of devices to add to the container.
    #[napi(js_name = "Devices")]
    #[map(crate::converts::convert_vec_to_vec(~))]
    pub devices: Option<Vec<DeviceMapping>>,

    /// a list of cgroup rules to apply to the container
    #[napi(js_name = "DeviceCgroupRules")]
    pub device_cgroup_rules: Option<Vec<String>>,

    /// a list of requests for devices to be sent to device drivers
    #[napi(js_name = "DeviceRequests")]
    #[map(crate::converts::convert_vec_to_vec(~))]
    pub device_requests: Option<Vec<DeviceRequest>>,

    /// Hard limit for kernel TCP buffer memory (in bytes).
    #[napi(js_name = "KernelMemoryTCP")]
    pub kernel_memory_tcp: Option<i64>,

    /// Memory soft limit in bytes.
    #[napi(js_name = "MemoryReservation")]
    pub memory_reservation: Option<i64>,

    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[napi(js_name = "MemorySwap")]
    pub memory_swap: Option<i64>,

    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[napi(js_name = "MemorySwappiness")]
    pub memory_swappiness: Option<i64>,

    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    #[napi(js_name = "NanoCpus")]
    pub nano_cpus: Option<i64>,

    /// Disable OOM Killer for the container.
    #[napi(js_name = "OomKillDisable")]
    pub oom_kill_disable: Option<bool>,

    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[napi(js_name = "Init")]
    pub init: Option<bool>,

    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change.
    #[napi(js_name = "PidsLimit")]
    pub pids_limit: Option<i64>,

    /// A list of resource limits to set in the container. For example: `{'Name': 'nofile', 'Soft': 1024, 'Hard': 2048}`
    #[napi(js_name = "Ulimits")]
    #[map(crate::converts::convert_vec_to_vec(~))]
    pub ulimits: Option<Vec<ResourcesUlimits>>,

    /// The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[napi(js_name = "CpuCount")]
    pub cpu_count: Option<i64>,

    /// The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[napi(js_name = "CpuPercent")]
    pub cpu_percent: Option<i64>,

    /// Maximum IOps for the container system drive (Windows only)
    #[napi(js_name = "IOMaximumIOps")]
    pub io_maximum_iops: Option<i64>,

    /// Maximum IO in bytes per second for the container system drive (Windows only)
    #[napi(js_name = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<i64>,

    /// The behavior to apply when the container exits. The default is not to restart.
    ///
    /// An ever increasing delay (double the previous delay, starting at 100ms) is added before
    /// each restart to prevent flooding the server.
    #[map(~.map(Into::into))]
    pub restart_policy: Option<RestartPolicy>,
}

#[napi]
impl Container {
    #[napi]
    pub async fn update(&self, option: UpdateContainerOptions) -> Result<()> {
        self.docker
            .update_container(&self.id, option.into())
            .await
            .map_err(format_err)
    }
}
