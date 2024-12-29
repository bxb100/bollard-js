import anyTest, {TestFn} from 'ava';
import { Docker } from '../index.js'

const test = anyTest as TestFn<{ docker: Docker; containerId: string }>;


test('version', async (t) => {
  // const docker = new Docker()
  // const version = await docker.version()
  // const v = JSON.parse(version.toString())
  // console.log(v)
  // t.is(typeof v, 'object')
  t.pass()
})

test('attach', async (t) => {
  // const docker = new Docker()
  // const output = await docker.attach(process.env.DOCKER_CONTAINER_ID, {
  //     stdin: true,
  //     stdout: true,
  //     stderr: true,
  //     stream: true,
  //     logs: true
  // })
  // console.log(process.stdout.writableHighWaterMark);
  // output.createReadStream().pipe(process.stdout);
  // let write = output.createWriteStream();
  // await new Promise((resolve, reject) => {
  //     let b = write.write("hello world\n", () => {
  //         resolve()
  //     })
  //     console.log("result:" , b);
  // })
  t.pass()
})

test.before('create_container', async (t) => {
  const docker = new Docker()
  // docker container create -i -t --name mycontainer alpine
  const { Id } = await docker.createContainer(
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
    containerId: Id,
  }
})

test.after('remove_container', async (t) => {
  const { containerId } = t.context
  console.log(containerId)
})
