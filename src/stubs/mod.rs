#![allow(clippy::all)]

use o2o::o2o;
use std::collections::HashMap;

/// Address represents an IPv4 or IPv6 IP address.
#[napi(object)]
pub struct Address {
    /// IP address.
    #[napi(js_name = "Addr")]
    pub addr: Option<String>,

    /// Mask length of the IP address.
    #[napi(js_name = "PrefixLen")]
    pub prefix_len: Option<i64>,
}

#[napi(object)]
pub struct AuthConfig {
    #[napi(js_name = "username")]
    pub username: Option<String>,

    #[napi(js_name = "password")]
    pub password: Option<String>,

    #[napi(js_name = "email")]
    pub email: Option<String>,

    #[napi(js_name = "serveraddress")]
    pub serveraddress: Option<String>,
}

/// Volume configuration
#[napi(object)]
pub struct Body {
    #[napi(js_name = "Spec")]
    pub spec: Option<ClusterVolumeSpec>,
}

/// BuildCache contains information about a build cache record.
#[napi(object)]
pub struct BuildCache {
    /// Unique ID of the build cache record.
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    /// ID of the parent build cache record.  > **Deprecated**: This field is deprecated, and omitted if empty.
    #[napi(js_name = "Parent")]
    pub parent: Option<String>,

    /// List of parent build cache record IDs.
    #[napi(js_name = "Parents")]
    pub parents: Option<Vec<String>>,

    /// Cache record type.
    #[napi(js_name = "Type")]
    pub typ: Option<BuildCacheTypeEnum>,

    /// Description of the build-step that produced the build cache.
    #[napi(js_name = "Description")]
    pub description: Option<String>,

    /// Indicates if the build cache is in use.
    #[napi(js_name = "InUse")]
    pub in_use: Option<bool>,

    /// Indicates if the build cache is shared.
    #[napi(js_name = "Shared")]
    pub shared: Option<bool>,

    /// Amount of disk space used by the build cache (in bytes).
    #[napi(js_name = "Size")]
    pub size: Option<i64>,

    /// Date and time at which the build cache was created in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    /// Date and time at which the build cache was last used in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "LastUsedAt")]
    pub last_used_at: Option<String>,

    #[napi(js_name = "UsageCount")]
    pub usage_count: Option<i64>,
}

#[allow(non_camel_case_types)]
#[napi(string_enum)]
pub enum BuildCacheTypeEnum {
    EMPTY,
    INTERNAL,
    FRONTEND,
    SOURCE_LOCAL,
    SOURCE_GIT_CHECKOUT,
    EXEC_CACHEMOUNT,
    REGULAR,
}

#[napi(object)]
pub struct BuildInfo {
    #[napi(js_name = "id")]
    pub id: Option<String>,

    #[napi(js_name = "stream")]
    pub stream: Option<String>,

    #[napi(js_name = "error")]
    pub error: Option<String>,

    #[napi(js_name = "errorDetail")]
    pub error_detail: Option<ErrorDetail>,

    #[napi(js_name = "status")]
    pub status: Option<String>,

    #[napi(js_name = "progress")]
    pub progress: Option<String>,

    #[napi(js_name = "progressDetail")]
    pub progress_detail: Option<ProgressDetail>,

    #[napi(js_name = "aux")]
    pub aux: Option<ImageId>,
}

#[napi(object)]
pub struct BuildPruneResponse {
    #[napi(js_name = "CachesDeleted")]
    pub caches_deleted: Option<Vec<String>>,

    /// Disk space reclaimed in bytes
    #[napi(js_name = "SpaceReclaimed")]
    pub space_reclaimed: Option<i64>,
}

/// Kind of change  Can be one of:  - `0`: Modified ('C') - `1`: Added ('A') - `2`: Deleted ('D')
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[napi]
pub enum ChangeType {
    _0 = 0,
    _1 = 1,
    _2 = 2,
}

/// ClusterInfo represents information about the swarm as is returned by the '/info' endpoint. Join-tokens are not included.
#[napi(object)]
pub struct ClusterInfo {
    /// The ID of the swarm.
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Version")]
    pub version: Option<ObjectVersion>,

    /// Date and time at which the swarm was initialised in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    /// Date and time at which the swarm was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "UpdatedAt")]
    pub updated_at: Option<String>,

    #[napi(js_name = "Spec")]
    pub spec: Option<SwarmSpec>,

    #[napi(js_name = "TLSInfo")]
    pub tls_info: Option<TlsInfo>,

    /// Whether there is currently a root CA rotation in progress for the swarm
    #[napi(js_name = "RootRotationInProgress")]
    pub root_rotation_in_progress: Option<bool>,

    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. If no port is set or is set to 0, the default port (4789) is used.
    #[napi(js_name = "DataPathPort")]
    pub data_path_port: Option<u32>,

    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[napi(js_name = "DefaultAddrPool")]
    pub default_addr_pool: Option<Vec<String>>,

    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[napi(js_name = "SubnetSize")]
    pub subnet_size: Option<u32>,
}

/// Options and information specific to, and only present on, Swarm CSI cluster volumes.
#[napi(object)]
pub struct ClusterVolume {
    /// The Swarm ID of this volume. Because cluster volumes are Swarm objects, they have an ID, unlike non-cluster volumes. This ID can be used to refer to the Volume instead of the name.
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Version")]
    pub version: Option<ObjectVersion>,

    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    #[napi(js_name = "UpdatedAt")]
    pub updated_at: Option<String>,

    #[napi(js_name = "Spec")]
    pub spec: Option<ClusterVolumeSpec>,

    #[napi(js_name = "Info")]
    pub info: Option<ClusterVolumeInfo>,

    /// The status of the volume as it pertains to its publishing and use on specific nodes
    #[napi(js_name = "PublishStatus")]
    pub publish_status: Option<Vec<ClusterVolumePublishStatus>>,
}

/// Information about the global status of the volume.
#[napi(object)]
pub struct ClusterVolumeInfo {
    /// The capacity of the volume in bytes. A value of 0 indicates that the capacity is unknown.
    #[napi(js_name = "CapacityBytes")]
    pub capacity_bytes: Option<i64>,

    /// A map of strings to strings returned from the storage plugin when the volume is created.
    #[napi(js_name = "VolumeContext")]
    pub volume_context: Option<HashMap<String, String>>,

    /// The ID of the volume as returned by the CSI storage plugin. This is distinct from the volume's ID as provided by Docker. This ID is never used by the user when communicating with Docker to refer to this volume. If the ID is blank, then the Volume has not been successfully created in the plugin yet.
    #[napi(js_name = "VolumeID")]
    pub volume_id: Option<String>,

    /// The topology this volume is actually accessible from.
    #[napi(js_name = "AccessibleTopology")]
    pub accessible_topology: Option<Vec<HashMap<String, Option<Vec<PortBinding>>>>>,
}

#[napi(object)]
pub struct ClusterVolumePublishStatus {
    /// The ID of the Swarm node the volume is published on.
    #[napi(js_name = "NodeID")]
    pub node_id: Option<String>,

    /// The published state of the volume. * `pending-publish` The volume should be published to this node, but the call to the controller plugin to do so has not yet been successfully completed. * `published` The volume is published successfully to the node. * `pending-node-unpublish` The volume should be unpublished from the node, and the manager is awaiting confirmation from the worker that it has done so. * `pending-controller-unpublish` The volume is successfully unpublished from the node, but has not yet been successfully unpublished on the controller.
    #[napi(js_name = "State")]
    pub state: Option<ClusterVolumePublishStatusStateEnum>,

    /// A map of strings to strings returned by the CSI controller plugin when a volume is published.
    #[napi(js_name = "PublishContext")]
    pub publish_context: Option<HashMap<String, String>>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ClusterVolumePublishStatusStateEnum {
    EMPTY,
    PENDING_PUBLISH,
    PUBLISHED,
    PENDING_NODE_UNPUBLISH,
    PENDING_CONTROLLER_UNPUBLISH,
}

/// Cluster-specific options used to create the volume.
#[napi(object)]
pub struct ClusterVolumeSpec {
    /// Group defines the volume group of this volume. Volumes belonging to the same group can be referred to by group name when creating Services.  Referring to a volume by group instructs Swarm to treat volumes in that group interchangeably for the purpose of scheduling. Volumes with an empty string for a group technically all belong to the same, emptystring group.
    #[napi(js_name = "Group")]
    pub group: Option<String>,

    #[napi(js_name = "AccessMode")]
    pub access_mode: Option<ClusterVolumeSpecAccessMode>,
}

/// Defines how the volume is used by tasks.
#[napi(object)]
pub struct ClusterVolumeSpecAccessMode {
    /// The set of nodes this volume can be used on at one time. - `single` The volume may only be scheduled to one node at a time. - `multi` the volume may be scheduled to any supported number of nodes at a time.
    #[napi(js_name = "Scope")]
    pub scope: Option<ClusterVolumeSpecAccessModeScopeEnum>,

    /// The number and way that different tasks can use this volume at one time. - `none` The volume may only be used by one task at a time. - `readonly` The volume may be used by any number of tasks, but they all must mount the volume as readonly - `onewriter` The volume may be used by any number of tasks, but only one may mount it as read/write. - `all` The volume may have any number of readers and writers.
    #[napi(js_name = "Sharing")]
    pub sharing: Option<ClusterVolumeSpecAccessModeSharingEnum>,

    /// Options for using this volume as a Mount-type volume.      Either MountVolume or BlockVolume, but not both, must be     present.   properties:     FsType:       type: 'string'       description: |         Specifies the filesystem type for the mount volume.         Optional.     MountFlags:       type: 'array'       description: |         Flags to pass when mounting the volume. Optional.       items:         type: 'string' BlockVolume:   type: 'object'   description: |     Options for using this volume as a Block-type volume.     Intentionally empty.
    #[napi(js_name = "MountVolume")]
    pub mount_volume: Option<()>,

    /// Swarm Secrets that are passed to the CSI storage plugin when operating on this volume.
    #[napi(js_name = "Secrets")]
    pub secrets: Option<Vec<ClusterVolumeSpecAccessModeSecrets>>,

    #[napi(js_name = "AccessibilityRequirements")]
    pub accessibility_requirements: Option<ClusterVolumeSpecAccessModeAccessibilityRequirements>,

    #[napi(js_name = "CapacityRange")]
    pub capacity_range: Option<ClusterVolumeSpecAccessModeCapacityRange>,

