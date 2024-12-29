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

test('attach', async (t) => {
  // todo: start then attach !!!
  // const { container } = t.context
  //
  // const output = await container.attach({
  //   stdin: true,
  //   stdout: true,
  //   stderr: true,
  //   stream: true,
  //   logs: true
  // })
  // const readable = output.createReadStream()
  // const writable = output.createWriteStream()
  //
  // readable.pipe(process.stdout);
  // writable.write('echo hello world\n')
  t.pass()
})

test.after('remove_container', async (t) => {
  const { container } = t.context

  await container.remove({
    force: true,
    v: false,
    link: false,
  })
})
