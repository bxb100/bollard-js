import anyTest, { TestFn } from 'ava'
import { Container, Docker } from '../index.js'

const test = anyTest as TestFn<{ docker: Docker; container: Container }>

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
  t.context = {
    docker,
    container,
  }
})

// todo: move this to other test
test('version', async (t) => {
  const { docker } = t.context
  const version = await docker.version()
  const v = JSON.parse(version.toString())
  // console.log(v)
  t.true(v !== null && v !== undefined)
  t.true(v.Version !== undefined)
})

test.serial('start', async (t) => {
  const { container } = t.context
  await container.start()
  t.pass()
})

test.serial('attach', async (t) => {
  const { container } = t.context
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
  const { container } = t.context
  const res = await container.inspect({
    size: true,
  })
  t.truthy(res.SizeRootFs)
  t.truthy(res.SizeRw)
})

test.serial('rename', async (t) => {
  const { container } = t.context
  await container.rename('mycontainer_new')
  t.pass()
})

test.serial('update', async (t) => {
  const { container } = t.context
  await container.update({
    CpuShares: 512,
    KernelMemoryTCP: 1024 * 1024,
  })
  t.pass()
})

test.serial('top', async (t) => {
  const { container } = t.context

  const { Titles } = await container.top()
  t.truthy(Titles?.includes("PID"))
})

test.after('remove_container', async (t) => {
  const { container } = t.context

  await container.remove({
    force: true,
    v: false,
    link: false,
  })
})
