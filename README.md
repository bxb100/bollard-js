<h1 align="center"><sup>Bollard</sup></h1>
<p align="center">
Another Docker client for Node.js.
<br>
<br>
<a href="https://github.com/bxb100/bollard-js/actions/workflows/CI.yml">
<img src="https://github.com/bxb100/bollard-js/actions/workflows/CI.yml/badge.svg" alt="">
</a>
<a href="https://www.npmjs.com/package/bollard">
<img src="https://img.shields.io/npm/v/bollard"  alt=""/>
</a>
</p>

## TODO

### Docker[^1]

- [x] docker.createContainer(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerCreate)
- [x] docker.createImage([auth], options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageCreate)
- [ ] docker.loadImage(file, options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageLoad)
- [ ] docker.importImage(file, options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageCreate)
- [ ] docker.buildImage(file, options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageBuild)
- [ ] docker.checkAuth(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SystemAuth)
- [x] docker.getContainer(id) - Returns a Container object.
- [ ] docker.getImage(name) - Returns an Image object.
- [ ] docker.getVolume(name) - Returns a Volume object.
- [ ] docker.getPlugin(name) - Returns a Plugin object.
- [ ] docker.getService(id) - Returns a Service object.
- [ ] docker.getTask(id) - Returns a Task object.
- [ ] docker.getNode(id) - Returns a Node object.
- [ ] docker.getNetwork(id) - Returns a Network object.
- [ ] docker.getSecret(id) - Returns a Secret object.
- [ ] docker.getConfig(id) - Returns a Config object.
- [ ] docker.getExec(id) - Returns a Exec object.
- [ ] docker.listContainers(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerList)
- [ ] docker.listImages(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageList)
- [ ] docker.listServices(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ServiceList)
- [ ] docker.listNodes(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NodeList)
- [ ] docker.listTasks(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/TaskList)
- [ ] docker.listSecrets(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SecretList)
- [ ] docker.listConfigs(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ConfigList)
- [ ] docker.listPlugins(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginList)
- [ ] docker.listVolumes(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/VolumeList)
- [ ] docker.listNetworks(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NetworkList)
- [ ] docker.createSecret(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SecretCreate)
- [ ] docker.createConfig(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ConfigCreate)
- [ ] docker.createPlugin(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginCreate)
- [ ] docker.createVolume(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/VolumeCreate)
- [ ] docker.createService(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ServiceCreate)
- [ ] docker.createNetwork(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NetworkCreate)
- [ ] docker.pruneImages(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImagePrune)
- [ ] docker.pruneBuilder() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/BuildPrune)
- [ ] docker.pruneContainers(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerPrune)
- [ ] docker.pruneVolumes(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/VolumePrune)
- [ ] docker.pruneNetworks(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NetworkPrune)
- [ ] docker.searchImages(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageSearch)
- [ ] docker.info() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SystemInfo)
- [x] docker.version() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SystemVersion)
- [ ] docker.ping() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SystemPing)
- [ ] docker.df() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SystemDataUsage)
- [ ] docker.getEvents(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SystemEvents)
- [ ] docker.swarmInit(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SwarmInit)
- [ ] docker.swarmJoin(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SwarmJoin)
- [ ] docker.swarmLeave(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SwarmLeave)
- [ ] docker.swarmUpdate(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SwarmUpdate)
- [ ] docker.swarmInspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SwarmInspect)
- [ ] docker.pull(repoTag, options, callback, auth) - Like Docker's CLI pull
- [ ] docker.pullAll(repoTag, options, callback, auth) - Like Docker's CLI pull with "-a"
- [ ] docker.run(image, cmd, stream, createOptions, startOptions) - Like Docker's CLI run

### Container

- [x] container.inspect(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerInspect)
- [x] container.rename(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerRename)
- [x] container.update(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerUpdate)
- [x] container.top(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerTop)
- [x] container.changes() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerChanges)
- [x] container.export() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerExport)
- [x] container.start(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerStart)
- [x] container.stop(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerStop)
- [x] container.pause(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerPause)
- [x] container.unpause(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerUnpause)
- [x] container.exec(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerExec)
- [x] container.commit(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageCommit)
- [x] container.restart(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerRestart)
- [x] container.kill(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerKill)
- [x] container.resize(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerResize)
- [x] container.attach(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerAttach)
- [x] container.wait(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerWait)
- [x] container.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerDelete)
- [x] container.getArchive(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerArchive)
- [ ] ~~container.infoArchive(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerArchiveInfo)~~
- [x] container.putArchive(file, options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PutContainerArchive)
- [x] container.logs(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerLogs)
- [x] container.stats(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ContainerStats)

### Exec

- [x] exec.start(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ExecStart)
- [x] exec.resize(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ExecResize)
- [x] exec.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ExecInspect)

### Image

- [x] image.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageInspect)
- [x] image.history() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageHistory)
- [x] image.push(options, callback, auth) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImagePush)
- [x] image.tag(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageTag)
- [x] image.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageDelete)
- [x] image.get() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ImageGet)

### Network

- [ ] network.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NetworkInspect)
- [ ] network.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NetworkDelete)
- [ ] network.connect(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NetworkConnect)
- [ ] network.disconnect(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NetworkDisconnect)

### Node

- [ ] node.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NodeInspect)
- [ ] node.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NodeDelete)
- [ ] node.update(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/NodeUpdate)

### Plugin

- [ ] plugin.privileges() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/GetPluginPrivileges)
- [ ] plugin.pull(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginPull)
- [ ] plugin.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginInspect)
- [ ] plugin.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginDelete)
- [ ] plugin.enable(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginEnable)
- [ ] plugin.disable(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginDisable)
- [ ] plugin.update([auth], options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginUpgrade)
- [ ] plugin.push(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginPush)
- [ ] plugin.configure(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/PluginSet)

### Secret

- [ ] secret.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SecretInspect)
- [ ] secret.remove() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SecretDelete)
- [ ] secret.update(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/SecretUpdate)

### Service

- [ ] service.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ServiceInspect)
- [ ] service.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ServiceDelete)
- [ ] service.update(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ServiceUpdate)
- [ ] service.logs(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/ServiceLogs)

### Task

- [ ] task.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/TaskInspect)
- [ ] task.logs(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/Session)

### Volume

- [ ] volume.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/VolumeInspect)
- [ ] volume.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.47/#operation/VolumeDelete)

### Other

- [ ] test using mock
- [ ] test with ssl
- [ ] test with http
- [ ] test with socket in windows
- [x] test attach
- [ ] figure out is attached write only work with enable `stream`
- [ ] fix CI

[^1]: https://github.com/apocas/dockerode