    /// The availability of the volume for use in tasks. - `active` The volume is fully available for scheduling on the cluster - `pause` No new workloads should use the volume, but existing workloads are not stopped. - `drain` All workloads using this volume should be stopped and rescheduled, and no new ones should be started.
    #[napi(js_name = "Availability")]
    pub availability: Option<ClusterVolumeSpecAccessModeAvailabilityEnum>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ClusterVolumeSpecAccessModeScopeEnum {
    EMPTY,
    SINGLE,
    MULTI,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ClusterVolumeSpecAccessModeSharingEnum {
    EMPTY,
    NONE,
    READONLY,
    ONEWRITER,
    ALL,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ClusterVolumeSpecAccessModeAvailabilityEnum {
    EMPTY,
    ACTIVE,
    PAUSE,
    DRAIN,
}

/// Requirements for the accessible topology of the volume. These fields are optional. For an in-depth description of what these fields mean, see the CSI specification.
#[napi(object)]
pub struct ClusterVolumeSpecAccessModeAccessibilityRequirements {
    /// A list of required topologies, at least one of which the volume must be accessible from.
    #[napi(js_name = "Requisite")]
    pub requisite: Option<Vec<HashMap<String, Option<Vec<PortBinding>>>>>,

    /// A list of topologies that the volume should attempt to be provisioned in.
    #[napi(js_name = "Preferred")]
    pub preferred: Option<Vec<HashMap<String, Option<Vec<PortBinding>>>>>,
}

/// The desired capacity that the volume should be created with. If empty, the plugin will decide the capacity.
#[napi(object)]
pub struct ClusterVolumeSpecAccessModeCapacityRange {
    /// The volume must be at least this big. The value of 0 indicates an unspecified minimum
    #[napi(js_name = "RequiredBytes")]
    pub required_bytes: Option<i64>,

    /// The volume must not be bigger than this. The value of 0 indicates an unspecified maximum.
    #[napi(js_name = "LimitBytes")]
    pub limit_bytes: Option<i64>,
}

/// One cluster volume secret entry. Defines a key-value pair that is passed to the plugin.
#[napi(object)]
pub struct ClusterVolumeSpecAccessModeSecrets {
    /// Key is the name of the key of the key-value pair passed to the plugin.
    #[napi(js_name = "Key")]
    pub key: Option<String>,

    /// Secret is the swarm Secret object from which to read data. This can be a Secret name or ID. The Secret data is retrieved by swarm and used as the value of the key-value pair passed to the plugin.
    #[napi(js_name = "Secret")]
    pub secret: Option<String>,
}

/// Commit holds the Git-commit (SHA1) that a binary was built from, as reported in the version-string of external tools, such as `containerd`, or `runC`.
#[napi(object)]
pub struct Commit {
    /// Actual commit ID of external tool.
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    /// Commit ID of external tool expected by dockerd as set at build time.
    #[napi(js_name = "Expected")]
    pub expected: Option<String>,
}

#[napi(object)]
pub struct Config {
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Version")]
    pub version: Option<ObjectVersion>,

    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    #[napi(js_name = "UpdatedAt")]
    pub updated_at: Option<String>,

    #[napi(js_name = "Spec")]
    pub spec: Option<ConfigSpec>,
}

/// The config-only network source to provide the configuration for this network.
#[napi(object)]
pub struct ConfigReference {
    /// The name of the config-only network that provides the network's configuration. The specified network must be an existing config-only network. Only network names are allowed, not network IDs.
    #[napi(js_name = "Network")]
    pub network: Option<String>,
}

#[napi(object)]
pub struct ConfigSpec {
    /// User-defined name of the config.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) config data.
    #[napi(js_name = "Data")]
    pub data: Option<String>,

    /// Templating driver, if applicable  Templating controls whether and how to evaluate the config payload as a template. If no driver is set, no templating is used.
    #[napi(js_name = "Templating")]
    pub templating: Option<Driver>,
}

/// Configuration for a container that is portable between hosts.
#[napi(object)]
pub struct ContainerConfig {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    #[napi(js_name = "Hostname")]
    pub hostname: Option<String>,

    /// The domain name to use for the container.
    #[napi(js_name = "Domainname")]
    pub domainname: Option<String>,

    /// The user that commands are run as inside the container.
    #[napi(js_name = "User")]
    pub user: Option<String>,

    /// Whether to attach to `stdin`.
    #[napi(js_name = "AttachStdin")]
    pub attach_stdin: Option<bool>,

    /// Whether to attach to `stdout`.
    #[napi(js_name = "AttachStdout")]
    pub attach_stdout: Option<bool>,

    /// Whether to attach to `stderr`.
    #[napi(js_name = "AttachStderr")]
    pub attach_stderr: Option<bool>,

    /// An object mapping ports to an empty object in the form:  `{'<port>/<tcp|udp|sctp>': {}}`
    #[napi(js_name = "ExposedPorts")]
    pub exposed_ports: Option<Vec<String>>,

    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[napi(js_name = "Tty")]
    pub tty: Option<bool>,

    /// Open `stdin`
    #[napi(js_name = "OpenStdin")]
    pub open_stdin: Option<bool>,

    /// Close `stdin` after one attached client disconnects
    #[napi(js_name = "StdinOnce")]
    pub stdin_once: Option<bool>,

    /// A list of environment variables to set inside the container in the form `['VAR=value', ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[napi(js_name = "Env")]
    pub env: Option<Vec<String>>,

    /// Command to run specified as a string or an array of strings.
    #[napi(js_name = "Cmd")]
    pub cmd: Option<Vec<String>>,

    #[napi(js_name = "Healthcheck")]
    pub healthcheck: Option<HealthConfig>,

    /// Command is already escaped (Windows only)
    #[napi(js_name = "ArgsEscaped")]
    pub args_escaped: Option<bool>,

    /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.
    #[napi(js_name = "Image")]
    pub image: Option<String>,

    /// An object mapping mount point paths inside the container to empty objects.
    #[napi(js_name = "Volumes")]
    pub volumes: Option<Vec<String>>,

    /// The working directory for commands to run in.
    #[napi(js_name = "WorkingDir")]
    pub working_dir: Option<String>,

    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[""]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[napi(js_name = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,

    /// Disable networking for the container.
    #[napi(js_name = "NetworkDisabled")]
    pub network_disabled: Option<bool>,

    /// MAC address of the container.  Deprecated: this field is deprecated in API v1.44 and up. Use EndpointSettings.MacAddress instead.
    #[napi(js_name = "MacAddress")]
    pub mac_address: Option<String>,

    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[napi(js_name = "OnBuild")]
    pub on_build: Option<Vec<String>>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// Signal to stop a container as a string or unsigned integer.
    #[napi(js_name = "StopSignal")]
    pub stop_signal: Option<String>,

    /// Timeout to stop a container in seconds.
    #[napi(js_name = "StopTimeout")]
    pub stop_timeout: Option<i64>,

    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[napi(js_name = "Shell")]
    pub shell: Option<Vec<String>>,
}

// todo: remove it
// /// OK response to ContainerCreate operation
// pub struct ContainerCreateResponse {
//     /// The ID of the created container
//     pub id: String,
//
//     /// Warnings encountered when creating the container
//     pub warnings: Vec<String>,
// }

#[napi(object)]
pub struct ContainerInspectResponse {
    /// The ID of the container
    #[napi(js_name = "Id")]
    pub id: Option<String>,

    /// The time the container was created
    #[napi(js_name = "Created")]
    pub created: Option<String>,

    /// The path to the command being run
    #[napi(js_name = "Path")]
    pub path: Option<String>,

    /// The arguments to the command being run
    #[napi(js_name = "Args")]
    pub args: Option<Vec<String>>,

    #[napi(js_name = "State")]
    pub state: Option<ContainerState>,

    /// The container's image ID
    #[napi(js_name = "Image")]
    pub image: Option<String>,

    #[napi(js_name = "ResolvConfPath")]
    pub resolv_conf_path: Option<String>,

    #[napi(js_name = "HostnamePath")]
    pub hostname_path: Option<String>,

    #[napi(js_name = "HostsPath")]
    pub hosts_path: Option<String>,

    #[napi(js_name = "LogPath")]
    pub log_path: Option<String>,

    #[napi(js_name = "Name")]
    pub name: Option<String>,

    #[napi(js_name = "RestartCount")]
    pub restart_count: Option<i64>,

    #[napi(js_name = "Driver")]
    pub driver: Option<String>,

    #[napi(js_name = "Platform")]
    pub platform: Option<String>,

    #[napi(js_name = "MountLabel")]
    pub mount_label: Option<String>,

    #[napi(js_name = "ProcessLabel")]
    pub process_label: Option<String>,

    #[napi(js_name = "AppArmorProfile")]
    pub app_armor_profile: Option<String>,

    /// IDs of exec instances that are running in the container.
    #[napi(js_name = "ExecIDs")]
    pub exec_ids: Option<Vec<String>>,

    #[napi(js_name = "HostConfig")]
    pub host_config: Option<HostConfig>,

    #[napi(js_name = "GraphDriver")]
    pub graph_driver: Option<DriverData>,

    /// The size of files that have been created or changed by this container.
    #[napi(js_name = "SizeRw")]
    pub size_rw: Option<i64>,

    /// The total size of all the files in this container.
    #[napi(js_name = "SizeRootFs")]
    pub size_root_fs: Option<i64>,

    #[napi(js_name = "Mounts")]
    pub mounts: Option<Vec<MountPoint>>,

    #[napi(js_name = "Config")]
    pub config: Option<ContainerConfig>,

    #[napi(js_name = "NetworkSettings")]
    pub network_settings: Option<NetworkSettings>,
}

#[napi(object)]
pub struct ContainerPruneResponse {
    /// Container IDs that were deleted
    #[napi(js_name = "ContainersDeleted")]
    pub containers_deleted: Option<Vec<String>>,

    /// Disk space reclaimed in bytes
    #[napi(js_name = "SpaceReclaimed")]
    pub space_reclaimed: Option<i64>,
}

/// ContainerState stores container's running state. It's part of ContainerJSONBase and will be returned by the 'inspect' command.
#[napi(object)]
pub struct ContainerState {
    /// String representation of the container state. Can be one of 'created', 'running', 'paused', 'restarting', 'removing', 'exited', or 'dead'.
    #[napi(js_name = "Status")]
    pub status: Option<ContainerStateStatusEnum>,

    /// Whether this container is running.  Note that a running container can be _paused_. The `Running` and `Paused` booleans are not mutually exclusive:  When pausing a container (on Linux), the freezer cgroup is used to suspend all processes in the container. Freezing the process requires the process to be running. As a result, paused containers are both `Running` _and_ `Paused`.  Use the `Status` field instead to determine if a container's state is 'running'.
    #[napi(js_name = "Running")]
    pub running: Option<bool>,

    /// Whether this container is paused.
    #[napi(js_name = "Paused")]
    pub paused: Option<bool>,

    /// Whether this container is restarting.
    #[napi(js_name = "Restarting")]
    pub restarting: Option<bool>,

    /// Whether a process within this container has been killed because it ran out of memory since the container was last started.
    #[napi(js_name = "OOMKilled")]
    pub oom_killed: Option<bool>,

    #[napi(js_name = "Dead")]
    pub dead: Option<bool>,

    /// The process ID of this container
    #[napi(js_name = "Pid")]
    pub pid: Option<i64>,

    /// The last exit code of this container
    #[napi(js_name = "ExitCode")]
    pub exit_code: Option<i64>,

    #[napi(js_name = "Error")]
    pub error: Option<String>,

    /// The time when this container was last started.
    #[napi(js_name = "StartedAt")]
    pub started_at: Option<String>,

    /// The time when this container last exited.
    #[napi(js_name = "FinishedAt")]
    pub finished_at: Option<String>,

    #[napi(js_name = "Health")]
    pub health: Option<Health>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ContainerStateStatusEnum {
    EMPTY,
    CREATED,
    RUNNING,
    PAUSED,
    RESTARTING,
    REMOVING,
    EXITED,
    DEAD,
}

/// represents the status of a container.
#[napi(object)]
pub struct ContainerStatus {
    #[napi(js_name = "ContainerID")]
    pub container_id: Option<String>,

    #[napi(js_name = "PID")]
    pub pid: Option<i64>,

    #[napi(js_name = "ExitCode")]
    pub exit_code: Option<i64>,
}

#[napi(object)]
pub struct ContainerSummary {
    /// The ID of this container
    #[napi(js_name = "Id")]
    pub id: Option<String>,

    /// The names that this container has been given
    #[napi(js_name = "Names")]
    pub names: Option<Vec<String>>,

    /// The name of the image used when creating this container
    #[napi(js_name = "Image")]
    pub image: Option<String>,

    /// The ID of the image that this container was created from
    #[napi(js_name = "ImageID")]
    pub image_id: Option<String>,

    /// Command to run when starting the container
    #[napi(js_name = "Command")]
    pub command: Option<String>,

    /// When the container was created
    #[napi(js_name = "Created")]
    pub created: Option<i64>,

    /// The ports exposed by this container
    #[napi(js_name = "Ports")]
    pub ports: Option<Vec<Port>>,

    /// The size of files that have been created or changed by this container
    #[napi(js_name = "SizeRw")]
    pub size_rw: Option<i64>,

    /// The total size of all the files in this container
    #[napi(js_name = "SizeRootFs")]
    pub size_root_fs: Option<i64>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// The state of this container (e.g. `Exited`)
    #[napi(js_name = "State")]
    pub state: Option<String>,

    /// Additional human-readable status of this container (e.g. `Exit 0`)
    #[napi(js_name = "Status")]
    pub status: Option<String>,

    #[napi(js_name = "HostConfig")]
    pub host_config: Option<ContainerSummaryHostConfig>,

    #[napi(js_name = "NetworkSettings")]
    pub network_settings: Option<ContainerSummaryNetworkSettings>,

    #[napi(js_name = "Mounts")]
    pub mounts: Option<Vec<MountPoint>>,
}

#[napi(object)]
pub struct ContainerSummaryHostConfig {
    #[napi(js_name = "NetworkMode")]
    pub network_mode: Option<String>,

    /// Arbitrary key-value metadata attached to container
    #[napi(js_name = "Annotations")]
    pub annotations: Option<HashMap<String, String>>,
}

/// A summary of the container's network settings
#[napi(object)]
pub struct ContainerSummaryNetworkSettings {
    #[napi(js_name = "Networks")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
}

/// OK response to ContainerTop operation
#[napi(object)]
pub struct ContainerTopResponse {
    /// The ps column titles
    #[napi(js_name = "Titles")]
    pub titles: Option<Vec<String>>,

    /// Each process running in the container, where each is process is an array of values corresponding to the titles.
    #[napi(js_name = "Processes")]
    pub processes: Option<Vec<Vec<String>>>,
}

/// OK response to ContainerUpdate operation
#[napi(object)]
pub struct ContainerUpdateResponse {
    #[napi(js_name = "Warnings")]
    pub warnings: Option<Vec<String>>,
}

/// container waiting error, if any
#[napi(object)]
pub struct ContainerWaitExitError {
    /// Details of an error
    #[napi(js_name = "Message")]
    pub message: Option<String>,
}

/// OK response to ContainerWait operation
#[napi(object)]
pub struct ContainerWaitResponse {
    /// Exit code of the container
    #[napi(js_name = "StatusCode")]
    pub status_code: i64,

    #[napi(js_name = "Error")]
    pub error: Option<ContainerWaitExitError>,
}

/// Information for connecting to the containerd instance that is used by the daemon. This is included for debugging purposes only.
#[napi(object)]
pub struct ContainerdInfo {
    /// The address of the containerd socket.
    #[napi(js_name = "Address")]
    pub address: Option<String>,

    #[napi(js_name = "Namespaces")]
    pub namespaces: Option<ContainerdInfoNamespaces>,
}

/// The namespaces that the daemon uses for running containers and plugins in containerd. These namespaces can be configured in the daemon configuration, and are considered to be used exclusively by the daemon, Tampering with the containerd instance may cause unexpected behavior.  As these namespaces are considered to be exclusively accessed by the daemon, it is not recommended to change these values, or to change them to a value that is used by other systems, such as cri-containerd.
#[napi(object)]
pub struct ContainerdInfoNamespaces {
    /// The default containerd namespace used for containers managed by the daemon.  The default namespace for containers is 'moby', but will be suffixed with the `<uid>.<gid>` of the remapped `root` if user-namespaces are enabled and the containerd image-store is used.
    #[napi(js_name = "Containers")]
    pub containers: Option<String>,

    /// The default containerd namespace used for plugins managed by the daemon.  The default namespace for plugins is 'plugins.moby', but will be suffixed with the `<uid>.<gid>` of the remapped `root` if user-namespaces are enabled and the containerd image-store is used.
    #[napi(js_name = "Plugins")]
    pub plugins: Option<String>,
}

#[napi(object)]
pub struct CreateImageInfo {
    #[napi(js_name = "id")]
    pub id: Option<String>,

    #[napi(js_name = "error")]
    pub error: Option<String>,

    #[napi(js_name = "errorDetail")]
    pub error_detail: Option<ErrorDetail>,

    #[napi(js_name = "status")]
    pub status: Option<String>,

    #[napi(js_name = "progress")]
    pub progress: Option<String>,

    #[napi(js_name = "progressDetail")]
    pub progress_detail: Option<ProgressDetail>,
}

/// A device mapping between the host and container
#[derive(o2o)]
#[owned_into(bollard::secret::DeviceMapping)]
#[napi(object)]
pub struct DeviceMapping {
    #[napi(js_name = "PathOnHost")]
    pub path_on_host: Option<String>,

    #[napi(js_name = "PathInContainer")]
    pub path_in_container: Option<String>,

    #[napi(js_name = "CgroupPermissions")]
    pub cgroup_permissions: Option<String>,
}

/// A request for devices to be sent to device drivers
#[derive(o2o)]
#[owned_into(bollard::secret::DeviceRequest)]
#[napi(object)]
pub struct DeviceRequest {
    #[napi(js_name = "Driver")]
    pub driver: Option<String>,

    #[napi(js_name = "Count")]
    pub count: Option<i64>,

    #[napi(js_name = "DeviceIDs")]
    pub device_ids: Option<Vec<String>>,

    /// A list of capabilities; an OR list of AND lists of capabilities.
    #[napi(js_name = "Capabilities")]
    pub capabilities: Option<Vec<Vec<String>>>,

    /// Driver-specific options, specified as a key/value pairs. These options are passed directly to the driver.
    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,
}

/// Describes the result obtained from contacting the registry to retrieve image metadata.
#[napi(object)]
pub struct DistributionInspect {
    #[napi(js_name = "Descriptor")]
    pub descriptor: OciDescriptor,

    /// An array containing all platforms supported by the image.
    #[napi(js_name = "Platforms")]
    pub platforms: Vec<OciPlatform>,
}

/// Driver represents a driver (network, logging, secrets).
#[napi(object)]
pub struct Driver {
    /// Name of the driver.
    #[napi(js_name = "Name")]
    pub name: String,

    /// Key/value map of driver-specific options.
    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,
}

/// Information about the storage driver used to store the container's and image's filesystem.
#[napi(object)]
pub struct DriverData {
    /// Name of the storage driver.
    #[napi(js_name = "Name")]
    pub name: String,

    /// Low-level storage metadata, provided as key/value pairs.  This information is driver-specific, and depends on the storage-driver in use, and should be used for informational purposes only.
    #[napi(js_name = "Data")]
    pub data: HashMap<String, String>,
}

/// EndpointIPAMConfig represents an endpoint's IPAM configuration.
#[derive(o2o)]
#[owned_into(bollard::secret::EndpointIpamConfig)]
#[napi(object)]
pub struct EndpointIpamConfig {
    #[napi(js_name = "IPv4Address")]
    pub ipv4_address: Option<String>,

    #[napi(js_name = "IPv6Address")]
    pub ipv6_address: Option<String>,

    #[napi(js_name = "LinkLocalIPs")]
    pub link_local_ips: Option<Vec<String>>,
}

#[napi(object)]
pub struct EndpointPortConfig {
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    #[napi(js_name = "Protocol")]
    pub protocol: Option<EndpointPortConfigProtocolEnum>,

    /// The port inside the container.
    #[napi(js_name = "TargetPort")]
    pub target_port: Option<i64>,

    /// The port on the swarm hosts.
    #[napi(js_name = "PublishedPort")]
    pub published_port: Option<i64>,

    /// The mode in which port is published.  <p><br /></p>  - 'ingress' makes the target port accessible on every node,   regardless of whether there is a task for the service running on   that node or not. - 'host' bypasses the routing mesh and publish the port directly on   the swarm node where that service is running.
    #[napi(js_name = "PublishMode")]
    pub publish_mode: Option<EndpointPortConfigPublishModeEnum>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum EndpointPortConfigProtocolEnum {
    EMPTY,
    TCP,
    UDP,
    SCTP,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum EndpointPortConfigPublishModeEnum {
    EMPTY,
    INGRESS,
    HOST,
}

/// Configuration for a network endpoint.
#[derive(o2o)]
#[owned_into(bollard::secret::EndpointSettings)]
#[napi(object)]
pub struct EndpointSettings {
    #[napi(js_name = "IPAMConfig")]
    #[map(~.map(Into::into))]
    pub ipam_config: Option<EndpointIpamConfig>,

    #[napi(js_name = "Links")]
    pub links: Option<Vec<String>>,

    /// MAC address for the endpoint on this network. The network driver might ignore this parameter.
    #[napi(js_name = "MacAddress")]
    pub mac_address: Option<String>,

    #[napi(js_name = "Aliases")]
    pub aliases: Option<Vec<String>>,

    /// DriverOpts is a mapping of driver options and values. These options are passed directly to the driver and are driver specific.
    #[napi(js_name = "DriverOpts")]
    pub driver_opts: Option<HashMap<String, String>>,

    /// Unique ID of the network.
    #[napi(js_name = "NetworkID")]
    pub network_id: Option<String>,

    /// Unique ID for the service endpoint in a Sandbox.
    #[napi(js_name = "EndpointID")]
    pub endpoint_id: Option<String>,

    /// Gateway address for this network.
    #[napi(js_name = "Gateway")]
    pub gateway: Option<String>,

    /// IPv4 address.
    #[napi(js_name = "IPAddress")]
    pub ip_address: Option<String>,

    /// Mask length of the IPv4 address.
    #[napi(js_name = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,

    /// IPv6 gateway address.
    #[napi(js_name = "IPv6Gateway")]
    pub ipv6_gateway: Option<String>,

    /// Global IPv6 address.
    #[napi(js_name = "GlobalIPv6Address")]
    pub global_ipv6_address: Option<String>,

    /// Mask length of the global IPv6 address.
    #[napi(js_name = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: Option<i64>,

    /// List of all DNS names an endpoint has on a specific network. This list is based on the container name, network aliases, container short ID, and hostname.  These DNS names are non-fully qualified but can contain several dots. You can get fully qualified DNS names by appending `.<network-name>`. For instance, if container name is `my.ctr` and the network is named `testnet`, `DNSNames` will contain `my.ctr` and the FQDN will be `my.ctr.testnet`.
    #[napi(js_name = "DNSNames")]
    pub dns_names: Option<Vec<String>>,
}

/// Properties that can be configured to access and load balance a service.
#[napi(object)]
pub struct EndpointSpec {
    /// The mode of resolution to use for internal load balancing between tasks.
    #[napi(js_name = "Mode")]
    pub mode: Option<EndpointSpecModeEnum>,

    /// List of exposed ports that this service is accessible on from the outside. Ports can only be provided if `vip` resolution mode is used.
    #[napi(js_name = "Ports")]
    pub ports: Option<Vec<EndpointPortConfig>>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum EndpointSpecModeEnum {
    EMPTY,
    VIP,
    DNSRR,
}

/// EngineDescription provides information about an engine.
#[napi(object)]
pub struct EngineDescription {
    #[napi(js_name = "EngineVersion")]
    pub engine_version: Option<String>,

    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    #[napi(js_name = "Plugins")]
    pub plugins: Option<Vec<EngineDescriptionPlugins>>,
}

#[napi(object)]
pub struct EngineDescriptionPlugins {
    #[napi(js_name = "Type")]
    pub typ: Option<String>,

    #[napi(js_name = "Name")]
    pub name: Option<String>,
}

#[napi(object)]
pub struct ErrorDetail {
    #[napi(js_name = "code")]
    pub code: Option<i64>,

    #[napi(js_name = "message")]
    pub message: Option<String>,
}

/// Represents an error.
#[napi(object)]
pub struct ErrorResponse {
    /// The error message.
    #[napi(js_name = "message")]
    pub message: String,
}

/// Actor describes something that generates events, like a container, network, or a volume.
#[napi(object)]
pub struct EventActor {
    /// The ID of the object emitting the event
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    /// Various key/value attributes of the object, depending on its type.
    #[napi(js_name = "Attributes")]
    pub attributes: Option<HashMap<String, String>>,
}

/// EventMessage represents the information an event contains.
#[napi(object)]
pub struct EventMessage {
    /// The type of object emitting the event
    #[napi(js_name = "Type")]
    pub typ: Option<EventMessageTypeEnum>,

    /// The type of event
    #[napi(js_name = "Action")]
    pub action: Option<String>,

    #[napi(js_name = "Actor")]
    pub actor: Option<EventActor>,

    /// Scope of the event. Engine events are `local` scope. Cluster (Swarm) events are `swarm` scope.
    #[napi(js_name = "scope")]
    pub scope: Option<EventMessageScopeEnum>,

    /// Timestamp of event
    #[napi(js_name = "time")]
    pub time: Option<i64>,

    /// Timestamp of event, with nanosecond accuracy
    #[napi(js_name = "timeNano")]
    pub time_nano: Option<i64>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum EventMessageTypeEnum {
    EMPTY,
    BUILDER,
    CONFIG,
    CONTAINER,
    DAEMON,
    IMAGE,
    NETWORK,
    NODE,
    PLUGIN,
    SECRET,
    SERVICE,
    VOLUME,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum EventMessageScopeEnum {
    EMPTY,
    LOCAL,
    SWARM,
}

#[napi(object)]
pub struct ExecConfig {
    /// Attach to `stdin` of the exec command.
    #[napi(js_name = "AttachStdin")]
    pub attach_stdin: Option<bool>,

    /// Attach to `stdout` of the exec command.
    #[napi(js_name = "AttachStdout")]
    pub attach_stdout: Option<bool>,

    /// Attach to `stderr` of the exec command.
    #[napi(js_name = "AttachStderr")]
    pub attach_stderr: Option<bool>,

    /// Initial console size, as an `[height, width]` array.
    #[napi(js_name = "ConsoleSize")]
    pub console_size: Option<Vec<i32>>,

    /// Override the key sequence for detaching a container. Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
    #[napi(js_name = "DetachKeys")]
    pub detach_keys: Option<String>,

    /// Allocate a pseudo-TTY.
    #[napi(js_name = "Tty")]
    pub tty: Option<bool>,

    /// A list of environment variables in the form `['VAR=value', ...]`.
    #[napi(js_name = "Env")]
    pub env: Option<Vec<String>>,

    /// Command to run, as a string or array of strings.
    #[napi(js_name = "Cmd")]
    pub cmd: Option<Vec<String>>,

    /// Runs the exec process with extended privileges.
    #[napi(js_name = "Privileged")]
    pub privileged: Option<bool>,

    /// The user, and optionally, group to run the exec process inside the container. Format is one of: `user`, `user:group`, `uid`, or `uid:gid`.
    #[napi(js_name = "User")]
    pub user: Option<String>,

    /// The working directory for the exec process inside the container.
    #[napi(js_name = "WorkingDir")]
    pub working_dir: Option<String>,
}

#[napi(object)]
pub struct ExecInspectResponse {
    #[napi(js_name = "CanRemove")]
    pub can_remove: Option<bool>,

    #[napi(js_name = "DetachKeys")]
    pub detach_keys: Option<String>,

    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Running")]
    pub running: Option<bool>,

    #[napi(js_name = "ExitCode")]
    pub exit_code: Option<i64>,

    #[napi(js_name = "ProcessConfig")]
    pub process_config: Option<ProcessConfig>,

    #[napi(js_name = "OpenStdin")]
    pub open_stdin: Option<bool>,

    #[napi(js_name = "OpenStderr")]
    pub open_stderr: Option<bool>,

    #[napi(js_name = "OpenStdout")]
    pub open_stdout: Option<bool>,

    #[napi(js_name = "ContainerID")]
    pub container_id: Option<String>,

    /// The system process ID for the exec process.
    #[napi(js_name = "Pid")]
    pub pid: Option<i64>,
}

#[napi(object)]
pub struct ExecStartConfig {
    /// Detach from the command.
    #[napi(js_name = "Detach")]
    pub detach: Option<bool>,

    /// Allocate a pseudo-TTY.
    #[napi(js_name = "Tty")]
    pub tty: Option<bool>,

    /// Initial console size, as an `[height, width]` array.
    #[napi(js_name = "ConsoleSize")]
    pub console_size: Option<Vec<i32>>,
}

/// Change in the container's filesystem.
#[napi(object)]
pub struct FilesystemChange {
    /// Path to file or directory that has changed.
    #[napi(js_name = "Path")]
    pub path: String,

    #[napi(js_name = "Kind")]
    pub kind: u8,
}

/// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, `GPU=UUID1`).
#[napi(object)]
pub struct GenericResourcesInner {
    #[napi(js_name = "NamedResourceSpec")]
    pub named_resource_spec: Option<GenericResourcesInnerNamedResourceSpec>,

    #[napi(js_name = "DiscreteResourceSpec")]
    pub discrete_resource_spec: Option<GenericResourcesInnerDiscreteResourceSpec>,
}

#[napi(object)]
pub struct GenericResourcesInnerDiscreteResourceSpec {
    #[napi(js_name = "Kind")]
    pub kind: Option<String>,

    #[napi(js_name = "Value")]
    pub value: Option<i64>,
}

#[napi(object)]
pub struct GenericResourcesInnerNamedResourceSpec {
    #[napi(js_name = "Kind")]
    pub kind: Option<String>,

    #[napi(js_name = "Value")]
    pub value: Option<String>,
}

/// Health stores information about the container's healthcheck results.
#[napi(object)]
pub struct Health {
    /// Status is one of `none`, `starting`, `healthy` or `unhealthy`  - 'none'      Indicates there is no healthcheck - 'starting'  Starting indicates that the container is not yet ready - 'healthy'   Healthy indicates that the container is running correctly - 'unhealthy' Unhealthy indicates that the container has a problem
    #[napi(js_name = "Status")]
    pub status: Option<HealthStatusEnum>,

    /// FailingStreak is the number of consecutive failures
    #[napi(js_name = "FailingStreak")]
    pub failing_streak: Option<i64>,

    /// Log contains the last few results (oldest first)
    #[napi(js_name = "Log")]
    pub log: Option<Vec<HealthcheckResult>>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum HealthStatusEnum {
    EMPTY,
    NONE,
    STARTING,
    HEALTHY,
    UNHEALTHY,
}

/// A test to perform to check that the container is healthy.
#[derive(o2o)]
#[owned_into(bollard::secret::HealthConfig)]
#[napi(object)]
pub struct HealthConfig {
    /// The test to perform. Possible values are:  - `[]` inherit healthcheck from image or parent image - `['NONE']` disable healthcheck - `['CMD', args...]` exec arguments directly - `['CMD-SHELL', command]` run command with system's default shell
    #[napi(js_name = "Test")]
    pub test: Option<Vec<String>>,

    /// The time to wait between checks in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[napi(js_name = "Interval")]
    pub interval: Option<i64>,

    /// The time to wait before considering the check to have hung. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[napi(js_name = "Timeout")]
    pub timeout: Option<i64>,

    /// The number of consecutive failures needed to consider a container as unhealthy. 0 means inherit.
    #[napi(js_name = "Retries")]
    pub retries: Option<i64>,

    /// Start period for the container to initialize before starting health-retries countdown in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[napi(js_name = "StartPeriod")]
    pub start_period: Option<i64>,

    /// The time to wait between checks in nanoseconds during the start period. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[napi(js_name = "StartInterval")]
    pub start_interval: Option<i64>,
}

/// HealthcheckResult stores information about a single run of a healthcheck probe
#[napi(object)]
pub struct HealthcheckResult {
    /// Date and time at which this check started in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "Start")]
    pub start: Option<String>,

    /// Date and time at which this check ended in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "End")]
    pub end: Option<String>,

    /// ExitCode meanings:  - `0` healthy - `1` unhealthy - `2` reserved (considered unhealthy) - other values: error running probe
    #[napi(js_name = "ExitCode")]
    pub exit_code: Option<i64>,

    /// Output from last check
    #[napi(js_name = "Output")]
    pub output: Option<String>,
}

/// individual image layer information in response to ImageHistory operation
#[napi(object)]
pub struct HistoryResponseItem {
    #[napi(js_name = "Id")]
    pub id: String,

    #[napi(js_name = "Created")]
    pub created: i64,

    #[napi(js_name = "CreatedBy")]
    pub created_by: String,

    #[napi(js_name = "Tags")]
    pub tags: Vec<String>,

    #[napi(js_name = "Size")]
    pub size: i64,

    #[napi(js_name = "Comment")]
    pub comment: String,
}

/// Container configuration that depends on the host we are running on
#[derive(o2o)]
#[owned_into(bollard::secret::HostConfig)]
#[napi(object)]
pub struct HostConfig {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[napi(js_name = "CpuShares")]
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

    /// Block IO weight (relative device weight) in the form:  ``` [{ 'Path':  'device_path',  'Weight': weight}] ```
    #[napi(js_name = "BlkioWeightDevice")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
    pub blkio_weight_device: Option<Vec<ResourcesBlkioWeightDevice>>,

    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{'Path': 'device_path', 'Rate': rate}] ```
    #[napi(js_name = "BlkioDeviceReadBps")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{'Path': 'device_path', 'Rate': rate}] ```
    #[napi(js_name = "BlkioDeviceWriteBps")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,

    /// Limit read rate (IO per second) from a device, in the form:  ``` [{'Path': 'device_path', 'Rate': rate}] ```
    #[napi(js_name = "BlkioDeviceReadIOps")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
    pub blkio_device_read_iops: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (IO per second) to a device, in the form:  ``` [{'Path': 'device_path', 'Rate': rate}] ```
    #[napi(js_name = "BlkioDeviceWriteIOps")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
    pub blkio_device_write_iops: Option<Vec<ThrottleDevice>>,

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

    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[napi(js_name = "CpusetCpus")]
    pub cpuset_cpus: Option<String>,

    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[napi(js_name = "CpusetMems")]
    pub cpuset_mems: Option<String>,

    /// A list of devices to add to the container.
    #[napi(js_name = "Devices")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
    pub devices: Option<Vec<DeviceMapping>>,

    /// a list of cgroup rules to apply to the container
    #[napi(js_name = "DeviceCgroupRules")]
    pub device_cgroup_rules: Option<Vec<String>>,

    /// A list of requests for devices to be sent to device drivers.
    #[napi(js_name = "DeviceRequests")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
    pub device_requests: Option<Vec<DeviceRequest>>,

    /// Hard limit for kernel TCP buffer memory (in bytes). Depending on the OCI runtime in use, this option may be ignored. It is no longer supported by the default (runc) runtime.  This field is omitted when empty.
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

    /// A list of resource limits to set in the container. For example:  ``` {'Name': 'nofile', 'Soft': 1024, 'Hard': 2048} ```
    #[napi(js_name = "Ulimits")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
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

    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[napi(js_name = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<i64>,

    /// A list of volume bindings for this container. Each volume binding is a string in one of these forms:  - `host-src:container-dest[:options]` to bind-mount a host path   into the container. Both `host-src`, and `container-dest` must   be an _absolute_ path. - `volume-name:container-dest[:options]` to bind-mount a volume   managed by a volume driver into the container. `container-dest`   must be an _absolute_ path.  `options` is an optional, comma-delimited list of:  - `nocopy` disables automatic copying of data from the container   path to the volume. The `nocopy` flag only applies to named volumes. - `[ro|rw]` mounts a volume read-only or read-write, respectively.   If omitted or set to `rw`, volumes are mounted read-write. - `[z|Z]` applies SELinux labels to allow or deny multiple containers   to read and write to the same volume.     - `z`: a _shared_ content label is applied to the content. This       label indicates that multiple containers can share the volume       content, for both reading and writing.     - `Z`: a _private unshared_ label is applied to the content.       This label indicates that only the current container can use       a private volume. Labeling systems such as SELinux require       proper labels to be placed on volume content that is mounted       into a container. Without a label, the security system can       prevent a container's processes from using the content. By       default, the labels set by the host operating system are not       modified. - `[[r]shared|[r]slave|[r]private]` specifies mount   [propagation behavior](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt).   This only applies to bind-mounted volumes, not internal volumes   or named volumes. Mount propagation requires the source mount   point (the location where the source directory is mounted in the   host operating system) to have the correct propagation properties.   For shared volumes, the source mount point must be set to `shared`.   For slave volumes, the mount must be set to either `shared` or   `slave`.
    #[napi(js_name = "Binds")]
    pub binds: Option<Vec<String>>,

    /// Path to a file where the container ID is written
    #[napi(js_name = "ContainerIDFile")]
    pub container_id_file: Option<String>,

    #[napi(js_name = "LogConfig")]
    #[map(~.map(Into::into))]
    pub log_config: Option<HostConfigLogConfig>,

    /// Network mode to use for this container. Supported standard values are: `bridge`, `host`, `none`, and `container:<name|id>`. Any other value is taken as a custom network's name to which this container should connect to.
    #[napi(js_name = "NetworkMode")]
    pub network_mode: Option<String>,

    #[napi(js_name = "PortBindings")]
    #[map(~.map(|o| covert_port_map(o)))]
    pub port_bindings: Option<HashMap<String, Option<Vec<PortBinding>>>>,

    #[napi(js_name = "RestartPolicy")]
    #[map(~.map(Into::into))]
    pub restart_policy: Option<RestartPolicy>,

    /// Automatically remove the container when the container's process exits. This has no effect if `RestartPolicy` is set.
    #[napi(js_name = "AutoRemove")]
    pub auto_remove: Option<bool>,

    /// Driver that this container uses to mount volumes.
    #[napi(js_name = "VolumeDriver")]
    pub volume_driver: Option<String>,

    /// A list of volumes to inherit from another container, specified in the form `<container name>[:<ro|rw>]`.
    #[napi(js_name = "VolumesFrom")]
    pub volumes_from: Option<Vec<String>>,

    /// Specification for mounts to be added to the container.
    #[napi(js_name = "Mounts")]
    #[map(~.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>()))]
    pub mounts: Option<Vec<Mount>>,

    /// Initial console size, as an `[height, width]` array.
    #[napi(js_name = "ConsoleSize")]
    pub console_size: Option<Vec<i32>>,

    /// Arbitrary non-identifying metadata attached to container and provided to the runtime when the container is started.
    #[napi(js_name = "Annotations")]
    pub annotations: Option<HashMap<String, String>>,

    /// A list of kernel capabilities to add to the container. Conflicts with option 'Capabilities'.
    #[napi(js_name = "CapAdd")]
    pub cap_add: Option<Vec<String>>,

    /// A list of kernel capabilities to drop from the container. Conflicts with option 'Capabilities'.
    #[napi(js_name = "CapDrop")]
    pub cap_drop: Option<Vec<String>>,

    /// cgroup namespace mode for the container. Possible values are:  - `'private'`: the container runs in its own private cgroup namespace - `'host'`: use the host system's cgroup namespace  If not specified, the daemon default is used, which can either be `'private'` or `'host'`, depending on daemon version, kernel support and configuration.
    #[napi(js_name = "CgroupnsMode")]
    #[map(~.map(Into::into))]
    pub cgroupns_mode: Option<HostConfigCgroupnsModeEnum>,

    /// A list of DNS servers for the container to use.
    #[napi(js_name = "Dns")]
    pub dns: Option<Vec<String>>,

    /// A list of DNS options.
    #[napi(js_name = "DnsOptions")]
    pub dns_options: Option<Vec<String>>,

    /// A list of DNS search domains.
    #[napi(js_name = "DnsSearch")]
    pub dns_search: Option<Vec<String>>,

    /// A list of hostnames/IP mappings to add to the container's `/etc/hosts` file. Specified in the form `['hostname:IP']`.
    #[napi(js_name = "ExtraHosts")]
    pub extra_hosts: Option<Vec<String>>,

    /// A list of additional groups that the container process will run as.
    #[napi(js_name = "GroupAdd")]
    pub group_add: Option<Vec<String>>,

    /// IPC sharing mode for the container. Possible values are:  - `'none'`: own private IPC namespace, with /dev/shm not mounted - `'private'`: own private IPC namespace - `'shareable'`: own private IPC namespace, with a possibility to share it with other containers - `'container:<name|id>'`: join another (shareable) container's IPC namespace - `'host'`: use the host system's IPC namespace  If not specified, daemon default is used, which can either be `'private'` or `'shareable'`, depending on daemon version and configuration.
    #[napi(js_name = "IpcMode")]
    pub ipc_mode: Option<String>,

    /// Cgroup to use for the container.
    #[napi(js_name = "Cgroup")]
    pub cgroup: Option<String>,

    /// A list of links for the container in the form `container_name:alias`.
    #[napi(js_name = "Links")]
    pub links: Option<Vec<String>>,

    /// An integer value containing the score given to the container in order to tune OOM killer preferences.
    #[napi(js_name = "OomScoreAdj")]
    pub oom_score_adj: Option<i64>,

    /// Set the PID (Process) Namespace mode for the container. It can be either:  - `'container:<name|id>'`: joins another container's PID namespace - `'host'`: use the host's PID namespace inside the container
    #[napi(js_name = "PidMode")]
    pub pid_mode: Option<String>,

    /// Gives the container full access to the host.
    #[napi(js_name = "Privileged")]
    pub privileged: Option<bool>,

    /// Allocates an ephemeral host port for all of a container's exposed ports.  Ports are de-allocated when the container stops and allocated when the container starts. The allocated port might be changed when restarting the container.  The port is selected from the ephemeral port range that depends on the kernel. For example, on Linux the range is defined by `/proc/sys/net/ipv4/ip_local_port_range`.
    #[napi(js_name = "PublishAllPorts")]
    pub publish_all_ports: Option<bool>,

    /// Mount the container's root filesystem as read only.
    #[napi(js_name = "ReadonlyRootfs")]
    pub readonly_rootfs: Option<bool>,

    /// A list of string values to customize labels for MLS systems, such as SELinux.
    #[napi(js_name = "SecurityOpt")]
    pub security_opt: Option<Vec<String>>,

    /// Storage driver options for this container, in the form `{'size': '120G'}`.
    #[napi(js_name = "StorageOpt")]
    pub storage_opt: Option<HashMap<String, String>>,

    /// A map of container directories which should be replaced by tmpfs mounts, and their corresponding mount options. For example:  ``` { '/run': 'rw,noexec,nosuid,size=65536k' } ```
    #[napi(js_name = "Tmpfs")]
    pub tmpfs: Option<HashMap<String, String>>,

    /// UTS namespace to use for the container.
    #[napi(js_name = "UTSMode")]
    pub uts_mode: Option<String>,

    /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
    #[napi(js_name = "UsernsMode")]
    pub userns_mode: Option<String>,

    /// Size of `/dev/shm` in bytes. If omitted, the system uses 64MB.
    #[napi(js_name = "ShmSize")]
    pub shm_size: Option<i64>,

    /// A list of kernel parameters (sysctls) to set in the container. For example:  ``` {'net.ipv4.ip_forward': '1'} ```
    #[napi(js_name = "Sysctls")]
    pub sysctls: Option<HashMap<String, String>>,

    /// Runtime to use with this container.
    #[napi(js_name = "Runtime")]
    pub runtime: Option<String>,

    /// Isolation technology of the container. (Windows only)
    #[napi(js_name = "Isolation")]
    #[map(~.map(Into::into))]
    pub isolation: Option<HostConfigIsolationEnum>,

    /// The list of paths to be masked inside the container (this overrides the default set of paths).
    #[napi(js_name = "MaskedPaths")]
    pub masked_paths: Option<Vec<String>>,

    /// The list of paths to be set as read-only inside the container (this overrides the default set of paths).
    #[napi(js_name = "ReadonlyPaths")]
    pub readonly_paths: Option<Vec<String>>,
}

#[derive(o2o)]
#[owned_into(bollard::secret::HostConfigCgroupnsModeEnum)]
#[allow(non_camel_case_types)]
#[napi]
pub enum HostConfigCgroupnsModeEnum {
    EMPTY,
    PRIVATE,
    HOST,
}

#[derive(o2o)]
#[owned_into(bollard::secret::HostConfigIsolationEnum)]
#[allow(non_camel_case_types)]
#[napi]
pub enum HostConfigIsolationEnum {
    EMPTY,
    DEFAULT,
    PROCESS,
    HYPERV,
}

/// The logging configuration for this container
#[derive(o2o)]
#[owned_into(bollard::secret::HostConfigLogConfig)]
#[napi(object)]
pub struct HostConfigLogConfig {
    #[napi(js_name = "Type")]
    pub typ: Option<String>,

    #[napi(js_name = "Config")]
    pub config: Option<HashMap<String, String>>,
}

/// Response to an API call that returns just an Id
#[napi(object)]
pub struct IdResponse {
    /// The id of the newly created object.
    pub id: String,
}

/// Configuration of the image. These fields are used as defaults when starting a container from the image.
#[napi(object)]
pub struct ImageConfig {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always empty. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "Hostname")]
    pub hostname: Option<String>,

    /// The domain name to use for the container.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always empty. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "Domainname")]
    pub domainname: Option<String>,

    /// The user that commands are run as inside the container.
    #[napi(js_name = "User")]
    pub user: Option<String>,

    /// Whether to attach to `stdin`.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "AttachStdin")]
    pub attach_stdin: Option<bool>,

    /// Whether to attach to `stdout`.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "AttachStdout")]
    pub attach_stdout: Option<bool>,

    /// Whether to attach to `stderr`.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "AttachStderr")]
    pub attach_stderr: Option<bool>,

    /// An object mapping ports to an empty object in the form:  `{'<port>/<tcp|udp|sctp>': {}}`
    #[napi(js_name = "ExposedPorts")]
    pub exposed_ports: Option<Vec<String>>,

    /// Attach standard streams to a TTY, including `stdin` if it is not closed.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "Tty")]
    pub tty: Option<bool>,

    /// Open `stdin`  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "OpenStdin")]
    pub open_stdin: Option<bool>,

    /// Close `stdin` after one attached client disconnects.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "StdinOnce")]
    pub stdin_once: Option<bool>,

    /// A list of environment variables to set inside the container in the form `['VAR=value', ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[napi(js_name = "Env")]
    pub env: Option<Vec<String>>,

    /// Command to run specified as a string or an array of strings.
    #[napi(js_name = "Cmd")]
    pub cmd: Option<Vec<String>>,

    #[napi(js_name = "Healthcheck")]
    pub healthcheck: Option<HealthConfig>,

    /// Command is already escaped (Windows only)
    #[napi(js_name = "ArgsEscaped")]
    pub args_escaped: Option<bool>,

    /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always empty. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "Image")]
    pub image: Option<String>,

    /// An object mapping mount point paths inside the container to empty objects.
    #[napi(js_name = "Volumes")]
    pub volumes: Option<Vec<String>>,

    /// The working directory for commands to run in.
    #[napi(js_name = "WorkingDir")]
    pub working_dir: Option<String>,

    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[""]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[napi(js_name = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,

    /// Disable networking for the container.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always omitted. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "NetworkDisabled")]
    pub network_disabled: Option<bool>,

    /// MAC address of the container.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always omitted. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "MacAddress")]
    pub mac_address: Option<String>,

    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[napi(js_name = "OnBuild")]
    pub on_build: Option<Vec<String>>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// Signal to stop a container as a string or unsigned integer.
    #[napi(js_name = "StopSignal")]
    pub stop_signal: Option<String>,

    /// Timeout to stop a container in seconds.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always omitted. It must not be used, and will be removed in API v1.48.
    #[napi(js_name = "StopTimeout")]
    pub stop_timeout: Option<i64>,

    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[napi(js_name = "Shell")]
    pub shell: Option<Vec<String>>,
}

#[napi(object)]
pub struct ImageDeleteResponseItem {
    /// The image ID of an image that was untagged
    #[napi(js_name = "Untagged")]
    pub untagged: Option<String>,

    /// The image ID of an image that was deleted
    #[napi(js_name = "Deleted")]
    pub deleted: Option<String>,
}

/// Image ID or Digest
#[napi(object)]
pub struct ImageId {
    #[napi(js_name = "ID")]
    pub id: Option<String>,
}

/// Information about an image in the local image cache.
#[napi(object)]
pub struct ImageInspect {
    /// ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.
    #[napi(js_name = "Id")]
    pub id: Option<String>,

    /// List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is 'untagged', in which case it can still be referenced by its ID.
    #[napi(js_name = "RepoTags")]
    pub repo_tags: Option<Vec<String>>,

    /// List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.
    #[napi(js_name = "RepoDigests")]
    pub repo_digests: Option<Vec<String>>,

    /// ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.
    #[napi(js_name = "Parent")]
    pub parent: Option<String>,

    /// Optional message that was set when committing or importing the image.
    #[napi(js_name = "Comment")]
    pub comment: Option<String>,

    /// Date and time at which the image was created, formatted in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if present in the image, and omitted otherwise.
    #[napi(js_name = "Created")]
    pub created: Option<String>,

    /// The version of Docker that was used to build the image.  Depending on how the image was created, this field may be empty.
    #[napi(js_name = "DockerVersion")]
    pub docker_version: Option<String>,

    /// Name of the author that was specified when committing the image, or as specified through MAINTAINER (deprecated) in the Dockerfile.
    #[napi(js_name = "Author")]
    pub author: Option<String>,

    #[napi(js_name = "Config")]
    pub config: Option<ImageConfig>,

    /// Hardware CPU architecture that the image runs on.
    #[napi(js_name = "Architecture")]
    pub architecture: Option<String>,

    /// CPU architecture variant (presently ARM-only).
    #[napi(js_name = "Variant")]
    pub variant: Option<String>,

    /// Operating System the image is built to run on.
    #[napi(js_name = "Os")]
    pub os: Option<String>,

    /// Operating System version the image is built to run on (especially for Windows).
    #[napi(js_name = "OsVersion")]
    pub os_version: Option<String>,

    /// Total size of the image including all layers it is composed of.
    #[napi(js_name = "Size")]
    pub size: Option<i64>,

    /// Total size of the image including all layers it is composed of.  Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.
    #[napi(js_name = "VirtualSize")]
    pub virtual_size: Option<i64>,

    #[napi(js_name = "GraphDriver")]
    pub graph_driver: Option<DriverData>,

    #[napi(js_name = "RootFS")]
    pub root_fs: Option<ImageInspectRootFs>,

    #[napi(js_name = "Metadata")]
    pub metadata: Option<ImageInspectMetadata>,
}

/// Additional metadata of the image in the local cache. This information is local to the daemon, and not part of the image itself.
#[napi(object)]
pub struct ImageInspectMetadata {
    /// Date and time at which the image was last tagged in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if the image was tagged locally, and omitted otherwise.
    #[napi(js_name = "LastTagTime")]
    pub last_tag_time: Option<String>,
}

/// Information about the image's RootFS, including the layer IDs.
#[napi(object)]
pub struct ImageInspectRootFs {
    #[napi(js_name = "Type")]
    pub typ: String,

    #[napi(js_name = "Layers")]
    pub layers: Option<Vec<String>>,
}

/// ImageManifestSummary represents a summary of an image manifest.
#[napi(object)]
pub struct ImageManifestSummary {
    /// ID is the content-addressable ID of an image and is the same as the digest of the image manifest.
    #[napi(js_name = "ID")]
    pub id: String,

    #[napi(js_name = "Descriptor")]
    pub descriptor: OciDescriptor,

    /// Indicates whether all the child content (image config, layers) is fully available locally.
    #[napi(js_name = "Available")]
    pub available: bool,

    #[napi(js_name = "Size")]
    pub size: ImageManifestSummarySize,

    /// The kind of the manifest.  kind         | description -------------|----------------------------------------------------------- image        | Image manifest that can be used to start a container. attestation  | Attestation manifest produced by the Buildkit builder for a specific image manifest.
    #[napi(js_name = "Kind")]
    pub kind: Option<ImageManifestSummaryKindEnum>,

    #[napi(js_name = "ImageData")]
    pub image_data: Option<ImageManifestSummaryImageData>,

    #[napi(js_name = "AttestationData")]
    pub attestation_data: Option<ImageManifestSummaryAttestationData>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ImageManifestSummaryKindEnum {
    EMPTY,
    IMAGE,
    ATTESTATION,
    UNKNOWN,
}

/// The image data for the attestation manifest. This field is only populated when Kind is 'attestation'.
#[napi(object)]
pub struct ImageManifestSummaryAttestationData {
    /// The digest of the image manifest that this attestation is for.
    #[napi(js_name = "For")]
    pub _for: String,
}

/// The image data for the image manifest. This field is only populated when Kind is 'image'.
#[napi(object)]
pub struct ImageManifestSummaryImageData {
    /// OCI platform of the image. This will be the platform specified in the manifest descriptor from the index/manifest list. If it's not available, it will be obtained from the image config.
    #[napi(js_name = "Platform")]
    pub platform: OciPlatform,

    /// The IDs of the containers that are using this image.
    #[napi(js_name = "Containers")]
    pub containers: Vec<String>,

    #[napi(js_name = "Size")]
    pub size: ImageManifestSummaryImageDataSize,
}

#[napi(object)]
pub struct ImageManifestSummaryImageDataSize {
    /// Unpacked is the size (in bytes) of the locally unpacked (uncompressed) image content that's directly usable by the containers running this image. It's independent of the distributable content - e.g. the image might still have an unpacked data that's still used by some container even when the distributable/compressed content is already gone.
    #[napi(js_name = "Unpacked")]
    pub unpacked: i64,
}

#[napi(object)]
pub struct ImageManifestSummarySize {
    /// Total is the total size (in bytes) of all the locally present data (both distributable and non-distributable) that's related to this manifest and its children. This equal to the sum of [Content] size AND all the sizes in the [Size] struct present in the Kind-specific data struct. For example, for an image kind (Kind == 'image') this would include the size of the image content and unpacked image snapshots ([Size.Content] + [ImageData.Size.Unpacked]).
    #[napi(js_name = "Total")]
    pub total: i64,

    /// Content is the size (in bytes) of all the locally present content in the content store (e.g. image config, layers) referenced by this manifest and its children. This only includes blobs in the content store.
    #[napi(js_name = "Content")]
    pub content: i64,
}

#[napi(object)]
pub struct ImagePruneResponse {
    /// Images that were deleted
    #[napi(js_name = "ImagesDeleted")]
    pub images_deleted: Option<Vec<ImageDeleteResponseItem>>,

    /// Disk space reclaimed in bytes
    #[napi(js_name = "SpaceReclaimed")]
    pub space_reclaimed: Option<i64>,
}

#[napi(object)]
pub struct ImageSearchResponseItem {
    #[napi(js_name = "description")]
    pub description: Option<String>,

    #[napi(js_name = "is_official")]
    pub is_official: Option<bool>,

    /// Whether this repository has automated builds enabled.  <p><br /></p>  > **Deprecated**: This field is deprecated and will always be 'false'.
    #[napi(js_name = "is_automated")]
    pub is_automated: Option<bool>,

    #[napi(js_name = "name")]
    pub name: Option<String>,

    #[napi(js_name = "star_count")]
    pub star_count: Option<i64>,
}

#[napi(object)]
pub struct ImageSummary {
    /// ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.
    #[napi(js_name = "Id")]
    pub id: String,

    /// ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.
    #[napi(js_name = "ParentId")]
    pub parent_id: String,

    /// List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is 'untagged', in which case it can still be referenced by its ID.
    #[napi(js_name = "RepoTags")]
    pub repo_tags: Vec<String>,

    /// List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.
    #[napi(js_name = "RepoDigests")]
    pub repo_digests: Vec<String>,

    /// Date and time at which the image was created as a Unix timestamp (number of seconds since EPOCH).
    #[napi(js_name = "Created")]
    pub created: i64,

    /// Total size of the image including all layers it is composed of.
    #[napi(js_name = "Size")]
    pub size: i64,

    /// Total size of image layers that are shared between this image and other images.  This size is not calculated by default. `-1` indicates that the value has not been set / calculated.
    #[napi(js_name = "SharedSize")]
    pub shared_size: i64,

    /// Total size of the image including all layers it is composed of.  Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.
    #[napi(js_name = "VirtualSize")]
    pub virtual_size: Option<i64>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: HashMap<String, String>,

    /// Number of containers using this image. Includes both stopped and running containers.  This size is not calculated by default, and depends on which API endpoint is used. `-1` indicates that the value has not been set / calculated.
    #[napi(js_name = "Containers")]
    pub containers: i64,

    /// Manifests is a list of manifests available in this image. It provides a more detailed view of the platform-specific image manifests or other image-attached data like build attestations.  WARNING: This is experimental and may change at any time without any backward compatibility.
    #[napi(js_name = "Manifests")]
    pub manifests: Option<Vec<ImageManifestSummary>>,
}

/// IndexInfo contains information about a registry.
#[napi(object)]
pub struct IndexInfo {
    /// Name of the registry, such as 'docker.io'.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// List of mirrors, expressed as URIs.
    #[napi(js_name = "Mirrors")]
    pub mirrors: Option<Vec<String>>,

    /// Indicates if the registry is part of the list of insecure registries.  If `false`, the registry is insecure. Insecure registries accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from unknown CAs) communication.  > **Warning**: Insecure registries can be useful when running a local > registry. However, because its use creates security vulnerabilities > it should ONLY be enabled for testing purposes. For increased > security, users should add their CA to their system's list of > trusted CAs instead of enabling this option.
    #[napi(js_name = "Secure")]
    pub secure: Option<bool>,

    /// Indicates whether this is an official registry (i.e., Docker Hub / docker.io)
    #[napi(js_name = "Official")]
    pub official: Option<bool>,
}

#[napi(object)]
pub struct Ipam {
    /// Name of the IPAM driver to use.
    #[napi(js_name = "Driver")]
    pub driver: Option<String>,

    /// List of IPAM configuration options, specified as a map:  ``` {'Subnet': <CIDR>, 'IPRange': <CIDR>, 'Gateway': <IP address>, 'AuxAddress': <device_name:IP address>} ```
    #[napi(js_name = "Config")]
    pub config: Option<Vec<IpamConfig>>,

    /// Driver-specific options, specified as a map.
    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,
}

#[napi(object)]
pub struct IpamConfig {
    #[napi(js_name = "Subnet")]
    pub subnet: Option<String>,

    #[napi(js_name = "IPRange")]
    pub ip_range: Option<String>,

    #[napi(js_name = "Gateway")]
    pub gateway: Option<String>,

    #[napi(js_name = "AuxiliaryAddresses")]
    pub auxiliary_addresses: Option<HashMap<String, String>>,
}

/// JoinTokens contains the tokens workers and managers need to join the swarm.
#[napi(object)]
pub struct JoinTokens {
    /// The token workers can use to join the swarm.
    #[napi(js_name = "Worker")]
    pub worker: Option<String>,

    /// The token managers can use to join the swarm.
    #[napi(js_name = "Manager")]
    pub manager: Option<String>,
}

/// An object describing a limit on resources which can be requested by a task.
#[napi(object)]
pub struct Limit {
    #[napi(js_name = "NanoCPUs")]
    pub nano_cpus: Option<i64>,

    #[napi(js_name = "MemoryBytes")]
    pub memory_bytes: Option<i64>,

    /// Limits the maximum number of PIDs in the container. Set `0` for unlimited.
    #[napi(js_name = "Pids")]
    pub pids: Option<i64>,
}

/// Current local status of this node.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[napi]
pub enum LocalNodeState {
    EMPTY,
    INACTIVE,
    PENDING,
    ACTIVE,
    ERROR,
    LOCKED,
}

/// ManagerStatus represents the status of a manager.  It provides the current status of a node's manager component, if the node is a manager.
#[napi(object)]
pub struct ManagerStatus {
    #[napi(js_name = "Leader")]
    pub leader: Option<bool>,

    #[napi(js_name = "Reachability")]
    pub reachability: Option<Reachability>,

    /// The IP address and port at which the manager is reachable.
    #[napi(js_name = "Addr")]
    pub addr: Option<String>,
}

#[derive(o2o)]
#[owned_into(bollard::secret::Mount)]
#[napi(object)]
pub struct Mount {
    /// Container path.
    #[napi(js_name = "Target")]
    pub target: Option<String>,

    /// Mount source (e.g. a volume name, a host path).
    #[napi(js_name = "Source")]
    pub source: Option<String>,

    /// The mount type. Available types:  - `bind` Mounts a file or directory from the host into the container. Must exist prior to creating the container. - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed. - `tmpfs` Create a tmpfs with the given options. The mount source cannot be specified for tmpfs. - `npipe` Mounts a named pipe from the host into the container. Must exist prior to creating the container. - `cluster` a Swarm cluster volume
    #[napi(js_name = "Type")]
    #[map(~.map(Into::into))]
    pub typ: Option<MountTypeEnum>,

    /// Whether the mount should be read-only.
    #[napi(js_name = "ReadOnly")]
    pub read_only: Option<bool>,

    /// The consistency requirement for the mount: `default`, `consistent`, `cached`, or `delegated`.
    #[napi(js_name = "Consistency")]
    pub consistency: Option<String>,

    #[napi(js_name = "BindOptions")]
    #[map(~.map(Into::into))]
    pub bind_options: Option<MountBindOptions>,

    #[napi(js_name = "VolumeOptions")]
    #[map(~.map(Into::into))]
    pub volume_options: Option<MountVolumeOptions>,

    #[napi(js_name = "TmpfsOptions")]
    #[map(~.map(Into::into))]
    pub tmpfs_options: Option<MountTmpfsOptions>,
}

#[derive(o2o)]
#[owned_into(bollard::secret::MountTypeEnum)]
#[allow(non_camel_case_types)]
#[napi]
pub enum MountTypeEnum {
    EMPTY,
    BIND,
    VOLUME,
    TMPFS,
    NPIPE,
    CLUSTER,
}

/// Optional configuration for the `bind` type.
#[derive(o2o)]
#[owned_into(bollard::secret::MountBindOptions)]
#[napi(object)]
pub struct MountBindOptions {
    /// A propagation mode with the value `[r]private`, `[r]shared`, or `[r]slave`.
    #[napi(js_name = "Propagation")]
    #[map(~.map(Into::into))]
    pub propagation: Option<MountBindOptionsPropagationEnum>,

    /// Disable recursive bind mount.
    #[napi(js_name = "NonRecursive")]
    pub non_recursive: Option<bool>,

    /// Create mount point on host if missing
    #[napi(js_name = "CreateMountpoint")]
    pub create_mountpoint: Option<bool>,

    /// Make the mount non-recursively read-only, but still leave the mount recursive (unless NonRecursive is set to `true` in conjunction).  Added in v1.44, before that version all read-only mounts were non-recursive by default. To match the previous behaviour this will default to `true` for clients on versions prior to v1.44.
    #[napi(js_name = "ReadOnlyNonRecursive")]
    pub read_only_non_recursive: Option<bool>,

    /// Raise an error if the mount cannot be made recursively read-only.
    #[napi(js_name = "ReadOnlyForceRecursive")]
    pub read_only_force_recursive: Option<bool>,
}

#[derive(o2o)]
#[owned_into(bollard::secret::MountBindOptionsPropagationEnum)]
#[allow(non_camel_case_types)]
#[napi]
pub enum MountBindOptionsPropagationEnum {
    EMPTY,
    PRIVATE,
    RPRIVATE,
    SHARED,
    RSHARED,
    SLAVE,
    RSLAVE,
}

/// MountPoint represents a mount point configuration inside the container. This is used for reporting the mountpoints in use by a container.
#[napi(object)]
pub struct MountPoint {
    /// The mount type:  - `bind` a mount of a file or directory from the host into the container. - `volume` a docker volume with the given `Name`. - `tmpfs` a `tmpfs`. - `npipe` a named pipe from the host into the container. - `cluster` a Swarm cluster volume
    #[napi(js_name = "Type")]
    pub typ: Option<MountPointTypeEnum>,

    /// Name is the name reference to the underlying data defined by `Source` e.g., the volume name.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// Source location of the mount.  For volumes, this contains the storage location of the volume (within `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains the source (host) part of the bind-mount. For `tmpfs` mount points, this field is empty.
    #[napi(js_name = "Source")]
    pub source: Option<String>,

    /// Destination is the path relative to the container root (`/`) where the `Source` is mounted inside the container.
    #[napi(js_name = "Destination")]
    pub destination: Option<String>,

    /// Driver is the volume driver used to create the volume (if it is a volume).
    #[napi(js_name = "Driver")]
    pub driver: Option<String>,

    /// Mode is a comma separated list of options supplied by the user when creating the bind/volume mount.  The default is platform-specific (`'z'` on Linux, empty on Windows).
    #[napi(js_name = "Mode")]
    pub mode: Option<String>,

    /// Whether the mount is mounted writable (read-write).
    #[napi(js_name = "RW")]
    pub rw: Option<bool>,

    /// Propagation describes how mounts are propagated from the host into the mount point, and vice-versa. Refer to the [Linux kernel documentation](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt) for details. This field is not used on Windows.
    #[napi(js_name = "Propagation")]
    pub propagation: Option<String>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum MountPointTypeEnum {
    EMPTY,
    BIND,
    VOLUME,
    TMPFS,
    NPIPE,
    CLUSTER,
}

/// Optional configuration for the `tmpfs` type.
#[derive(o2o)]
#[owned_into(bollard::secret::MountTmpfsOptions)]
#[napi(object)]
pub struct MountTmpfsOptions {
    /// The size for the tmpfs mount in bytes.
    #[napi(js_name = "SizeBytes")]
    pub size_bytes: Option<i64>,

    /// The permission mode for the tmpfs mount in an integer.
    #[napi(js_name = "Mode")]
    pub mode: Option<i64>,

    /// The options to be passed to the tmpfs mount. An array of arrays. Flag options should be provided as 1-length arrays. Other types should be provided as as 2-length arrays, where the first item is the key and the second the value.
    #[napi(js_name = "Options")]
    pub options: Option<Vec<Vec<String>>>,
}

/// Optional configuration for the `volume` type.
#[derive(o2o)]
#[owned_into(bollard::secret::MountVolumeOptions)]
#[napi(object)]
pub struct MountVolumeOptions {
    /// Populate volume with data from the target.
    #[napi(js_name = "NoCopy")]
    pub no_copy: Option<bool>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    #[napi(js_name = "DriverConfig")]
    #[map(~.map(Into::into))]
    pub driver_config: Option<MountVolumeOptionsDriverConfig>,

    /// Source path inside the volume. Must be relative without any back traversals.
    #[napi(js_name = "Subpath")]
    pub subpath: Option<String>,
}

/// Map of driver specific options
#[derive(o2o)]
#[owned_into(bollard::secret::MountVolumeOptionsDriverConfig)]
#[napi(object)]
pub struct MountVolumeOptionsDriverConfig {
    /// Name of the driver to use to create the volume.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// key/value map of driver specific options.
    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,
}

#[napi(object)]
pub struct Network {
    /// Name of the network.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// ID that uniquely identifies a network on a single machine.
    #[napi(js_name = "Id")]
    pub id: Option<String>,

    /// Date and time at which the network was created in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "Created")]
    pub created: Option<String>,

    /// The level at which the network exists (e.g. `swarm` for cluster-wide or `local` for machine level)
    #[napi(js_name = "Scope")]
    pub scope: Option<String>,

    /// The name of the driver used to create the network (e.g. `bridge`, `overlay`).
    #[napi(js_name = "Driver")]
    pub driver: Option<String>,

    /// Whether the network was created with IPv4 enabled.
    #[napi(js_name = "EnableIPv4")]
    pub enable_ipv4: Option<bool>,

    /// Whether the network was created with IPv6 enabled.
    #[napi(js_name = "EnableIPv6")]
    pub enable_ipv6: Option<bool>,

    #[napi(js_name = "IPAM")]
    pub ipam: Option<Ipam>,

    /// Whether the network is created to only allow internal networking connectivity.
    #[napi(js_name = "Internal")]
    pub internal: Option<bool>,

    /// Whether a global / swarm scope network is manually attachable by regular containers from workers in swarm mode.
    #[napi(js_name = "Attachable")]
    pub attachable: Option<bool>,

    /// Whether the network is providing the routing-mesh for the swarm cluster.
    #[napi(js_name = "Ingress")]
    pub ingress: Option<bool>,

    #[napi(js_name = "ConfigFrom")]
    pub config_from: Option<ConfigReference>,

    /// Whether the network is a config-only network. Config-only networks are placeholder networks for network configurations to be used by other networks. Config-only networks cannot be used directly to run containers or services.
    #[napi(js_name = "ConfigOnly")]
    pub config_only: Option<bool>,

    /// Contains endpoints attached to the network.
    #[napi(js_name = "Containers")]
    pub containers: Option<HashMap<String, NetworkContainer>>,

    /// Network-specific options uses when creating the network.
    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// List of peer nodes for an overlay network. This field is only present for overlay networks, and omitted for other network types.
    #[napi(js_name = "Peers")]
    pub peers: Option<Vec<PeerInfo>>,
}

/// Specifies how a service should be attached to a particular network.
#[napi(object)]
pub struct NetworkAttachmentConfig {
    /// The target network for attachment. Must be a network name or ID.
    #[napi(js_name = "Target")]
    pub target: Option<String>,

    /// Discoverable alternate names for the service on this network.
    #[napi(js_name = "Aliases")]
    pub aliases: Option<Vec<String>>,

    /// Driver attachment options for the network target.
    #[napi(js_name = "DriverOpts")]
    pub driver_opts: Option<HashMap<String, String>>,
}

#[napi(object)]
pub struct NetworkConnectRequest {
    /// The ID or name of the container to connect to the network.
    #[napi(js_name = "Container")]
    pub container: Option<String>,

    #[napi(js_name = "EndpointConfig")]
    pub endpoint_config: Option<EndpointSettings>,
}

#[napi(object)]
pub struct NetworkContainer {
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    #[napi(js_name = "EndpointID")]
    pub endpoint_id: Option<String>,

    #[napi(js_name = "MacAddress")]
    pub mac_address: Option<String>,

    #[napi(js_name = "IPv4Address")]
    pub ipv4_address: Option<String>,

    #[napi(js_name = "IPv6Address")]
    pub ipv6_address: Option<String>,
}

#[napi(object)]
pub struct NetworkCreateRequest {
    /// The network's name.
    #[napi(js_name = "Name")]
    pub name: String,

    /// Name of the network driver plugin to use.
    #[napi(js_name = "Driver")]
    pub driver: Option<String>,

    /// The level at which the network exists (e.g. `swarm` for cluster-wide or `local` for machine level).
    #[napi(js_name = "Scope")]
    pub scope: Option<String>,

    /// Restrict external access to the network.
    #[napi(js_name = "Internal")]
    pub internal: Option<bool>,

    /// Globally scoped network is manually attachable by regular containers from workers in swarm mode.
    #[napi(js_name = "Attachable")]
    pub attachable: Option<bool>,

    /// Ingress network is the network which provides the routing-mesh in swarm mode.
    #[napi(js_name = "Ingress")]
    pub ingress: Option<bool>,

    /// Creates a config-only network. Config-only networks are placeholder networks for network configurations to be used by other networks. Config-only networks cannot be used directly to run containers or services.
    #[napi(js_name = "ConfigOnly")]
    pub config_only: Option<bool>,

    /// Specifies the source which will provide the configuration for this network. The specified network must be an existing config-only network; see ConfigOnly.
    #[napi(js_name = "ConfigFrom")]
    pub config_from: Option<ConfigReference>,

    /// Optional custom IP scheme for the network.
    #[napi(js_name = "IPAM")]
    pub ipam: Option<Ipam>,

    /// Enable IPv4 on the network. To disable IPv4, the daemon must be started with experimental features enabled.
    #[napi(js_name = "EnableIPv4")]
    pub enable_ipv4: Option<bool>,

    /// Enable IPv6 on the network.
    #[napi(js_name = "EnableIPv6")]
    pub enable_ipv6: Option<bool>,

    /// Network specific options to be used by the drivers.
    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,
}

/// OK response to NetworkCreate operation
#[napi(object)]
pub struct NetworkCreateResponse {
    /// The ID of the created network.
    #[napi(js_name = "Id")]
    pub id: String,

    /// Warnings encountered when creating the container
    #[napi(js_name = "Warning")]
    pub warning: String,
}

#[napi(object)]
pub struct NetworkDisconnectRequest {
    /// The ID or name of the container to disconnect from the network.
    #[napi(js_name = "Container")]
    pub container: Option<String>,

    /// Force the container to disconnect from the network.
    #[napi(js_name = "Force")]
    pub force: Option<bool>,
}

#[napi(object)]
pub struct NetworkPruneResponse {
    /// Networks that were deleted
    #[napi(js_name = "NetworksDeleted")]
    pub networks_deleted: Option<Vec<String>>,
}

/// NetworkSettings exposes the network settings in the API
#[napi(object)]
pub struct NetworkSettings {
    /// Name of the default bridge interface when dockerd's --bridge flag is set.
    #[napi(js_name = "Bridge")]
    pub bridge: Option<String>,

    /// SandboxID uniquely represents a container's network stack.
    #[napi(js_name = "SandboxID")]
    pub sandbox_id: Option<String>,

    /// Indicates if hairpin NAT should be enabled on the virtual interface.  Deprecated: This field is never set and will be removed in a future release.
    #[napi(js_name = "HairpinMode")]
    pub hairpin_mode: Option<bool>,

    /// IPv6 unicast address using the link-local prefix.  Deprecated: This field is never set and will be removed in a future release.
    #[napi(js_name = "LinkLocalIPv6Address")]
    pub link_local_ipv6_address: Option<String>,

    /// Prefix length of the IPv6 unicast address.  Deprecated: This field is never set and will be removed in a future release.
    #[napi(js_name = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6_prefix_len: Option<i64>,

    #[napi(js_name = "Ports")]
    pub ports: Option<HashMap<String, Option<Vec<PortBinding>>>>,

    /// SandboxKey is the full path of the netns handle
    #[napi(js_name = "SandboxKey")]
    pub sandbox_key: Option<String>,

    /// Deprecated: This field is never set and will be removed in a future release.
    #[napi(js_name = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Option<Vec<Address>>,

    /// Deprecated: This field is never set and will be removed in a future release.
    #[napi(js_name = "SecondaryIPv6Addresses")]
    pub secondary_ipv6_addresses: Option<Vec<Address>>,

    /// EndpointID uniquely represents a service endpoint in a Sandbox.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default 'bridge' network. Use the information from the 'bridge' > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[napi(js_name = "EndpointID")]
    pub endpoint_id: Option<String>,

    /// Gateway address for the default 'bridge' network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default 'bridge' network. Use the information from the 'bridge' > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[napi(js_name = "Gateway")]
    pub gateway: Option<String>,

    /// Global IPv6 address for the default 'bridge' network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default 'bridge' network. Use the information from the 'bridge' > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[napi(js_name = "GlobalIPv6Address")]
    pub global_ipv6_address: Option<String>,

    /// Mask length of the global IPv6 address.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default 'bridge' network. Use the information from the 'bridge' > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[napi(js_name = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: Option<i64>,

    /// IPv4 address for the default 'bridge' network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default 'bridge' network. Use the information from the 'bridge' > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[napi(js_name = "IPAddress")]
    pub ip_address: Option<String>,

    /// Mask length of the IPv4 address.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default 'bridge' network. Use the information from the 'bridge' > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[napi(js_name = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,

    /// IPv6 gateway address for this network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default 'bridge' network. Use the information from the 'bridge' > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[napi(js_name = "IPv6Gateway")]
    pub ipv6_gateway: Option<String>,

    /// MAC address for the container on the default 'bridge' network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default 'bridge' network. Use the information from the 'bridge' > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[napi(js_name = "MacAddress")]
    pub mac_address: Option<String>,

    /// Information about all networks that the container is connected to.
    #[napi(js_name = "Networks")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
}

/// NetworkingConfig represents the container's networking configuration for each of its interfaces. It is used for the networking configs specified in the `docker create` and `docker network connect` commands.
#[napi(object)]
pub struct NetworkingConfig {
    /// A mapping of network name to endpoint configuration for that network. The endpoint configuration can be left empty to connect to that network with no particular endpoint configuration.
    #[napi(js_name = "EndpointsConfig")]
    pub endpoints_config: Option<HashMap<String, EndpointSettings>>,
}

#[napi(object)]
pub struct Node {
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Version")]
    pub version: Option<ObjectVersion>,

    /// Date and time at which the node was added to the swarm in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    /// Date and time at which the node was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "UpdatedAt")]
    pub updated_at: Option<String>,

    #[napi(js_name = "Spec")]
    pub spec: Option<NodeSpec>,

    #[napi(js_name = "Description")]
    pub description: Option<NodeDescription>,

    #[napi(js_name = "Status")]
    pub status: Option<NodeStatus>,

    #[napi(js_name = "ManagerStatus")]
    pub manager_status: Option<ManagerStatus>,
}

/// NodeDescription encapsulates the properties of the Node as reported by the agent.
#[napi(object)]
pub struct NodeDescription {
    #[napi(js_name = "Hostname")]
    pub hostname: Option<String>,

    #[napi(js_name = "Platform")]
    pub platform: Option<Platform>,

    #[napi(js_name = "Resources")]
    pub resources: Option<ResourceObject>,

    #[napi(js_name = "Engine")]
    pub engine: Option<EngineDescription>,

    #[napi(js_name = "TLSInfo")]
    pub tls_info: Option<TlsInfo>,
}

#[napi(object)]
pub struct NodeSpec {
    /// Name for the node.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// Role of the node.
    #[napi(js_name = "Role")]
    pub role: Option<NodeSpecRoleEnum>,

    /// Availability of the node.
    #[napi(js_name = "Availability")]
    pub availability: Option<NodeSpecAvailabilityEnum>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum NodeSpecRoleEnum {
    EMPTY,
    WORKER,
    MANAGER,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum NodeSpecAvailabilityEnum {
    EMPTY,
    ACTIVE,
    PAUSE,
    DRAIN,
}

/// which helps with FFI.
#[allow(non_camel_case_types)]
#[napi]
pub enum NodeState {
    UNKNOWN,
    DOWN,
    READY,
    DISCONNECTED,
}

/// NodeStatus represents the status of a node.  It provides the current status of the node, as seen by the manager.
#[napi(object)]
pub struct NodeStatus {
    #[napi(js_name = "State")]
    pub state: Option<NodeState>,

    #[napi(js_name = "Message")]
    pub message: Option<String>,

    /// IP address of the node.
    #[napi(js_name = "Addr")]
    pub addr: Option<String>,
}

/// The version number of the object such as node, service, etc. This is needed to avoid conflicting writes. The client must send the version number along with the modified specification when updating these objects.  This approach ensures safe concurrency and determinism in that the change on the object may not be applied if the version number has changed from the last read. In other words, if two update requests specify the same base version, only one of the requests can succeed. As a result, two separate update requests that happen at the same time will not unintentionally overwrite each other.
#[napi(object)]
pub struct ObjectVersion {
    #[napi(js_name = "Index")]
    pub index: Option<u32>,
}

/// A descriptor struct containing digest, media type, and size, as defined in the [OCI Content Descriptors Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/descriptor.md).
#[napi(object)]
pub struct OciDescriptor {
    /// The media type of the object this schema refers to.
    #[napi(js_name = "mediaType")]
    pub media_type: Option<String>,

    /// The digest of the targeted content.
    #[napi(js_name = "digest")]
    pub digest: Option<String>,

    /// The size in bytes of the blob.
    #[napi(js_name = "size")]
    pub size: Option<i64>,
}

/// Describes the platform which the image in the manifest runs on, as defined in the [OCI Image Index Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/image-index.md).
#[napi(object)]
pub struct OciPlatform {
    /// The CPU architecture, for example `amd64` or `ppc64`.
    #[napi(js_name = "architecture")]
    pub architecture: Option<String>,

    /// The operating system, for example `linux` or `windows`.
    pub os: Option<String>,

    /// Optional field specifying the operating system version, for example on Windows `10.0.19041.1165`.
    pub os_version: Option<String>,

    /// Optional field specifying an array of strings, each listing a required OS feature (for example on Windows `win32k`).
    pub os_features: Option<Vec<String>>,

    /// Optional field specifying a variant of the CPU, for example `v7` to specify ARMv7 when architecture is `arm`.
    #[napi(js_name = "variant")]
    pub variant: Option<String>,
}

/// PeerInfo represents one peer of an overlay network.
#[napi(object)]
pub struct PeerInfo {
    /// ID of the peer-node in the Swarm cluster.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// IP-address of the peer-node in the Swarm cluster.
    #[napi(js_name = "IP")]
    pub ip: Option<String>,
}

/// Represents a peer-node in the swarm
#[napi(object)]
pub struct PeerNode {
    /// Unique identifier of for this node in the swarm.
    #[napi(js_name = "NodeID")]
    pub node_id: Option<String>,

    /// IP address and ports at which this node can be reached.
    #[napi(js_name = "Addr")]
    pub addr: Option<String>,
}

/// Platform represents the platform (Arch/OS).
#[napi(object)]
pub struct Platform {
    /// Architecture represents the hardware architecture (for example, `x86_64`).
    #[napi(js_name = "Architecture")]
    pub architecture: Option<String>,

    /// OS represents the Operating System (for example, `linux` or `windows`).
    #[napi(js_name = "OS")]
    pub os: Option<String>,
}

/// A plugin for the Engine API
#[napi(object)]
pub struct Plugin {
    #[napi(js_name = "Id")]
    pub id: Option<String>,

    #[napi(js_name = "Name")]
    pub name: String,

    /// True if the plugin is running. False if the plugin is not running, only installed.
    #[napi(js_name = "Enabled")]
    pub enabled: bool,

    #[napi(js_name = "Settings")]
    pub settings: PluginSettings,

    /// plugin remote reference used to push/pull the plugin
    #[napi(js_name = "PluginReference")]
    pub plugin_reference: Option<String>,

    #[napi(js_name = "Config")]
    pub config: PluginConfig,
}

/// The config of a plugin.
#[napi(object)]
pub struct PluginConfig {
    /// Docker Version used to create the plugin
    #[napi(js_name = "DockerVersion")]
    pub docker_version: Option<String>,

    #[napi(js_name = "Description")]
    pub description: String,

    #[napi(js_name = "Documentation")]
    pub documentation: String,

    #[napi(js_name = "Interface")]
    pub interface: PluginConfigInterface,

    #[napi(js_name = "Entrypoint")]
    pub entrypoint: Vec<String>,

    #[napi(js_name = "WorkDir")]
    pub work_dir: String,

    #[napi(js_name = "User")]
    pub user: Option<PluginConfigUser>,

    #[napi(js_name = "Network")]
    pub network: PluginConfigNetwork,

    #[napi(js_name = "Linux")]
    pub linux: PluginConfigLinux,

    #[napi(js_name = "PropagatedMount")]
    pub propagated_mount: String,

    #[napi(js_name = "IpcHost")]
    pub ipc_host: bool,

    #[napi(js_name = "PidHost")]
    pub pid_host: bool,

    #[napi(js_name = "Mounts")]
    pub mounts: Vec<PluginMount>,

    #[napi(js_name = "Env")]
    pub env: Vec<PluginEnv>,

    #[napi(js_name = "Args")]
    pub args: PluginConfigArgs,

    #[napi(js_name = "rootfs")]
    pub rootfs: Option<PluginConfigRootfs>,
}

#[napi(object)]
pub struct PluginConfigArgs {
    #[napi(js_name = "Name")]
    pub name: String,

    #[napi(js_name = "Description")]
    pub description: String,

    #[napi(js_name = "Settable")]
    pub settable: Vec<String>,

    #[napi(js_name = "Value")]
    pub value: Vec<String>,
}

/// The interface between Docker and the plugin
#[napi(object)]
pub struct PluginConfigInterface {
    #[napi(js_name = "Types")]
    pub types: Vec<PluginInterfaceType>,

    #[napi(js_name = "Socket")]
    pub socket: String,

    /// Protocol to use for clients connecting to the plugin.
    #[napi(js_name = "ProtocolScheme")]
    pub protocol_scheme: Option<PluginConfigInterfaceProtocolSchemeEnum>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum PluginConfigInterfaceProtocolSchemeEnum {
    EMPTY,
    MOBY_PLUGINS_HTTP_V1,
}

#[napi(object)]
pub struct PluginConfigLinux {
    #[napi(js_name = "Capabilities")]
    pub capabilities: Vec<String>,

    #[napi(js_name = "AllowAllDevices")]
    pub allow_all_devices: bool,

    #[napi(js_name = "Devices")]
    pub devices: Vec<PluginDevice>,
}

#[napi(object)]
pub struct PluginConfigNetwork {
    #[napi(js_name = "Type")]
    pub typ: String,
}

#[napi(object)]
pub struct PluginConfigRootfs {
    #[napi(js_name = "type")]
    pub typ: Option<String>,

    #[napi(js_name = "diff_ids")]
    pub diff_ids: Option<Vec<String>>,
}

#[napi(object)]
pub struct PluginConfigUser {
    #[napi(js_name = "UID")]
    pub uid: Option<u32>,

    #[napi(js_name = "GID")]
    pub gid: Option<u32>,
}

#[napi(object)]
pub struct PluginDevice {
    #[napi(js_name = "Name")]
    pub name: String,

    #[napi(js_name = "Description")]
    pub description: String,

    #[napi(js_name = "Settable")]
    pub settable: Vec<String>,

    #[napi(js_name = "Path")]
    pub path: String,
}

#[napi(object)]
pub struct PluginEnv {
    #[napi(js_name = "Name")]
    pub name: String,

    #[napi(js_name = "Description")]
    pub description: String,

    #[napi(js_name = "Settable")]
    pub settable: Vec<String>,

    #[napi(js_name = "Value")]
    pub value: String,
}

#[napi(object)]
pub struct PluginInterfaceType {
    #[napi(js_name = "Prefix")]
    pub prefix: String,

    #[napi(js_name = "Capability")]
    pub capability: String,

    #[napi(js_name = "Version")]
    pub version: String,
}

#[napi(object)]
pub struct PluginMount {
    #[napi(js_name = "Name")]
    pub name: String,

    #[napi(js_name = "Description")]
    pub description: String,

    #[napi(js_name = "Settable")]
    pub settable: Vec<String>,

    #[napi(js_name = "Source")]
    pub source: String,

    #[napi(js_name = "Destination")]
    pub destination: String,

    #[napi(js_name = "Type")]
    pub typ: String,

    #[napi(js_name = "Options")]
    pub options: Vec<String>,
}

/// Describes a permission the user has to accept upon installing the plugin.
#[napi(object)]
pub struct PluginPrivilege {
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    #[napi(js_name = "Description")]
    pub description: Option<String>,

    #[napi(js_name = "Value")]
    pub value: Option<Vec<String>>,
}

/// Settings that can be modified by users.
#[napi(object)]
pub struct PluginSettings {
    #[napi(js_name = "Mounts")]
    pub mounts: Vec<PluginMount>,

    #[napi(js_name = "Env")]
    pub env: Vec<String>,

    #[napi(js_name = "Args")]
    pub args: Vec<String>,

    #[napi(js_name = "Devices")]
    pub devices: Vec<PluginDevice>,
}

/// Available plugins per type.  <p><br /></p>  > **Note**: Only unmanaged (V1) plugins are included in this list. > V1 plugins are 'lazily' loaded, and are not returned in this list > if there is no resource using the plugin.
#[napi(object)]
pub struct PluginsInfo {
    /// Names of available volume-drivers, and network-driver plugins.
    #[napi(js_name = "Volume")]
    pub volume: Option<Vec<String>>,

    /// Names of available network-drivers, and network-driver plugins.
    #[napi(js_name = "Network")]
    pub network: Option<Vec<String>>,

    /// Names of available authorization plugins.
    #[napi(js_name = "Authorization")]
    pub authorization: Option<Vec<String>>,

    /// Names of available logging-drivers, and logging-driver plugins.
    #[napi(js_name = "Log")]
    pub log: Option<Vec<String>>,
}

/// An open port on a container
#[napi(object)]
pub struct Port {
    /// Host IP address that the container's port is mapped to
    #[napi(js_name = "IP")]
    pub ip: Option<String>,

    /// Port on the container
    #[napi(js_name = "PrivatePort")]
    pub private_port: u16,

    /// Port exposed on the host
    #[napi(js_name = "PublicPort")]
    pub public_port: Option<u16>,

    #[napi(js_name = "Type")]
    pub typ: Option<PortTypeEnum>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum PortTypeEnum {
    EMPTY,
    TCP,
    UDP,
    SCTP,
}

/// PortBinding represents a binding between a host IP address and a host port.
#[napi(object)]
#[derive(o2o)]
#[owned_into(bollard::secret::PortBinding)]
pub struct PortBinding {
    /// Host IP address that the container's port is mapped to.
    #[napi(js_name = "HostIp")]
    pub host_ip: Option<String>,

    /// Host port number that the container's port is mapped to.
    #[napi(js_name = "HostPort")]
    pub host_port: Option<String>,
}

fn covert_port_map(
    port_map: HashMap<String, Option<Vec<PortBinding>>>,
) -> HashMap<String, Option<Vec<bollard::secret::PortBinding>>> {
    port_map
        .into_iter()
        .map(|(k, v)| (k, v.map(|v| v.into_iter().map(Into::into).collect())))
        .collect()
}

/// represents the port status of a task's host ports whose service has published host ports
#[napi(object)]
pub struct PortStatus {
    #[napi(js_name = "Ports")]
    pub ports: Option<Vec<EndpointPortConfig>>,
}

#[napi(object)]
pub struct ProcessConfig {
    #[napi(js_name = "privileged")]
    pub privileged: Option<bool>,

    #[napi(js_name = "user")]
    pub user: Option<String>,

    #[napi(js_name = "tty")]
    pub tty: Option<bool>,

    #[napi(js_name = "entrypoint")]
    pub entrypoint: Option<String>,

    #[napi(js_name = "arguments")]
    pub arguments: Option<Vec<String>>,
}

#[napi(object)]
pub struct ProgressDetail {
    #[napi(js_name = "current")]
    pub current: Option<i64>,

    #[napi(js_name = "total")]
    pub total: Option<i64>,
}

#[napi(object)]
pub struct PushImageInfo {
    #[napi(js_name = "error")]
    pub error: Option<String>,

    #[napi(js_name = "status")]
    pub status: Option<String>,

    #[napi(js_name = "progress")]
    pub progress: Option<String>,

    #[napi(js_name = "progressDetail")]
    pub progress_detail: Option<ProgressDetail>,
}

/// Reachability represents the reachability of a node.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[napi]
pub enum Reachability {
    UNKNOWN,
    UNREACHABLE,
    REACHABLE,
}

/// RegistryServiceConfig stores daemon registry services configuration.
#[napi(object)]
pub struct RegistryServiceConfig {
    /// List of IP ranges to which nondistributable artifacts can be pushed, using the CIDR syntax [RFC 4632](https://tools.ietf.org/html/4632).  Some images (for example, Windows base images) contain artifacts whose distribution is restricted by license. When these images are pushed to a registry, restricted artifacts are not included.  This configuration override this behavior, and enables the daemon to push nondistributable artifacts to all registries whose resolved IP address is within the subnet described by the CIDR syntax.  This option is useful when pushing images containing nondistributable artifacts to a registry on an air-gapped network so hosts on that network can pull the images without connecting to another server.  > **Warning**: Nondistributable artifacts typically have restrictions > on how and where they can be distributed and shared. Only use this > feature to push artifacts to private registries and ensure that you > are in compliance with any terms that cover redistributing > nondistributable artifacts.
    #[napi(js_name = "AllowNondistributableArtifactsCIDRs")]
    pub allow_nondistributable_artifacts_cidrs: Option<Vec<String>>,

    /// List of registry hostnames to which nondistributable artifacts can be pushed, using the format `<hostname>[:<port>]` or `<IP address>[:<port>]`.  Some images (for example, Windows base images) contain artifacts whose distribution is restricted by license. When these images are pushed to a registry, restricted artifacts are not included.  This configuration override this behavior for the specified registries.  This option is useful when pushing images containing nondistributable artifacts to a registry on an air-gapped network so hosts on that network can pull the images without connecting to another server.  > **Warning**: Nondistributable artifacts typically have restrictions > on how and where they can be distributed and shared. Only use this > feature to push artifacts to private registries and ensure that you > are in compliance with any terms that cover redistributing > nondistributable artifacts.
    #[napi(js_name = "AllowNondistributableArtifactsHostnames")]
    pub allow_nondistributable_artifacts_hostnames: Option<Vec<String>>,

    /// List of IP ranges of insecure registries, using the CIDR syntax ([RFC 4632](https://tools.ietf.org/html/4632)). Insecure registries accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from unknown CAs) communication.  By default, local registries (`127.0.0.0/8`) are configured as insecure. All other registries are secure. Communicating with an insecure registry is not possible if the daemon assumes that registry is secure.  This configuration override this behavior, insecure communication with registries whose resolved IP address is within the subnet described by the CIDR syntax.  Registries can also be marked insecure by hostname. Those registries are listed under `IndexConfigs` and have their `Secure` field set to `false`.  > **Warning**: Using this option can be useful when running a local > registry, but introduces security vulnerabilities. This option > should therefore ONLY be used for testing purposes. For increased > security, users should add their CA to their system's list of trusted > CAs instead of enabling this option.
    #[napi(js_name = "InsecureRegistryCIDRs")]
    pub insecure_registry_cidrs: Option<Vec<String>>,

    #[napi(js_name = "IndexConfigs")]
    pub index_configs: Option<HashMap<String, IndexInfo>>,

    /// List of registry URLs that act as a mirror for the official (`docker.io`) registry.
    #[napi(js_name = "Mirrors")]
    pub mirrors: Option<Vec<String>>,
}

/// An object describing the resources which can be advertised by a node and requested by a task.
#[napi(object)]
pub struct ResourceObject {
    #[napi(js_name = "NanoCPUs")]
    pub nano_cpus: Option<i64>,

    #[napi(js_name = "MemoryBytes")]
    pub memory_bytes: Option<i64>,

    #[napi(js_name = "GenericResources")]
    pub generic_resources: Option<GenericResourcesInner>,
}

/// A container's resources (cgroups config, ulimits, etc)
#[napi(object)]
pub struct Resources {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[napi(js_name = "CpuShares")]
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

    /// Block IO weight (relative device weight) in the form:  ``` [{'Path': 'device_path', 'Weight': weight}] ```
    #[napi(js_name = "BlkioWeightDevice")]
    pub blkio_weight_device: Option<Vec<ResourcesBlkioWeightDevice>>,

    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{'Path': 'device_path', 'Rate': rate}] ```
    #[napi(js_name = "BlkioDeviceReadBps")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{'Path': 'device_path', 'Rate': rate}] ```
    #[napi(js_name = "BlkioDeviceWriteBps")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,

    /// Limit read rate (IO per second) from a device, in the form:  ``` [{'Path': 'device_path', 'Rate': rate}] ```
    #[napi(js_name = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (IO per second) to a device, in the form:  ``` [{'Path': 'device_path', 'Rate': rate}] ```
    #[napi(js_name = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Option<Vec<ThrottleDevice>>,

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

    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[napi(js_name = "CpusetCpus")]
    pub cpuset_cpus: Option<String>,

    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[napi(js_name = "CpusetMems")]
    pub cpuset_mems: Option<String>,

    /// A list of devices to add to the container.
    #[napi(js_name = "Devices")]
    pub devices: Option<Vec<DeviceMapping>>,

    /// a list of cgroup rules to apply to the container
    #[napi(js_name = "DeviceCgroupRules")]
    pub device_cgroup_rules: Option<Vec<String>>,

    /// A list of requests for devices to be sent to device drivers.
    #[napi(js_name = "DeviceRequests")]
    pub device_requests: Option<Vec<DeviceRequest>>,

    /// Hard limit for kernel TCP buffer memory (in bytes). Depending on the OCI runtime in use, this option may be ignored. It is no longer supported by the default (runc) runtime.  This field is omitted when empty.
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

    /// A list of resource limits to set in the container. For example:  ``` {'Name': 'nofile', 'Soft': 1024, 'Hard': 2048} ```
    #[napi(js_name = "Ulimits")]
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

    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[napi(js_name = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<i64>,
}

#[derive(o2o)]
#[owned_into(bollard::secret::ResourcesBlkioWeightDevice)]
#[napi(object)]
pub struct ResourcesBlkioWeightDevice {
    #[napi(js_name = "Path")]
    pub path: Option<String>,

    #[napi(js_name = "Weight")]
    #[map(~.map(|o| o as usize))]
    pub weight: Option<u32>,
}

#[derive(o2o)]
#[owned_into(bollard::secret::ResourcesUlimits)]
#[napi(object)]
pub struct ResourcesUlimits {
    /// Name of ulimit
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// Soft limit
    #[napi(js_name = "Soft")]
    pub soft: Option<i64>,

    /// Hard limit
    #[napi(js_name = "Hard")]
    pub hard: Option<i64>,
}

/// The behavior to apply when the container exits. The default is not to restart.  An ever increasing delay (double the previous delay, starting at 100ms) is added before each restart to prevent flooding the server.
#[derive(o2o)]
#[owned_into(bollard::secret::RestartPolicy)]
#[napi(object)]
pub struct RestartPolicy {
    /// - Empty string means not to restart - `no` Do not automatically restart - `always` Always restart - `unless-stopped` Restart always except when the user has manually stopped the container - `on-failure` Restart only when the container exit code is non-zero
    #[napi(js_name = "Name")]
    #[map(~.map(Into::into))]
    pub name: Option<RestartPolicyNameEnum>,

    /// If `on-failure` is used, the number of times to retry before giving up.
    #[napi(js_name = "MaximumRetryCount")]
    pub maximum_retry_count: Option<i64>,
}

#[derive(o2o)]
#[owned_into(bollard::secret::RestartPolicyNameEnum)]
#[allow(non_camel_case_types)]
#[napi]
pub enum RestartPolicyNameEnum {
    EMPTY,
    NO,
    ALWAYS,
    UNLESS_STOPPED,
    ON_FAILURE,
}

/// Runtime describes an [OCI compliant](https://github.com/opencontainers/runtime-spec) runtime.  The runtime is invoked by the daemon via the `containerd` daemon. OCI runtimes act as an interface to the Linux kernel namespaces, cgroups, and SELinux.
#[napi(object)]
pub struct Runtime {
    /// Name and, optional, path, of the OCI executable binary.  If the path is omitted, the daemon searches the host's `$PATH` for the binary and uses the first result.
    #[napi(js_name = "path")]
    pub path: Option<String>,

    /// List of command-line arguments to pass to the runtime when invoked.
    #[napi(js_name = "runtimeArgs")]
    pub runtime_args: Option<Vec<String>>,

    /// Information specific to the runtime.  While this API specification does not define data provided by runtimes, the following well-known properties may be provided by runtimes:  `org.opencontainers.runtime-spec.features`: features structure as defined in the [OCI Runtime Specification](https://github.com/opencontainers/runtime-spec/blob/main/features.md), in a JSON string representation.  <p><br /></p>  > **Note**: The information returned in this field, including the > formatting of values and labels, should not be considered stable, > and may change without notice.
    #[napi(js_name = "status")]
    pub status: Option<HashMap<String, String>>,
}

#[napi(object)]
pub struct Secret {
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Version")]
    pub version: Option<ObjectVersion>,

    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    #[napi(js_name = "UpdatedAt")]
    pub updated_at: Option<String>,

    #[napi(js_name = "Spec")]
    pub spec: Option<SecretSpec>,
}

#[napi(object)]
pub struct SecretSpec {
    /// User-defined name of the secret.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) data to store as secret.  This field is only used to _create_ a secret, and is not returned by other endpoints.
    #[napi(js_name = "Data")]
    pub data: Option<String>,

    /// Name of the secrets driver used to fetch the secret's value from an external secret store.
    #[napi(js_name = "Driver")]
    pub driver: Option<Driver>,

    /// Templating driver, if applicable  Templating controls whether and how to evaluate the config payload as a template. If no driver is set, no templating is used.
    #[napi(js_name = "Templating")]
    pub templating: Option<Driver>,
}

#[napi(object)]
pub struct Service {
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Version")]
    pub version: Option<ObjectVersion>,

    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    pub updated_at: Option<String>,

    #[napi(js_name = "Spec")]
    pub spec: Option<ServiceSpec>,

    #[napi(js_name = "Endpoint")]
    pub endpoint: Option<ServiceEndpoint>,

    #[napi(js_name = "UpdateStatus")]
    pub update_status: Option<ServiceUpdateStatus>,

    #[napi(js_name = "ServiceStatus")]
    pub service_status: Option<ServiceServiceStatus>,

    #[napi(js_name = "JobStatus")]
    pub job_status: Option<ServiceJobStatus>,
}

/// contains the information returned to a client on the creation of a new service.
#[napi(object)]
pub struct ServiceCreateResponse {
    /// The ID of the created service.
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    /// Optional warning message.  FIXME(thaJeztah): this should have 'omitempty' in the generated type.
    #[napi(js_name = "Warnings")]
    pub warnings: Option<Vec<String>>,
}

#[napi(object)]
pub struct ServiceEndpoint {
    #[napi(js_name = "Spec")]
    pub spec: Option<EndpointSpec>,

    #[napi(js_name = "Ports")]
    pub ports: Option<Vec<EndpointPortConfig>>,

    #[napi(js_name = "VirtualIPs")]
    pub virtual_ips: Option<Vec<ServiceEndpointVirtualIps>>,
}

#[napi(object)]
pub struct ServiceEndpointVirtualIps {
    #[napi(js_name = "NetworkID")]
    pub network_id: Option<String>,

    #[napi(js_name = "Addr")]
    pub addr: Option<String>,
}

/// The status of the service when it is in one of ReplicatedJob or GlobalJob modes. Absent on Replicated and Global mode services. The JobIteration is an ObjectVersion, but unlike the Service's version, does not need to be sent with an update request.
#[napi(object)]
pub struct ServiceJobStatus {
    /// JobIteration is a value increased each time a Job is executed, successfully or otherwise. 'Executed', in this case, means the job as a whole has been started, not that an individual Task has been launched. A job is 'Executed' when its ServiceSpec is updated. JobIteration can be used to disambiguate Tasks belonging to different executions of a job.  Though JobIteration will increase with each subsequent execution, it may not necessarily increase by 1, and so JobIteration should not be used to
    #[napi(js_name = "JobIteration")]
    pub job_iteration: Option<ObjectVersion>,

    /// The last time, as observed by the server, that this job was started.
    #[napi(js_name = "LastExecution")]
    pub last_execution: Option<String>,
}

/// The status of the service's tasks. Provided only when requested as part of a ServiceList operation.
#[napi(object)]
pub struct ServiceServiceStatus {
    /// The number of tasks for the service currently in the Running state.
    #[napi(js_name = "RunningTasks")]
    pub running_tasks: Option<u32>,

    /// The number of tasks for the service desired to be running. For replicated services, this is the replica count from the service spec. For global services, this is computed by taking count of all tasks for the service with a Desired State other than Shutdown.
    #[napi(js_name = "DesiredTasks")]
    pub desired_tasks: Option<u32>,

    /// The number of tasks for a job that are in the Completed state. This field must be cross-referenced with the service type, as the value of 0 may mean the service is not in a job mode, or it may mean the job-mode service has no tasks yet Completed.
    #[napi(js_name = "CompletedTasks")]
    pub completed_tasks: Option<u32>,
}

/// User modifiable configuration for a service.
#[napi(object)]
pub struct ServiceSpec {
    /// Name of the service.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    #[napi(js_name = "TaskTemplate")]
    pub task_template: Option<TaskSpec>,

    #[napi(js_name = "Mode")]
    pub mode: Option<ServiceSpecMode>,

    #[napi(js_name = "UpdateConfig")]
    pub update_config: Option<ServiceSpecUpdateConfig>,

    #[napi(js_name = "RollbackConfig")]
    pub rollback_config: Option<ServiceSpecRollbackConfig>,

    /// Specifies which networks the service should attach to.  Deprecated: This field is deprecated since v1.44. The Networks field in TaskSpec should be used instead.
    #[napi(js_name = "Networks")]
    pub networks: Option<Vec<NetworkAttachmentConfig>>,

    #[napi(js_name = "EndpointSpec")]
    pub endpoint_spec: Option<EndpointSpec>,
}

/// Scheduling mode for the service.
#[napi(object)]
pub struct ServiceSpecMode {
    #[napi(js_name = "Replicated")]
    pub replicated: Option<ServiceSpecModeReplicated>,

    #[napi(js_name = "Global")]
    pub global: Option<()>,

    #[napi(js_name = "ReplicatedJob")]
    pub replicated_job: Option<ServiceSpecModeReplicatedJob>,

    /// The mode used for services which run a task to the completed state on each valid node.
    #[napi(js_name = "GlobalJob")]
    pub global_job: Option<()>,
}

#[napi(object)]
pub struct ServiceSpecModeReplicated {
    #[napi(js_name = "Replicas")]
    pub replicas: Option<i64>,
}

/// The mode used for services with a finite number of tasks that run to a completed state.
#[napi(object)]
pub struct ServiceSpecModeReplicatedJob {
    /// The maximum number of replicas to run simultaneously.
    #[napi(js_name = "MaxConcurrent")]
    pub max_concurrent: Option<i64>,

    /// The total number of replicas desired to reach the Completed state. If unset, will default to the value of `MaxConcurrent`
    #[napi(js_name = "TotalCompletions")]
    pub total_completions: Option<i64>,
}

/// Specification for the rollback strategy of the service.
#[napi(object)]
pub struct ServiceSpecRollbackConfig {
    /// Maximum number of tasks to be rolled back in one iteration (0 means unlimited parallelism).
    #[napi(js_name = "Parallelism")]
    pub parallelism: Option<i64>,

    /// Amount of time between rollback iterations, in nanoseconds.
    #[napi(js_name = "Delay")]
    pub delay: Option<i64>,

    /// Action to take if an rolled back task fails to run, or stops running during the rollback.
    #[napi(js_name = "FailureAction")]
    pub failure_action: Option<ServiceSpecRollbackConfigFailureActionEnum>,

    /// Amount of time to monitor each rolled back task for failures, in nanoseconds.
    #[napi(js_name = "Monitor")]
    pub monitor: Option<i64>,

    /// The fraction of tasks that may fail during a rollback before the failure action is invoked, specified as a floating point number between 0 and 1.
    #[napi(js_name = "MaxFailureRatio")]
    pub max_failure_ratio: Option<f64>,

    /// The order of operations when rolling back a task. Either the old task is shut down before the new task is started, or the new task is started before the old task is shut down.
    #[napi(js_name = "Order")]
    pub order: Option<ServiceSpecRollbackConfigOrderEnum>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ServiceSpecRollbackConfigFailureActionEnum {
    EMPTY,
    CONTINUE,
    PAUSE,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ServiceSpecRollbackConfigOrderEnum {
    EMPTY,
    STOP_FIRST,
    START_FIRST,
}

/// Specification for the update strategy of the service.
#[napi(object)]
pub struct ServiceSpecUpdateConfig {
    /// Maximum number of tasks to be updated in one iteration (0 means unlimited parallelism).
    #[napi(js_name = "Parallelism")]
    pub parallelism: Option<i64>,

    /// Amount of time between updates, in nanoseconds.
    #[napi(js_name = "Delay")]
    pub delay: Option<i64>,

    /// Action to take if an updated task fails to run, or stops running during the update.
    #[napi(js_name = "FailureAction")]
    pub failure_action: Option<ServiceSpecUpdateConfigFailureActionEnum>,

    /// Amount of time to monitor each updated task for failures, in nanoseconds.
    #[napi(js_name = "Monitor")]
    pub monitor: Option<i64>,

    /// The fraction of tasks that may fail during an update before the failure action is invoked, specified as a floating point number between 0 and 1.
    #[napi(js_name = "MaxFailureRatio")]
    pub max_failure_ratio: Option<f64>,

    /// The order of operations when rolling out an updated task. Either the old task is shut down before the new task is started, or the new task is started before the old task is shut down.
    #[napi(js_name = "Order")]
    pub order: Option<ServiceSpecUpdateConfigOrderEnum>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ServiceSpecUpdateConfigFailureActionEnum {
    EMPTY,
    CONTINUE,
    PAUSE,
    ROLLBACK,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ServiceSpecUpdateConfigOrderEnum {
    EMPTY,
    STOP_FIRST,
    START_FIRST,
}

#[napi(object)]
pub struct ServiceUpdateResponse {
    /// Optional warning messages
    #[napi(js_name = "Warnings")]
    pub warnings: Option<Vec<String>>,
}

/// The status of a service update.
#[napi(object)]
pub struct ServiceUpdateStatus {
    #[napi(js_name = "State")]
    pub state: Option<ServiceUpdateStatusStateEnum>,

    #[napi(js_name = "StartedAt")]
    pub started_at: Option<String>,

    #[napi(js_name = "CompletedAt")]
    pub completed_at: Option<String>,

    #[napi(js_name = "Message")]
    pub message: Option<String>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum ServiceUpdateStatusStateEnum {
    EMPTY,
    UPDATING,
    PAUSED,
    COMPLETED,
    ROLLBACK_STARTED,
    ROLLBACK_PAUSED,
    ROLLBACK_COMPLETED,
}

#[napi(object)]
pub struct Swarm {
    /// The ID of the swarm.
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Version")]
    pub version: Option<ObjectVersion>,

    /// Date and time at which the swarm was initialised in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    /// Date and time at which the swarm was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "UpdatedAt")]
    pub updated_at: Option<String>,

    #[napi(js_name = "Spec")]
    pub spec: Option<SwarmSpec>,

    #[napi(js_name = "TLSInfo")]
    pub tls_info: Option<TlsInfo>,

    /// Whether there is currently a root CA rotation in progress for the swarm
    #[napi(js_name = "RootRotationInProgress")]
    pub root_rotation_in_progress: Option<bool>,

    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. If no port is set or is set to 0, the default port (4789) is used.
    #[napi(js_name = "DataPathPort")]
    pub data_path_port: Option<u32>,

    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[napi(js_name = "DefaultAddrPool")]
    pub default_addr_pool: Option<Vec<String>>,

    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[napi(js_name = "SubnetSize")]
    pub subnet_size: Option<u32>,

    #[napi(js_name = "JoinTokens")]
    pub join_tokens: Option<JoinTokens>,
}

/// Represents generic information about swarm.
#[napi(object)]
pub struct SwarmInfo {
    /// Unique identifier of for this node in the swarm.
    #[napi(js_name = "NodeID")]
    pub node_id: Option<String>,

    /// IP address at which this node can be reached by other nodes in the swarm.
    #[napi(js_name = "NodeAddr")]
    pub node_addr: Option<String>,

    #[napi(js_name = "LocalNodeState")]
    pub local_node_state: Option<LocalNodeState>,

    #[napi(js_name = "ControlAvailable")]
    pub control_available: Option<bool>,

    #[napi(js_name = "Error")]
    pub error: Option<String>,

    /// List of ID's and addresses of other managers in the swarm.
    #[napi(js_name = "RemoteManagers")]
    pub remote_managers: Option<Vec<PeerNode>>,

    /// Total number of nodes in the swarm.
    #[napi(js_name = "Nodes")]
    pub nodes: Option<i64>,

    /// Total number of managers in the swarm.
    #[napi(js_name = "Managers")]
    pub managers: Option<i64>,

    #[napi(js_name = "Cluster")]
    pub cluster: Option<ClusterInfo>,
}

#[napi(object)]
pub struct SwarmInitRequest {
    /// Listen address used for inter-manager communication, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP). This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the default swarm listening port is used.
    #[napi(js_name = "ListenAddr")]
    pub listen_addr: Option<String>,

    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[napi(js_name = "AdvertiseAddr")]
    pub advertise_addr: Option<String>,

    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same address as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other  nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[napi(js_name = "DataPathAddr")]
    pub data_path_addr: Option<String>,

    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. if no port is set or is set to 0, default port 4789 will be used.
    #[napi(js_name = "DataPathPort")]
    pub data_path_port: Option<u32>,

    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[napi(js_name = "DefaultAddrPool")]
    pub default_addr_pool: Option<Vec<String>>,

    /// Force creation of a new swarm.
    #[napi(js_name = "ForceNewCluster")]
    pub force_new_cluster: Option<bool>,

    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[napi(js_name = "SubnetSize")]
    pub subnet_size: Option<u32>,

    #[napi(js_name = "Spec")]
    pub spec: Option<SwarmSpec>,
}

#[napi(object)]
pub struct SwarmJoinRequest {
    /// Listen address used for inter-manager communication if the node gets promoted to manager, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP).
    #[napi(js_name = "ListenAddr")]
    pub listen_addr: Option<String>,

    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[napi(js_name = "AdvertiseAddr")]
    pub advertise_addr: Option<String>,

    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same address as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[napi(js_name = "DataPathAddr")]
    pub data_path_addr: Option<String>,

    /// Addresses of manager nodes already participating in the swarm.
    #[napi(js_name = "RemoteAddrs")]
    pub remote_addrs: Option<Vec<String>>,

    /// Secret token for joining this swarm.
    #[napi(js_name = "JoinToken")]
    pub join_token: Option<String>,
}

/// User modifiable swarm configuration.
#[napi(object)]
pub struct SwarmSpec {
    /// Name of the swarm.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    #[napi(js_name = "Orchestration")]
    pub orchestration: Option<SwarmSpecOrchestration>,

    #[napi(js_name = "Raft")]
    pub raft: Option<SwarmSpecRaft>,

    #[napi(js_name = "Dispatcher")]
    pub dispatcher: Option<SwarmSpecDispatcher>,

    #[napi(js_name = "CAConfig")]
    pub ca_config: Option<SwarmSpecCaConfig>,

    #[napi(js_name = "EncryptionConfig")]
    pub encryption_config: Option<SwarmSpecEncryptionConfig>,

    #[napi(js_name = "TaskDefaults")]
    pub task_defaults: Option<SwarmSpecTaskDefaults>,
}

/// CA configuration.
#[napi(object)]
pub struct SwarmSpecCaConfig {
    /// The duration node certificates are issued for.
    #[napi(js_name = "NodeCertExpiry")]
    pub node_cert_expiry: Option<i64>,

    /// Configuration for forwarding signing requests to an external certificate authority.
    #[napi(js_name = "ExternalCAs")]
    pub external_cas: Option<Vec<SwarmSpecCaConfigExternalCas>>,

    /// The desired signing CA certificate for all swarm node TLS leaf certificates, in PEM format.
    #[napi(js_name = "SigningCACert")]
    pub signing_ca_cert: Option<String>,

    /// The desired signing CA key for all swarm node TLS leaf certificates, in PEM format.
    #[napi(js_name = "SigningCAKey")]
    pub signing_ca_key: Option<String>,

    /// An integer whose purpose is to force swarm to generate a new signing CA certificate and key, if none have been specified in `SigningCACert` and `SigningCAKey`
    #[napi(js_name = "ForceRotate")]
    pub force_rotate: Option<i64>,
}

#[napi(object)]
pub struct SwarmSpecCaConfigExternalCas {
    /// Protocol for communication with the external CA (currently only `cfssl` is supported).
    #[napi(js_name = "Protocol")]
    pub protocol: Option<SwarmSpecCaConfigExternalCasProtocolEnum>,

    /// URL where certificate signing requests should be sent.
    #[napi(js_name = "URL")]
    pub url: Option<String>,

    /// An object with key/value pairs that are interpreted as protocol-specific options for the external CA driver.
    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,

    /// The root CA certificate (in PEM format) this external CA uses to issue TLS certificates (assumed to be to the current swarm root CA certificate if not provided).
    #[napi(js_name = "CACert")]
    pub ca_cert: Option<String>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum SwarmSpecCaConfigExternalCasProtocolEnum {
    EMPTY,
    CFSSL,
}

/// Dispatcher configuration.
#[napi(object)]
pub struct SwarmSpecDispatcher {
    /// The delay for an agent to send a heartbeat to the dispatcher.
    #[napi(js_name = "HeartbeatPeriod")]
    pub heartbeat_period: Option<i64>,
}

/// Parameters related to encryption-at-rest.
#[napi(object)]
pub struct SwarmSpecEncryptionConfig {
    /// If set, generate a key and use it to lock data stored on the managers.
    #[napi(js_name = "AutoLockManagers")]
    pub auto_lock_managers: Option<bool>,
}

/// Orchestration configuration.
#[napi(object)]
pub struct SwarmSpecOrchestration {
    /// The number of historic tasks to keep per instance or node. If negative, never remove completed or failed tasks.
    #[napi(js_name = "TaskHistoryRetentionLimit")]
    pub task_history_retention_limit: Option<i64>,
}

/// Raft configuration.
#[napi(object)]
pub struct SwarmSpecRaft {
    /// The number of log entries between snapshots.
    #[napi(js_name = "SnapshotInterval")]
    pub snapshot_interval: Option<u32>,

    /// The number of snapshots to keep beyond the current snapshot.
    #[napi(js_name = "KeepOldSnapshots")]
    pub keep_old_snapshots: Option<u32>,

    /// The number of log entries to keep around to sync up slow followers after a snapshot is created.
    #[napi(js_name = "LogEntriesForSlowFollowers")]
    pub log_entries_for_slow_followers: Option<u32>,

    /// The number of ticks that a follower will wait for a message from the leader before becoming a candidate and starting an election. `ElectionTick` must be greater than `HeartbeatTick`.  A tick currently defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed.
    #[napi(js_name = "ElectionTick")]
    pub election_tick: Option<i64>,

    /// The number of ticks between heartbeats. Every HeartbeatTick ticks, the leader will send a heartbeat to the followers.  A tick currently defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed.
    #[napi(js_name = "HeartbeatTick")]
    pub heartbeat_tick: Option<i64>,
}

/// Defaults for creating tasks in this cluster.
#[napi(object)]
pub struct SwarmSpecTaskDefaults {
    #[napi(js_name = "LogDriver")]
    pub log_driver: Option<SwarmSpecTaskDefaultsLogDriver>,
}

/// The log driver to use for tasks created in the orchestrator if unspecified by a service.  Updating this value only affects new tasks. Existing tasks continue to use their previously configured log driver until recreated.
#[napi(object)]
pub struct SwarmSpecTaskDefaultsLogDriver {
    /// The log driver to use as a default for new tasks.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// Driver-specific options for the selected log driver, specified as key/value pairs.
    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,
}

#[napi(object)]
pub struct SwarmUnlockRequest {
    /// The swarm's unlock key.
    #[napi(js_name = "UnlockKey")]
    pub unlock_key: Option<String>,
}

#[napi(object)]
pub struct SystemAuthResponse {
    /// The status of the authentication
    #[napi(js_name = "Status")]
    pub status: String,

    /// An opaque token used to authenticate a user after a successful login
    #[napi(js_name = "IdentityToken")]
    pub identity_token: Option<String>,
}

#[napi(object)]
pub struct SystemDataUsageResponse {
    #[napi(js_name = "LayersSize")]
    pub layers_size: Option<i64>,

    #[napi(js_name = "Images")]
    pub images: Option<Vec<ImageSummary>>,

    #[napi(js_name = "Containers")]
    pub containers: Option<Vec<ContainerSummary>>,

    #[napi(js_name = "Volumes")]
    pub volumes: Option<Vec<Volume>>,

    #[napi(js_name = "BuildCache")]
    pub build_cache: Option<Vec<BuildCache>>,
}

#[napi(object)]
pub struct SystemInfo {
    /// Unique identifier of the daemon.  <p><br /></p>  > **Note**: The format of the ID itself is not part of the API, and > should not be considered stable.
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    /// Total number of containers on the host.
    #[napi(js_name = "Containers")]
    pub containers: Option<i64>,

    /// Number of containers with status `'running'`.
    #[napi(js_name = "ContainersRunning")]
    pub containers_running: Option<i64>,

    /// Number of containers with status `'paused'`.
    #[napi(js_name = "ContainersPaused")]
    pub containers_paused: Option<i64>,

    /// Number of containers with status `'stopped'`.
    #[napi(js_name = "ContainersStopped")]
    pub containers_stopped: Option<i64>,

    /// Total number of images on the host.  Both _tagged_ and _untagged_ (dangling) images are counted.
    #[napi(js_name = "Images")]
    pub images: Option<i64>,

    /// Name of the storage driver in use.
    #[napi(js_name = "Driver")]
    pub driver: Option<String>,

    /// Information specific to the storage driver, provided as 'label' / 'value' pairs.  This information is provided by the storage driver, and formatted in a way consistent with the output of `docker info` on the command line.  <p><br /></p>  > **Note**: The information returned in this field, including the > formatting of values and labels, should not be considered stable, > and may change without notice.
    #[napi(js_name = "DriverStatus")]
    pub driver_status: Option<Vec<Vec<String>>>,

    /// Root directory of persistent Docker state.  Defaults to `/var/lib/docker` on Linux, and `C:\\ProgramData\\docker` on Windows.
    #[napi(js_name = "DockerRootDir")]
    pub docker_root_dir: Option<String>,

    #[napi(js_name = "Plugins")]
    pub plugins: Option<PluginsInfo>,

    /// Indicates if the host has memory limit support enabled.
    #[napi(js_name = "MemoryLimit")]
    pub memory_limit: Option<bool>,

    /// Indicates if the host has memory swap limit support enabled.
    #[napi(js_name = "SwapLimit")]
    pub swap_limit: Option<bool>,

    /// Indicates if the host has kernel memory TCP limit support enabled. This field is omitted if not supported.  Kernel memory TCP limits are not supported when using cgroups v2, which does not support the corresponding `memory.kmem.tcp.limit_in_bytes` cgroup.
    #[napi(js_name = "KernelMemoryTCP")]
    pub kernel_memory_tcp: Option<bool>,

    /// Indicates if CPU CFS(Completely Fair Scheduler) period is supported by the host.
    #[napi(js_name = "CpuCfsPeriod")]
    pub cpu_cfs_period: Option<bool>,

    /// Indicates if CPU CFS(Completely Fair Scheduler) quota is supported by the host.
    #[napi(js_name = "CpuCfsQuota")]
    pub cpu_cfs_quota: Option<bool>,

    /// Indicates if CPU Shares limiting is supported by the host.
    #[napi(js_name = "CPUShares")]
    pub cpu_shares: Option<bool>,

    /// Indicates if CPUsets (cpuset.cpus, cpuset.mems) are supported by the host.  See [cpuset(7)](https://www.kernel.org/doc/Documentation/cgroup-v1/cpusets.txt)
    #[napi(js_name = "CPUSet")]
    pub cpu_set: Option<bool>,

    /// Indicates if the host kernel has PID limit support enabled.
    #[napi(js_name = "PidsLimit")]
    pub pids_limit: Option<bool>,

    /// Indicates if OOM killer disable is supported on the host.
    #[napi(js_name = "OomKillDisable")]
    pub oom_kill_disable: Option<bool>,

    /// Indicates IPv4 forwarding is enabled.
    #[napi(js_name = "IPv4Forwarding")]
    pub ipv4_forwarding: Option<bool>,

    /// Indicates if `bridge-nf-call-iptables` is available on the host.
    #[napi(js_name = "BridgeNfIptables")]
    pub bridge_nf_iptables: Option<bool>,

    /// Indicates if `bridge-nf-call-ip6tables` is available on the host.
    #[napi(js_name = "BridgeNfIp6tables")]
    pub bridge_nf_ip6tables: Option<bool>,

    /// Indicates if the daemon is running in debug-mode / with debug-level logging enabled.
    #[napi(js_name = "Debug")]
    pub debug: Option<bool>,

    /// The total number of file Descriptors in use by the daemon process.  This information is only returned if debug-mode is enabled.
    #[napi(js_name = "NFd")]
    pub nfd: Option<i64>,

    /// The  number of goroutines that currently exist.  This information is only returned if debug-mode is enabled.
    #[napi(js_name = "NGoroutines")]
    pub n_goroutines: Option<i64>,

    /// Current system-time in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[napi(js_name = "SystemTime")]
    pub system_time: Option<String>,

    /// The logging driver to use as a default for new containers.
    #[napi(js_name = "LoggingDriver")]
    pub logging_driver: Option<String>,

    /// The driver to use for managing cgroups.
    #[napi(js_name = "CgroupDriver")]
    pub cgroup_driver: Option<SystemInfoCgroupDriverEnum>,

    /// The version of the cgroup.
    #[napi(js_name = "CgroupVersion")]
    pub cgroup_version: Option<SystemInfoCgroupVersionEnum>,

    /// Number of event listeners subscribed.
    #[napi(js_name = "NEventsListener")]
    pub n_events_listener: Option<i64>,

    /// Kernel version of the host.  On Linux, this information obtained from `uname`. On Windows this information is queried from the <kbd>HKEY_LOCAL_MACHINE\\\\SOFTWARE\\\\Microsoft\\\\Windows NT\\\\CurrentVersion\\\\</kbd> registry value, for example _'10.0 14393 (14393.1198.amd64free.rs1_release_sec.170427-1353)'_.
    #[napi(js_name = "KernelVersion")]
    pub kernel_version: Option<String>,

    /// Name of the host's operating system, for example: 'Ubuntu 24.04 LTS' or 'Windows Server 2016 Datacenter'
    #[napi(js_name = "OperatingSystem")]
    pub operating_system: Option<String>,

    /// Version of the host's operating system  <p><br /></p>  > **Note**: The information returned in this field, including its > very existence, and the formatting of values, should not be considered > stable, and may change without notice.
    #[napi(js_name = "OSVersion")]
    pub os_version: Option<String>,

    /// Generic type of the operating system of the host, as returned by the Go runtime (`GOOS`).  Currently returned values are 'linux' and 'windows'. A full list of possible values can be found in the [Go documentation](https://go.dev/doc/install/source#environment).
    #[napi(js_name = "OSType")]
    pub os_type: Option<String>,

    /// Hardware architecture of the host, as returned by the Go runtime (`GOARCH`).  A full list of possible values can be found in the [Go documentation](https://go.dev/doc/install/source#environment).
    #[napi(js_name = "Architecture")]
    pub architecture: Option<String>,

    /// The number of logical CPUs usable by the daemon.  The number of available CPUs is checked by querying the operating system when the daemon starts. Changes to operating system CPU allocation after the daemon is started are not reflected.
    #[napi(js_name = "NCPU")]
    pub ncpu: Option<i64>,

    /// Total amount of physical memory available on the host, in bytes.
    #[napi(js_name = "MemTotal")]
    pub mem_total: Option<i64>,

    /// Address / URL of the index server that is used for image search, and as a default for user authentication for Docker Hub and Docker Cloud.
    #[napi(js_name = "IndexServerAddress")]
    pub index_server_address: Option<String>,

    #[napi(js_name = "RegistryConfig")]
    pub registry_config: Option<RegistryServiceConfig>,

    #[napi(js_name = "GenericResources")]
    pub generic_resources: Option<GenericResourcesInner>,

    /// HTTP-proxy configured for the daemon. This value is obtained from the [`HTTP_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration.
    #[napi(js_name = "HttpProxy")]
    pub http_proxy: Option<String>,

    /// HTTPS-proxy configured for the daemon. This value is obtained from the [`HTTPS_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration.
    #[napi(js_name = "HttpsProxy")]
    pub https_proxy: Option<String>,

    /// Comma-separated list of domain extensions for which no proxy should be used. This value is obtained from the [`NO_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable.  Containers do not automatically inherit this configuration.
    #[napi(js_name = "NoProxy")]
    pub no_proxy: Option<String>,

    /// Hostname of the host.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// User-defined labels (key/value metadata) as set on the daemon.  <p><br /></p>  > **Note**: When part of a Swarm, nodes can both have _daemon_ labels, > set through the daemon configuration, and _node_ labels, set from a > manager node in the Swarm. Node labels are not included in this > field. Node labels can be retrieved using the `/nodes/(id)` endpoint > on a manager node in the Swarm.
    #[napi(js_name = "Labels")]
    pub labels: Option<Vec<String>>,

    /// Indicates if experimental features are enabled on the daemon.
    #[napi(js_name = "ExperimentalBuild")]
    pub experimental_build: Option<bool>,

    /// Version string of the daemon.
    #[napi(js_name = "ServerVersion")]
    pub server_version: Option<String>,

    /// List of [OCI compliant](https://github.com/opencontainers/runtime-spec) runtimes configured on the daemon. Keys hold the 'name' used to reference the runtime.  The Docker daemon relies on an OCI compliant runtime (invoked via the `containerd` daemon) as its interface to the Linux kernel namespaces, cgroups, and SELinux.  The default runtime is `runc`, and automatically configured. Additional runtimes can be configured by the user and will be listed here.
    #[napi(js_name = "Runtimes")]
    pub runtimes: Option<HashMap<String, Runtime>>,

    /// Name of the default OCI runtime that is used when starting containers.  The default can be overridden per-container at create time.
    #[napi(js_name = "DefaultRuntime")]
    pub default_runtime: Option<String>,

    #[napi(js_name = "Swarm")]
    pub swarm: Option<SwarmInfo>,

    /// Indicates if live restore is enabled.  If enabled, containers are kept running when the daemon is shutdown or upon daemon start if running containers are detected.
    #[napi(js_name = "LiveRestoreEnabled")]
    pub live_restore_enabled: Option<bool>,

    /// Represents the isolation technology to use as a default for containers. The supported values are platform-specific.  If no isolation value is specified on daemon start, on Windows client, the default is `hyperv`, and on Windows server, the default is `process`.  This option is currently not used on other platforms.
    #[napi(js_name = "Isolation")]
    pub isolation: Option<SystemInfoIsolationEnum>,

    /// Name and, optional, path of the `docker-init` binary.  If the path is omitted, the daemon searches the host's `$PATH` for the binary and uses the first result.
    #[napi(js_name = "InitBinary")]
    pub init_binary: Option<String>,

    #[napi(js_name = "ContainerdCommit")]
    pub containerd_commit: Option<Commit>,

    #[napi(js_name = "RuncCommit")]
    pub runc_commit: Option<Commit>,

    #[napi(js_name = "InitCommit")]
    pub init_commit: Option<Commit>,

    /// List of security features that are enabled on the daemon, such as apparmor, seccomp, SELinux, user-namespaces (userns), rootless and no-new-privileges.  Additional configuration options for each security feature may be present, and are included as a comma-separated list of key/value pairs.
    #[napi(js_name = "SecurityOptions")]
    pub security_options: Option<Vec<String>>,

    /// Reports a summary of the product license on the daemon.  If a commercial license has been applied to the daemon, information such as number of nodes, and expiration are included.
    #[napi(js_name = "ProductLicense")]
    pub product_license: Option<String>,

    /// List of custom default address pools for local networks, which can be specified in the daemon.json file or dockerd option.  Example: a Base '10.10.0.0/16' with Size 24 will define the set of 256 10.10.[0-255].0/24 address pools.
    #[napi(js_name = "DefaultAddressPools")]
    pub default_address_pools: Option<Vec<SystemInfoDefaultAddressPools>>,

    /// List of warnings / informational messages about missing features, or issues related to the daemon configuration.  These messages can be printed by the client as information to the user.
    #[napi(js_name = "Warnings")]
    pub warnings: Option<Vec<String>>,

    /// List of directories where (Container Device Interface) CDI specifications are located.  These specifications define vendor-specific modifications to an OCI runtime specification for a container being created.  An empty list indicates that CDI device injection is disabled.  Note that since using CDI device injection requires the daemon to have experimental enabled. For non-experimental daemons an empty list will always be returned.
    #[napi(js_name = "CDISpecDirs")]
    pub cdi_spec_dirs: Option<Vec<String>>,

    #[napi(js_name = "Containerd")]
    pub containerd: Option<ContainerdInfo>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum SystemInfoCgroupDriverEnum {
    EMPTY,
    CGROUPFS,
    SYSTEMD,
    NONE,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum SystemInfoCgroupVersionEnum {
    EMPTY,
    _1,
    _2,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum SystemInfoIsolationEnum {
    EMPTY,
    DEFAULT,
    HYPERV,
    PROCESS,
}

#[napi(object)]
pub struct SystemInfoDefaultAddressPools {
    /// The network address in CIDR format
    #[napi(js_name = "Base")]
    pub base: Option<String>,

    /// The network pool size
    #[napi(js_name = "Size")]
    pub size: Option<i64>,
}

/// Response of Engine API: GET '/version'
#[napi(object)]
pub struct SystemVersion {
    #[napi(js_name = "Platform")]
    pub platform: Option<SystemVersionPlatform>,

    /// Information about system components
    #[napi(js_name = "Components")]
    pub components: Option<Vec<SystemVersionComponents>>,

    /// The version of the daemon
    #[napi(js_name = "Version")]
    pub version: Option<String>,

    /// The default (and highest) API version that is supported by the daemon
    #[napi(js_name = "ApiVersion")]
    pub api_version: Option<String>,

    /// The minimum API version that is supported by the daemon
    #[napi(js_name = "MinAPIVersion")]
    pub min_api_version: Option<String>,

    /// The Git commit of the source code that was used to build the daemon
    #[napi(js_name = "GitCommit")]
    pub git_commit: Option<String>,

    /// The version Go used to compile the daemon, and the version of the Go runtime in use.
    #[napi(js_name = "GoVersion")]
    pub go_version: Option<String>,

    /// The operating system that the daemon is running on ('linux' or 'windows')
    #[napi(js_name = "Os")]
    pub os: Option<String>,

    /// The architecture that the daemon is running on
    #[napi(js_name = "Arch")]
    pub arch: Option<String>,

    /// The kernel version (`uname -r`) that the daemon is running on.  This field is omitted when empty.
    #[napi(js_name = "KernelVersion")]
    pub kernel_version: Option<String>,

    /// Indicates if the daemon is started with experimental features enabled.  This field is omitted when empty / false.
    #[napi(js_name = "Experimental")]
    pub experimental: Option<bool>,

    /// The date and time that the daemon was compiled.
    #[napi(js_name = "BuildTime")]
    pub build_time: Option<String>,
}

#[napi(object)]
pub struct SystemVersionComponents {
    /// Name of the component
    #[napi(js_name = "Name")]
    pub name: String,

    /// Version of the component
    #[napi(js_name = "Version")]
    pub version: String,

    /// Key/value pairs of strings with additional information about the component. These values are intended for informational purposes only, and their content is not defined, and not part of the API specification.  These messages can be printed by the client as information to the user.
    #[napi(js_name = "Details")]
    pub details: Option<()>,
}

#[napi(object)]
pub struct SystemVersionPlatform {
    #[napi(js_name = "Name")]
    pub name: String,
}

#[napi(object)]
pub struct Task {
    /// The ID of the task.
    #[napi(js_name = "ID")]
    pub id: Option<String>,

    #[napi(js_name = "Version")]
    pub version: Option<ObjectVersion>,

    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    #[napi(js_name = "UpdatedAt")]
    pub updated_at: Option<String>,

    /// Name of the task.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    #[napi(js_name = "Spec")]
    pub spec: Option<TaskSpec>,

    /// The ID of the service this task is part of.
    #[napi(js_name = "ServiceID")]
    pub service_id: Option<String>,

    #[napi(js_name = "Slot")]
    pub slot: Option<i64>,

    /// The ID of the node that this task is on.
    #[napi(js_name = "NodeID")]
    pub node_id: Option<String>,

    #[napi(js_name = "AssignedGenericResources")]
    pub assigned_generic_resources: Option<GenericResourcesInner>,

    #[napi(js_name = "Status")]
    pub status: Option<TaskStatus>,

    #[napi(js_name = "DesiredState")]
    pub desired_state: Option<TaskState>,

    /// If the Service this Task belongs to is a job-mode service, contains the JobIteration of the Service this Task was created for. Absent if the Task was created for a Replicated or Global Service.
    #[napi(js_name = "JobIteration")]
    pub job_iteration: Option<ObjectVersion>,
}

/// User modifiable task configuration.
#[napi(object)]
pub struct TaskSpec {
    #[napi(js_name = "PluginSpec")]
    pub plugin_spec: Option<TaskSpecPluginSpec>,

    #[napi(js_name = "ContainerSpec")]
    pub container_spec: Option<TaskSpecContainerSpec>,

    #[napi(js_name = "NetworkAttachmentSpec")]
    pub network_attachment_spec: Option<TaskSpecNetworkAttachmentSpec>,

    #[napi(js_name = "Resources")]
    pub resources: Option<TaskSpecResources>,

    #[napi(js_name = "RestartPolicy")]
    pub restart_policy: Option<TaskSpecRestartPolicy>,

    #[napi(js_name = "Placement")]
    pub placement: Option<TaskSpecPlacement>,

    /// A counter that triggers an update even if no relevant parameters have been changed.
    #[napi(js_name = "ForceUpdate")]
    pub force_update: Option<i64>,

    /// Runtime is the type of runtime specified for the task executor.
    #[napi(js_name = "Runtime")]
    pub runtime: Option<String>,

    /// Specifies which networks the service should attach to.
    #[napi(js_name = "Networks")]
    pub networks: Option<Vec<NetworkAttachmentConfig>>,

    #[napi(js_name = "LogDriver")]
    pub log_driver: Option<TaskSpecLogDriver>,
}

/// Container spec for the service.  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[napi(object)]
pub struct TaskSpecContainerSpec {
    /// The image name to use for the container
    #[napi(js_name = "Image")]
    pub image: Option<String>,

    /// User-defined key/value data.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// The command to be run in the image.
    #[napi(js_name = "Command")]
    pub command: Option<Vec<String>>,

    /// Arguments to the command.
    #[napi(js_name = "Args")]
    pub args: Option<Vec<String>>,

    /// The hostname to use for the container, as a valid [RFC 1123](https://tools.ietf.org/html/rfc1123) hostname.
    #[napi(js_name = "Hostname")]
    pub hostname: Option<String>,

    /// A list of environment variables in the form `VAR=value`.
    #[napi(js_name = "Env")]
    pub env: Option<Vec<String>>,

    /// The working directory for commands to run in.
    #[napi(js_name = "Dir")]
    pub dir: Option<String>,

    /// The user inside the container.
    #[napi(js_name = "User")]
    pub user: Option<String>,

    /// A list of additional groups that the container process will run as.
    #[napi(js_name = "Groups")]
    pub groups: Option<Vec<String>>,

    #[napi(js_name = "Privileges")]
    pub privileges: Option<TaskSpecContainerSpecPrivileges>,

    /// Whether a pseudo-TTY should be allocated.
    #[napi(js_name = "TTY")]
    pub tty: Option<bool>,

    /// Open `stdin`
    #[napi(js_name = "OpenStdin")]
    pub open_stdin: Option<bool>,

    /// Mount the container's root filesystem as read only.
    #[napi(js_name = "ReadOnly")]
    pub read_only: Option<bool>,

    /// Specification for mounts to be added to containers created as part of the service.
    #[napi(js_name = "Mounts")]
    pub mounts: Option<Vec<Mount>>,

    /// Signal to stop the container.
    #[napi(js_name = "StopSignal")]
    pub stop_signal: Option<String>,

    /// Amount of time to wait for the container to terminate before forcefully killing it.
    #[napi(js_name = "StopGracePeriod")]
    pub stop_grace_period: Option<i64>,

    #[napi(js_name = "HealthCheck")]
    pub health_check: Option<HealthConfig>,

    /// A list of hostname/IP mappings to add to the container's `hosts` file. The format of extra hosts is specified in the [hosts(5)](http://man7.org/linux/man-pages/man5/hosts.5.html) man page:      IP_address canonical_hostname [aliases...]
    #[napi(js_name = "Hosts")]
    pub hosts: Option<Vec<String>>,

    #[napi(js_name = "DNSConfig")]
    pub dns_config: Option<TaskSpecContainerSpecDnsConfig>,

    /// Secrets contains references to zero or more secrets that will be exposed to the service.
    #[napi(js_name = "Secrets")]
    pub secrets: Option<Vec<TaskSpecContainerSpecSecrets>>,

    /// An integer value containing the score given to the container in order to tune OOM killer preferences.
    #[napi(js_name = "OomScoreAdj")]
    pub oom_score_adj: Option<i64>,

    /// Configs contains references to zero or more configs that will be exposed to the service.
    #[napi(js_name = "Configs")]
    pub configs: Option<Vec<TaskSpecContainerSpecConfigs>>,

    /// Isolation technology of the containers running the service. (Windows only)
    #[napi(js_name = "Isolation")]
    pub isolation: Option<TaskSpecContainerSpecIsolationEnum>,

    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[napi(js_name = "Init")]
    pub init: Option<bool>,

    /// Set kernel namedspaced parameters (sysctls) in the container. The Sysctls option on services accepts the same sysctls as the are supported on containers. Note that while the same sysctls are supported, no guarantees or checks are made about their suitability for a clustered environment, and it's up to the user to determine whether a given sysctl will work properly in a Service.
    #[napi(js_name = "Sysctls")]
    pub sysctls: Option<HashMap<String, String>>,

    /// A list of kernel capabilities to add to the default set for the container.
    #[napi(js_name = "CapabilityAdd")]
    pub capability_add: Option<Vec<String>>,

    /// A list of kernel capabilities to drop from the default set for the container.
    #[napi(js_name = "CapabilityDrop")]
    pub capability_drop: Option<Vec<String>>,

    /// A list of resource limits to set in the container. For example: `{'Name': 'nofile', 'Soft': 1024, 'Hard': 2048}`"
    #[napi(js_name = "Ulimits")]
    pub ulimits: Option<Vec<ResourcesUlimits>>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum TaskSpecContainerSpecIsolationEnum {
    EMPTY,
    DEFAULT,
    PROCESS,
    HYPERV,
}

#[napi(object)]
pub struct TaskSpecContainerSpecConfigs {
    #[napi(js_name = "File")]
    pub file: Option<TaskSpecContainerSpecFile1>,

    /// Runtime represents a target that is not mounted into the container but is used by the task  <p><br /><p>  > **Note**: `Configs.File` and `Configs.Runtime` are mutually > exclusive
    #[napi(js_name = "Runtime")]
    pub runtime: Option<()>,

    /// ConfigID represents the ID of the specific config that we're referencing.
    #[napi(js_name = "ConfigID")]
    pub config_id: Option<String>,

    /// ConfigName is the name of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID.
    #[napi(js_name = "ConfigName")]
    pub config_name: Option<String>,
}

/// Specification for DNS related configurations in resolver configuration file (`resolv.conf`).
#[napi(object)]
pub struct TaskSpecContainerSpecDnsConfig {
    /// The IP addresses of the name servers.
    #[napi(js_name = "Nameservers")]
    pub nameservers: Option<Vec<String>>,

    /// A search list for host-name lookup.
    #[napi(js_name = "Search")]
    pub search: Option<Vec<String>>,

    /// A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.).
    #[napi(js_name = "Options")]
    pub options: Option<Vec<String>>,
}

/// File represents a specific target that is backed by a file.
#[napi(object)]
pub struct TaskSpecContainerSpecFile {
    /// Name represents the final filename in the filesystem.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// UID represents the file UID.
    #[napi(js_name = "UID")]
    pub uid: Option<String>,

    /// GID represents the file GID.
    #[napi(js_name = "GID")]
    pub gid: Option<String>,

    /// Mode represents the FileMode of the file.
    #[napi(js_name = "Mode")]
    pub mode: Option<u32>,
}

/// File represents a specific target that is backed by a file.  <p><br /><p>  > **Note**: `Configs.File` and `Configs.Runtime` are mutually exclusive
#[napi(object)]
pub struct TaskSpecContainerSpecFile1 {
    /// Name represents the final filename in the filesystem.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// UID represents the file UID.
    #[napi(js_name = "UID")]
    pub uid: Option<String>,

    /// GID represents the file GID.
    #[napi(js_name = "GID")]
    pub gid: Option<String>,

    /// Mode represents the FileMode of the file.
    #[napi(js_name = "Mode")]
    pub mode: Option<u32>,
}

/// Security options for the container
#[napi(object)]
pub struct TaskSpecContainerSpecPrivileges {
    #[napi(js_name = "CredentialSpec")]
    pub credential_spec: Option<TaskSpecContainerSpecPrivilegesCredentialSpec>,

    #[napi(js_name = "SELinuxContext")]
    pub se_linux_context: Option<TaskSpecContainerSpecPrivilegesSeLinuxContext>,

    #[napi(js_name = "Seccomp")]
    pub seccomp: Option<TaskSpecContainerSpecPrivilegesSeccomp>,

    #[napi(js_name = "AppArmor")]
    pub app_armor: Option<TaskSpecContainerSpecPrivilegesAppArmor>,

    /// Configuration of the no_new_privs bit in the container
    #[napi(js_name = "NoNewPrivileges")]
    pub no_new_privileges: Option<bool>,
}

/// Options for configuring AppArmor on the container
#[napi(object)]
pub struct TaskSpecContainerSpecPrivilegesAppArmor {
    #[napi(js_name = "Mode")]
    pub mode: Option<TaskSpecContainerSpecPrivilegesAppArmorModeEnum>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum TaskSpecContainerSpecPrivilegesAppArmorModeEnum {
    EMPTY,
    DEFAULT,
    DISABLED,
}

/// CredentialSpec for managed service account (Windows only)
#[napi(object)]
pub struct TaskSpecContainerSpecPrivilegesCredentialSpec {
    /// Load credential spec from a Swarm Config with the given ID. The specified config must also be present in the Configs field with the Runtime property set.  <p><br /></p>   > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[napi(js_name = "Config")]
    pub config: Option<String>,

    /// Load credential spec from this file. The file is read by the daemon, and must be present in the `CredentialSpecs` subdirectory in the docker data directory, which defaults to `C:\\ProgramData\\Docker\\` on Windows.  For example, specifying `spec.json` loads `C:\\ProgramData\\Docker\\CredentialSpecs\\spec.json`.  <p><br /></p>  > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[napi(js_name = "File")]
    pub file: Option<String>,

    /// Load credential spec from this value in the Windows registry. The specified registry value must be located in:  `HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Virtualization\\Containers\\CredentialSpecs`  <p><br /></p>   > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[napi(js_name = "Registry")]
    pub registry: Option<String>,
}

/// SELinux labels of the container
#[napi(object)]
pub struct TaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// Disable SELinux
    #[napi(js_name = "Disable")]
    pub disable: Option<bool>,

    /// SELinux user label
    #[napi(js_name = "User")]
    pub user: Option<String>,

    /// SELinux role label
    #[napi(js_name = "Role")]
    pub role: Option<String>,

    /// SELinux type label
    #[napi(js_name = "Type")]
    pub typ: Option<String>,

    /// SELinux level label
    #[napi(js_name = "Level")]
    pub level: Option<String>,
}

/// Options for configuring seccomp on the container
#[napi(object)]
pub struct TaskSpecContainerSpecPrivilegesSeccomp {
    #[napi(js_name = "Mode")]
    pub mode: Option<TaskSpecContainerSpecPrivilegesSeccompModeEnum>,

    /// The custom seccomp profile as a json object
    #[napi(js_name = "Profile")]
    pub profile: Option<String>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum TaskSpecContainerSpecPrivilegesSeccompModeEnum {
    EMPTY,
    DEFAULT,
    UNCONFINED,
    CUSTOM,
}

#[napi(object)]
pub struct TaskSpecContainerSpecSecrets {
    #[napi(js_name = "File")]
    pub file: Option<TaskSpecContainerSpecFile>,

    /// SecretID represents the ID of the specific secret that we're referencing.
    #[napi(js_name = "SecretID")]
    pub secret_id: Option<String>,

    /// SecretName is the name of the secret that this references, but this is just provided for lookup/display purposes. The secret in the reference will be identified by its ID.
    #[napi(js_name = "SecretName")]
    pub secret_name: Option<String>,
}

/// Specifies the log driver to use for tasks created from this spec. If not present, the default one for the swarm will be used, finally falling back to the engine default if not specified.
#[napi(object)]
pub struct TaskSpecLogDriver {
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    #[napi(js_name = "Options")]
    pub options: Option<HashMap<String, String>>,
}

/// Read-only spec type for non-swarm containers attached to swarm overlay networks.  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[napi(object)]
pub struct TaskSpecNetworkAttachmentSpec {
    /// ID of the container represented by this task
    #[napi(js_name = "ContainerID")]
    pub container_id: Option<String>,
}

#[napi(object)]
pub struct TaskSpecPlacement {
    /// An array of constraint expressions to limit the set of nodes where a task can be scheduled. Constraint expressions can either use a _match_ (`==`) or _exclude_ (`!=`) rule. Multiple constraints find nodes that satisfy every expression (AND match). Constraints can match node or Docker Engine labels as follows:  node attribute       | matches                        | example ---------------------|--------------------------------|----------------------------------------------- `node.id`            | Node ID                        | `node.id==2ivku8v2gvtg4` `node.hostname`      | Node hostname                  | `node.hostname!=node-2` `node.role`          | Node role (`manager`/`worker`) | `node.role==manager` `node.platform.os`   | Node operating system          | `node.platform.os==windows` `node.platform.arch` | Node architecture              | `node.platform.arch==x86_64` `node.labels`        | User-defined node labels       | `node.labels.security==high` `engine.labels`      | Docker Engine's labels         | `engine.labels.operatingsystem==ubuntu-24.04`  `engine.labels` apply to Docker Engine labels like operating system, drivers, etc. Swarm administrators add `node.labels` for operational purposes by using the [`node update endpoint`](#operation/NodeUpdate).
    #[napi(js_name = "Constraints")]
    pub constraints: Option<Vec<String>>,

    /// Preferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence.
    #[napi(js_name = "Preferences")]
    pub preferences: Option<Vec<TaskSpecPlacementPreferences>>,

    /// Maximum number of replicas for per node (default value is 0, which is unlimited)
    #[napi(js_name = "MaxReplicas")]
    pub max_replicas: Option<i64>,

    /// Platforms stores all the platforms that the service's image can run on. This field is used in the platform filter for scheduling. If empty, then the platform filter is off, meaning there are no scheduling restrictions.
    #[napi(js_name = "Platforms")]
    pub platforms: Option<Vec<Platform>>,
}

#[napi(object)]
pub struct TaskSpecPlacementPreferences {
    #[napi(js_name = "Spread")]
    pub spread: Option<TaskSpecPlacementSpread>,
}

#[napi(object)]
pub struct TaskSpecPlacementSpread {
    /// label descriptor, such as `engine.labels.az`.
    #[napi(js_name = "SpreadDescriptor")]
    pub spread_descriptor: Option<String>,
}

/// Plugin spec for the service.  *(Experimental release only.)*  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[napi(object)]
pub struct TaskSpecPluginSpec {
    /// The name or 'alias' to use for the plugin.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// The plugin image reference to use.
    #[napi(js_name = "Remote")]
    pub remote: Option<String>,

    /// Disable the plugin once scheduled.
    #[napi(js_name = "Disabled")]
    pub disabled: Option<bool>,

    #[napi(js_name = "PluginPrivilege")]
    pub plugin_privilege: Option<Vec<PluginPrivilege>>,
}

/// Resource requirements which apply to each individual container created as part of the service.
#[napi(object)]
pub struct TaskSpecResources {
    /// Define resources limits.
    #[napi(js_name = "Limits")]
    pub limits: Option<Limit>,

    /// Define resources reservation.
    #[napi(js_name = "Reservations")]
    pub reservations: Option<ResourceObject>,
}

/// Specification for the restart policy which applies to containers created as part of this service.
#[napi(object)]
pub struct TaskSpecRestartPolicy {
    /// Condition for restart.
    #[napi(js_name = "Condition")]
    pub condition: Option<TaskSpecRestartPolicyConditionEnum>,

    /// Delay between restart attempts.
    #[napi(js_name = "Delay")]
    pub delay: Option<i64>,

    /// Maximum attempts to restart a given container before giving up (default value is 0, which is ignored).
    #[napi(js_name = "MaxAttempts")]
    pub max_attempts: Option<i64>,

    /// Windows is the time window used to evaluate the restart policy (default value is 0, which is unbounded).
    #[napi(js_name = "Window")]
    pub window: Option<i64>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum TaskSpecRestartPolicyConditionEnum {
    EMPTY,
    NONE,
    ON_FAILURE,
    ANY,
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[napi]
pub enum TaskState {
    NEW,
    ALLOCATED,
    PENDING,
    ASSIGNED,
    ACCEPTED,
    PREPARING,
    READY,
    STARTING,
    RUNNING,
    COMPLETE,
    SHUTDOWN,
    FAILED,
    REJECTED,
    REMOVE,
    ORPHANED,
}

/// represents the status of a task.
#[napi(object)]
pub struct TaskStatus {
    #[napi(js_name = "Timestamp")]
    pub timestamp: Option<String>,

    #[napi(js_name = "State")]
    pub state: Option<TaskState>,

    #[napi(js_name = "Message")]
    pub message: Option<String>,

    #[napi(js_name = "Err")]
    pub err: Option<String>,

    #[napi(js_name = "ContainerStatus")]
    pub container_status: Option<ContainerStatus>,

    #[napi(js_name = "PortStatus")]
    pub port_status: Option<PortStatus>,
}

#[derive(o2o)]
#[owned_into(bollard::secret::ThrottleDevice)]
#[napi(object)]
pub struct ThrottleDevice {
    /// Device path
    #[napi(js_name = "Path")]
    pub path: Option<String>,

    /// Rate
    #[napi(js_name = "Rate")]
    pub rate: Option<i64>,
}

/// Information about the issuer of leaf TLS certificates and the trusted root CA certificate.
#[napi(object)]
pub struct TlsInfo {
    /// The root CA certificate(s) that are used to validate leaf TLS certificates.
    #[napi(js_name = "TrustRoot")]
    pub trust_root: Option<String>,

    /// The base64-url-safe-encoded raw subject bytes of the issuer.
    #[napi(js_name = "CertIssuerSubject")]
    pub cert_issuer_subject: Option<String>,

    /// The base64-url-safe-encoded raw public key bytes of the issuer.
    #[napi(js_name = "CertIssuerPublicKey")]
    pub cert_issuer_public_key: Option<String>,
}

#[napi(object)]
pub struct UnlockKeyResponse {
    /// The swarm's unlock key.
    #[napi(js_name = "UnlockKey")]
    pub unlock_key: Option<String>,
}

#[napi(object)]
pub struct Volume {
    /// Name of the volume.
    #[napi(js_name = "Name")]
    pub name: String,

    /// Name of the volume driver used by the volume.
    #[napi(js_name = "Driver")]
    pub driver: String,

    /// Mount path of the volume on the host.
    #[napi(js_name = "Mountpoint")]
    pub mountpoint: String,

    /// Date/Time the volume was created.
    #[napi(js_name = "CreatedAt")]
    pub created_at: Option<String>,

    /// Low-level details about the volume, provided by the volume driver. Details are returned as a map with key/value pairs: `{'key':'value','key2':'value2'}`.  The `Status` field is optional, and is omitted if the volume driver does not support this feature.
    #[napi(js_name = "Status")]
    pub status: Option<Vec<String>>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: HashMap<String, String>,

    /// The level at which the volume exists. Either `global` for cluster-wide, or `local` for machine level.
    #[napi(js_name = "Scope")]
    pub scope: Option<VolumeScopeEnum>,

    #[napi(js_name = "ClusterVolume")]
    pub cluster_volume: Option<ClusterVolume>,

    /// The driver specific options used when creating the volume.
    #[napi(js_name = "Options")]
    pub options: HashMap<String, String>,

    #[napi(js_name = "UsageData")]
    pub usage_data: Option<VolumeUsageData>,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum VolumeScopeEnum {
    EMPTY,
    LOCAL,
    GLOBAL,
}

/// Volume configuration
#[napi(object)]
pub struct VolumeCreateOptions {
    /// The new volume's name. If not specified, Docker generates a name.
    #[napi(js_name = "Name")]
    pub name: Option<String>,

    /// Name of the volume driver to use.
    #[napi(js_name = "Driver")]
    pub driver: Option<String>,

    /// A mapping of driver options and values. These options are passed directly to the driver and are driver specific.
    #[napi(js_name = "DriverOpts")]
    pub driver_opts: Option<HashMap<String, String>>,

    /// User-defined key/value metadata.
    #[napi(js_name = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    #[napi(js_name = "ClusterVolumeSpec")]
    pub cluster_volume_spec: Option<ClusterVolumeSpec>,
}

/// Volume list response
#[napi(object)]
pub struct VolumeListResponse {
    /// List of volumes
    #[napi(js_name = "Volumes")]
    pub volumes: Option<Vec<Volume>>,

    /// Warnings that occurred when fetching the list of volumes.
    #[napi(js_name = "Warnings")]
    pub warnings: Option<Vec<String>>,
}

#[napi(object)]
pub struct VolumePruneResponse {
    /// Volumes that were deleted
    #[napi(js_name = "VolumesDeleted")]
    pub volumes_deleted: Option<Vec<String>>,

    /// Disk space reclaimed in bytes
    #[napi(js_name = "SpaceReclaimed")]
    pub space_reclaimed: Option<i64>,
}

/// Usage details about the volume. This information is used by the `GET /system/df` endpoint, and omitted in other endpoints.
#[napi(object)]
pub struct VolumeUsageData {
    /// Amount of disk space used by the volume (in bytes). This information is only available for volumes created with the `'local'` volume driver. For volumes created with other volume drivers, this field is set to `-1` ('not available')
    #[napi(js_name = "Size")]
    pub size: i64,

    /// The number of containers referencing this volume. This field is set to `-1` if the reference-count is not available.
    #[napi(js_name = "RefCount")]
    pub ref_count: i64,
}
