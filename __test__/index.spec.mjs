import test from 'ava'
import { Docker } from '../index.js'

test('version', async (t) => {
  const docker = new Docker()
  const version = await docker.version()
  const v = JSON.parse(version.toString())
  // console.log(v)
  t.is(typeof v, 'object')
})

test('attach', async (t) => {
    const docker = new Docker()
    const output = await docker.attach(process.env.DOCKER_CONTAINER_ID, {
        stdin: true,
        stdout: true,
        stderr: true,
        stream: true,
        logs: true
    })
    console.log(process.stdout.writableHighWaterMark);
    output.createReadStream().pipe(process.stdout);
    let write = output.createWriteStream();
    await new Promise((resolve, reject) => {
        let b = write.write("hello world\n", () => {
            resolve()
        })
        console.log("result:" , b);
    })
    t.pass()
})
