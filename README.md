## TODO

### Docker[^1]

- [x] docker.createContainer(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerCreate)
- [ ] docker.createImage([auth], options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageCreate)
- [ ] docker.loadImage(file, options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageLoad)
- [ ] docker.importImage(file, options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageCreate)
- [ ] docker.buildImage(file, options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageBuild)
- [ ] docker.checkAuth(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SystemAuth)
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
- [ ] docker.listContainers(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerList)
- [ ] docker.listImages(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageList)
- [ ] docker.listServices(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ServiceList)
- [ ] docker.listNodes(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NodeList)
- [ ] docker.listTasks(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/TaskList)
- [ ] docker.listSecrets(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SecretList)
- [ ] docker.listConfigs(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ConfigList)
- [ ] docker.listPlugins(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginList)
- [ ] docker.listVolumes(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/VolumeList)
- [ ] docker.listNetworks(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NetworkList)
- [ ] docker.createSecret(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SecretCreate)
- [ ] docker.createConfig(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ConfigCreate)
- [ ] docker.createPlugin(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginCreate)
- [ ] docker.createVolume(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/VolumeCreate)
- [ ] docker.createService(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ServiceCreate)
- [ ] docker.createNetwork(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NetworkCreate)
- [ ] docker.pruneImages(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImagePrune)
- [ ] docker.pruneBuilder() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/BuildPrune)
- [ ] docker.pruneContainers(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerPrune)
- [ ] docker.pruneVolumes(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/VolumePrune)
- [ ] docker.pruneNetworks(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NetworkPrune)
- [ ] docker.searchImages(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageSearch)
- [ ] docker.info() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SystemInfo)
- [x] docker.version() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SystemVersion)
- [ ] docker.ping() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SystemPing)
- [ ] docker.df() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SystemDataUsage)
- [ ] docker.getEvents(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SystemEvents)
- [ ] docker.swarmInit(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SwarmInit)
- [ ] docker.swarmJoin(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SwarmJoin)
- [ ] docker.swarmLeave(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SwarmLeave)
- [ ] docker.swarmUpdate(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SwarmUpdate)
- [ ] docker.swarmInspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SwarmInspect)
- [ ] docker.pull(repoTag, options, callback, auth) - Like Docker's CLI pull
- [ ] docker.pullAll(repoTag, options, callback, auth) - Like Docker's CLI pull with "-a"
- [ ] docker.run(image, cmd, stream, createOptions, startOptions) - Like Docker's CLI run

### Container

- [ ] container.inspect(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerInspect)
- [ ] container.rename(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerRename)
- [ ] container.update(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerUpdate)
- [ ] container.top(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerTop)
- [ ] container.changes() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerChanges)
- [ ] container.export() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerExport)
- [ ] container.start(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerStart)
- [ ] container.stop(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerStop)
- [ ] container.pause(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerPause)
- [ ] container.unpause(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerUnpause)
- [ ] container.exec(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerExec)
- [ ] container.commit(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageCommit)
- [ ] container.restart(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerRestart)
- [ ] container.kill(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerKill)
- [ ] container.resize(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerResize)
- [x] container.attach(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerAttach)
- [ ] container.wait(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerWait)
- [ ] container.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerDelete)
- [ ] container.getArchive(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerArchive)
- [ ] container.infoArchive(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerArchiveInfo)
- [ ] container.putArchive(file, options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PutContainerArchive)
- [ ] container.logs(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerLogs)
- [ ] container.stats(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ContainerStats)

### Exec

- [ ] exec.start(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ExecStart)
- [ ] exec.resize(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ExecResize)
- [ ] exec.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ExecInspect)

### Image

- [ ] image.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageInspect)
- [ ] image.history() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageHistory)
- [ ] image.push(options, callback, auth) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImagePush)
- [ ] image.tag(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageTag)
- [ ] image.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageDelete)
- [ ] image.get() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ImageGet)

### Network

- [ ] network.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NetworkInspect)
- [ ] network.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NetworkDelete)
- [ ] network.connect(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NetworkConnect)
- [ ] network.disconnect(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NetworkDisconnect)

### Node

- [ ] node.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NodeInspect)
- [ ] node.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NodeDelete)
- [ ] node.update(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/NodeUpdate)

### Plugin

- [ ] plugin.privileges() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/GetPluginPrivileges)
- [ ] plugin.pull(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginPull)
- [ ] plugin.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginInspect)
- [ ] plugin.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginDelete)
- [ ] plugin.enable(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginEnable)
- [ ] plugin.disable(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginDisable)
- [ ] plugin.update([auth], options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginUpgrade)
- [ ] plugin.push(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginPush)
- [ ] plugin.configure(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/PluginSet)

### Secret

- [ ] secret.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SecretInspect)
- [ ] secret.remove() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SecretDelete)
- [ ] secret.update(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/SecretUpdate)

### Service

- [ ] service.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ServiceInspect)
- [ ] service.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ServiceDelete)
- [ ] service.update(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ServiceUpdate)
- [ ] service.logs(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/ServiceLogs)

### Task

- [ ] task.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/TaskInspect)
- [ ] task.logs(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/Session)

### Volume

- [ ] volume.inspect() - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/VolumeInspect)
- [ ] volume.remove(options) - [Docker API Endpoint](https://docs.docker.com/engine/api/v1.37/#operation/VolumeDelete)

### Other

- [ ] test using mock
- [ ] test with ssl
- [ ] test with http
- [ ] test with socket in windows
- [ ] test attach
- [ ] attach write only work with enable `stream`
- [ ] fix CI

[^1]: https://github.com/apocas/dockerode
