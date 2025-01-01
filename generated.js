/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'bollard.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./bollard.android-arm64.node')
          } else {
            nativeBinding = require('@bollard/lib-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'bollard.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./bollard.android-arm-eabi.node')
          } else {
            nativeBinding = require('@bollard/lib-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'bollard.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./bollard.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@bollard/lib-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'bollard.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./bollard.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@bollard/lib-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'bollard.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./bollard.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@bollard/lib-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'bollard.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./bollard.darwin-universal.node')
      } else {
        nativeBinding = require('@bollard/lib-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'bollard.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./bollard.darwin-x64.node')
          } else {
            nativeBinding = require('@bollard/lib-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'bollard.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./bollard.darwin-arm64.node')
          } else {
            nativeBinding = require('@bollard/lib-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'bollard.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./bollard.freebsd-x64.node')
      } else {
        nativeBinding = require('@bollard/lib-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'bollard.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./bollard.linux-x64-musl.node')
            } else {
              nativeBinding = require('@bollard/lib-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'bollard.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./bollard.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@bollard/lib-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'bollard.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./bollard.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@bollard/lib-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'bollard.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./bollard.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@bollard/lib-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'bollard.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./bollard.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('@bollard/lib-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'bollard.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./bollard.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('@bollard/lib-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'bollard.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./bollard.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('@bollard/lib-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'bollard.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./bollard.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('@bollard/lib-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'bollard.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./bollard.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('@bollard/lib-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { CreateContainerResponse, CreateExecResults, LogsResponse, StatsStream, Container, ReadStream, Exec, BuildCacheTypeEnum, ChangeType, ClusterVolumePublishStatusStateEnum, ClusterVolumeSpecAccessModeScopeEnum, ClusterVolumeSpecAccessModeSharingEnum, ClusterVolumeSpecAccessModeAvailabilityEnum, ContainerStateStatusEnum, EndpointPortConfigProtocolEnum, EndpointPortConfigPublishModeEnum, EndpointSpecModeEnum, EventMessageTypeEnum, EventMessageScopeEnum, HealthStatusEnum, HostConfigCgroupnsModeEnum, HostConfigIsolationEnum, ImageManifestSummaryKindEnum, LocalNodeState, MountTypeEnum, MountBindOptionsPropagationEnum, MountPointTypeEnum, NodeSpecRoleEnum, NodeSpecAvailabilityEnum, NodeState, PluginConfigInterfaceProtocolSchemeEnum, PortTypeEnum, Reachability, RestartPolicyNameEnum, ServiceSpecRollbackConfigFailureActionEnum, ServiceSpecRollbackConfigOrderEnum, ServiceSpecUpdateConfigFailureActionEnum, ServiceSpecUpdateConfigOrderEnum, ServiceUpdateStatusStateEnum, SwarmSpecCaConfigExternalCasProtocolEnum, SystemInfoCgroupDriverEnum, SystemInfoCgroupVersionEnum, SystemInfoIsolationEnum, TaskSpecContainerSpecIsolationEnum, TaskSpecContainerSpecPrivilegesAppArmorModeEnum, TaskSpecContainerSpecPrivilegesSeccompModeEnum, TaskSpecRestartPolicyConditionEnum, TaskState, VolumeScopeEnum, Output, Docker } = nativeBinding

module.exports.CreateContainerResponse = CreateContainerResponse
module.exports.CreateExecResults = CreateExecResults
module.exports.LogsResponse = LogsResponse
module.exports.StatsStream = StatsStream
module.exports.Container = Container
module.exports.ReadStream = ReadStream
module.exports.Exec = Exec
module.exports.BuildCacheTypeEnum = BuildCacheTypeEnum
module.exports.ChangeType = ChangeType
module.exports.ClusterVolumePublishStatusStateEnum = ClusterVolumePublishStatusStateEnum
module.exports.ClusterVolumeSpecAccessModeScopeEnum = ClusterVolumeSpecAccessModeScopeEnum
module.exports.ClusterVolumeSpecAccessModeSharingEnum = ClusterVolumeSpecAccessModeSharingEnum
module.exports.ClusterVolumeSpecAccessModeAvailabilityEnum = ClusterVolumeSpecAccessModeAvailabilityEnum
module.exports.ContainerStateStatusEnum = ContainerStateStatusEnum
module.exports.EndpointPortConfigProtocolEnum = EndpointPortConfigProtocolEnum
module.exports.EndpointPortConfigPublishModeEnum = EndpointPortConfigPublishModeEnum
module.exports.EndpointSpecModeEnum = EndpointSpecModeEnum
module.exports.EventMessageTypeEnum = EventMessageTypeEnum
module.exports.EventMessageScopeEnum = EventMessageScopeEnum
module.exports.HealthStatusEnum = HealthStatusEnum
module.exports.HostConfigCgroupnsModeEnum = HostConfigCgroupnsModeEnum
module.exports.HostConfigIsolationEnum = HostConfigIsolationEnum
module.exports.ImageManifestSummaryKindEnum = ImageManifestSummaryKindEnum
module.exports.LocalNodeState = LocalNodeState
module.exports.MountTypeEnum = MountTypeEnum
module.exports.MountBindOptionsPropagationEnum = MountBindOptionsPropagationEnum
module.exports.MountPointTypeEnum = MountPointTypeEnum
module.exports.NodeSpecRoleEnum = NodeSpecRoleEnum
module.exports.NodeSpecAvailabilityEnum = NodeSpecAvailabilityEnum
module.exports.NodeState = NodeState
module.exports.PluginConfigInterfaceProtocolSchemeEnum = PluginConfigInterfaceProtocolSchemeEnum
module.exports.PortTypeEnum = PortTypeEnum
module.exports.Reachability = Reachability
module.exports.RestartPolicyNameEnum = RestartPolicyNameEnum
module.exports.ServiceSpecRollbackConfigFailureActionEnum = ServiceSpecRollbackConfigFailureActionEnum
module.exports.ServiceSpecRollbackConfigOrderEnum = ServiceSpecRollbackConfigOrderEnum
module.exports.ServiceSpecUpdateConfigFailureActionEnum = ServiceSpecUpdateConfigFailureActionEnum
module.exports.ServiceSpecUpdateConfigOrderEnum = ServiceSpecUpdateConfigOrderEnum
module.exports.ServiceUpdateStatusStateEnum = ServiceUpdateStatusStateEnum
module.exports.SwarmSpecCaConfigExternalCasProtocolEnum = SwarmSpecCaConfigExternalCasProtocolEnum
module.exports.SystemInfoCgroupDriverEnum = SystemInfoCgroupDriverEnum
module.exports.SystemInfoCgroupVersionEnum = SystemInfoCgroupVersionEnum
module.exports.SystemInfoIsolationEnum = SystemInfoIsolationEnum
module.exports.TaskSpecContainerSpecIsolationEnum = TaskSpecContainerSpecIsolationEnum
module.exports.TaskSpecContainerSpecPrivilegesAppArmorModeEnum = TaskSpecContainerSpecPrivilegesAppArmorModeEnum
module.exports.TaskSpecContainerSpecPrivilegesSeccompModeEnum = TaskSpecContainerSpecPrivilegesSeccompModeEnum
module.exports.TaskSpecRestartPolicyConditionEnum = TaskSpecRestartPolicyConditionEnum
module.exports.TaskState = TaskState
module.exports.VolumeScopeEnum = VolumeScopeEnum
module.exports.Output = Output
module.exports.Docker = Docker
