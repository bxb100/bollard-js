import anyTest, { TestFn } from 'ava'
import { Container, Docker, Exec } from '../index.js'

const test = anyTest as TestFn<{
  exec: {
    docker: Docker
    container: Container
    exec?: Exec
  }
}>

// https://docs.docker.com/reference/cli/docker/container/exec/
test.before('create_container', async (t) => {
  const docker = new Docker()
  // docker run --name mycontainer -d -i -t alpine /bin/sh
  const { container } = await docker.createContainer(
    {
      name: 'mycontainer_exec',
    },
    {
      Image: 'alpine',
      Tty: true,
      Cmd: ['/bin/sh'],
    },
  )
  await container.start()

  t.context.exec = {
    docker,
    container,
  }
})

test.serial('touch /tmp/execWorks', async (t) => {
  const { container } = t.context.exec

  // docker exec -d mycontainer touch /tmp/execWorks
  const { exec } = await container.exec({
    AttachStdin: true,
    AttachStdout: true,
    AttachStderr: true,
    Tty: true,
    Cmd: ['touch', '/tmp/execWorks'],
  })
  await exec.start()

  t.pass()
})

test.serial('ls /tmp/execWorks', async (t) => {
  const { container } = t.context.exec

  // docker exec -it mycontainer sh
  const { exec } = await container.exec({
    AttachStdin: true,
    AttachStdout: true,
    AttachStderr: true,
    Tty: true,
    Cmd: ['sh'],
  })
  const output = await exec.start()

  // maybe useful?
  await exec.resize({
    h: 1,
    w: 0,
  })

  const read = output!.createReadStream()
  const write = output!.createWriteStream()

  write.write('ls /tmp/execWorks\n')
  write.destroy()

  const chunks: Buffer[] = []

  read.on('readable', () => {
    let chunk
    while (null !== (chunk = read.read())) {
      chunks.push(chunk)
    }

    if ((chunks.join('').match(/execWorks/g)?.length || 0) > 1) {
      read.destroy()
    }
  })

  await new Promise((resolve) => {
    read.on('close', () => resolve(true))
  })

  t.context.exec.exec = exec

  t.pass()
})

test.serial('inspect', async (t) => {
  const { exec } = t.context.exec

  const info = await exec!.inspect()

  t.truthy(info.Running)
})

test.after('remove_container', async (t) => {
  const { container } = t.context.exec

  await container.remove({ v: true, force: true, link: false })
})
