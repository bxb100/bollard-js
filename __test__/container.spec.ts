import anyTest, { TestFn } from 'ava'
import { Container, Docker } from '../index'
import * as fs from 'node:fs'

const test = anyTest as TestFn<{
  container: {
    docker: Docker
    container: Container
  }
}>

test.before('create_container', async (t) => {
  const docker = new Docker()
  // docker container create -i -t --name mycontainer alpine
  const { container } = await docker.createContainer(
    {
      name: 'mycontainer',
    },
    {
      Image: 'alpine',
      OpenStdin: true,
      Tty: true,
    },
  )
  t.context.container = {
    docker,
    container,
  }
})

test.serial('start', async (t) => {
  const { container } = t.context.container
  await container.start()
  t.pass()
})

test.serial('attach', async (t) => {
  const { container } = t.context.container
  const output = await container.attach({
    stdin: true,
    stdout: true,
    stderr: true,
    stream: true,
    logs: true,
  })
  const readable = output.createReadStream()
  const writable = output.createWriteStream()

  let data = ''
  readable.on('data', (chunk: Buffer) => {
    data += chunk.toString()
  })

  const res: string = await new Promise((resolve) => {
    writable.on('close', () => {
      resolve(data)
    })
    writable.write('echo hello world\n')
    setTimeout(() => writable.end(), 500)
  })

  if (res.includes('hello world')) {
    t.pass()
  } else {
    console.log('failed:: ', res)
    t.fail()
  }
})

test.serial('inspect', async (t) => {
  const { container } = t.context.container
  const res = await container.inspect({
    size: true,
  })
  t.truthy(res.SizeRootFs)
  t.truthy(res.SizeRw)
})

test.serial('rename', async (t) => {
  const { container } = t.context.container
  await container.rename('mycontainer_new')
  t.pass()
})

test.serial('update', async (t) => {
  const { container } = t.context.container
  await container.update({
    CpuShares: 512,
    KernelMemoryTCP: 1024 * 1024,
  })
  t.pass()
})

test.serial('top', async (t) => {
  const { container } = t.context.container

  const { Titles } = await container.top()
  t.truthy(Titles?.includes('PID'))
})

test.serial('changes', async (t) => {
  const { container } = t.context.container

  const res = await container.changes()
  t.truthy(res)
})

test.serial('export', async (t) => {
  const { container } = t.context.container

  const path = './__test__/cs.tar'
  await container.export(path)
  t.true(fs.existsSync(path))
  fs.rmSync(path)
})

test.serial('stop', async (t) => {
  const { container } = t.context.container

  await container.stop()
  await container.start()
  t.pass()
})

test.serial('pause - unpause', async (t) => {
  const { container } = t.context.container

  await container.pause()
  await container.unpause()
  t.pass()
})

// todo: move to image
test.serial('commit', async (t) => {
  const { container } = t.context.container

  const { ID, Expected } = await container.commit(
    {
      repo: 'alpine',
      tag: 'version1',
      comment: 'build for test',
      author: 'test',
      pause: true,
    },
    {},
  )

  // fixme: if image exist will return undefined
  console.log(ID, Expected)
  t.pass()
})

test.after('remove_container', async (t) => {
  const { container } = t.context.container

  await container.remove({
    force: true,
    v: false,
    link: false,
  })
})
