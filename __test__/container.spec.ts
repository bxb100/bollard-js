import anyTest, { TestFn } from 'ava'
import { Container, Docker } from '../index'
import { iterLine } from './common'
import * as fs from 'node:fs'
import { exec } from 'node:child_process'
import * as util from 'node:util'

const test = anyTest as TestFn<{
  container: {
    docker: Docker
    container: Container
  }
}>

test.before('create', async (t) => {
  const docker = new Docker()
  // docker container create -i -t --name mycontainer alpine
  const { container } = await docker.createContainer(
    {
      name: 'mycontainer',
    },
    {
      Image: 'alpine',
      OpenStdin: true,
      Tty: false,
    },
  )
  t.context.container = {
    docker,
    container,
  }
})

test.beforeEach('start', async (t) => {
  const { container } = t.context.container
  await container.start()
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

test.serial('export', async (t) => {
  const { container } = t.context.container

  const path = './__test__/export.tar'
  const res = container.export()
  await res.save(path)
  t.true(fs.existsSync(path))
  fs.rmSync(path)
})

test.serial('stop', async (t) => {
  const { container } = t.context.container

  await container.stop()
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

test.serial('restart', async (t) => {
  const { container } = t.context.container

  await container.restart({ t: 2 })
  t.pass()
})

// https://www.baeldung.com/ops/docker-stop-vs-kill
test.serial('kill', async (t) => {
  const { container } = t.context.container

  await container.kill({ signal: 'SIGKILL' })
  t.pass()
})

test.serial('resize', async (t) => {
  const { container } = t.context.container

  await container.resize({ w: 50, h: 20 })
  t.pass()
})

test.serial('wait', async (t) => {
  const { container } = t.context.container

  // this test for wait block, nothing magic
  // but is it normal return null when container stop?
  let wait = container.wait({ condition: 'not-running' })
  container.stop()
  await wait
  t.pass()
})

test.serial('getArchive', async (t) => {
  const { container } = t.context.container

  const res = container.getArchive({
    path: '/opt',
  })

  const path = './__test__/opt.tar.gz'
  await res.save(path)
  t.true(fs.existsSync(path))
  fs.rmSync(path)
  t.pass()
})

test.serial('putArchive', async (t) => {
  const { container } = t.context.container

  const exec2 = util.promisify(exec)
  await exec2('/usr/bin/tar -czf ./__test__/fixtures/tarball.tar.gz ./__test__/fixtures/cs.md')

  const buffer = await fs.promises.readFile('./__test__/fixtures/tarball.tar.gz')

  await container.putArchive({ path: '/opt', noOverwriteDirNonDir: '0' }, buffer)
  // There missing decompress and check archive part
  fs.rmSync('./__test__/fixtures/tarball.tar.gz')
  t.pass()
})

test.serial('inspect', async (t) => {
  const { container } = t.context.container
  const res = await container.inspect({
    size: true,
  })
  // console.log(res)
  t.truthy(res)
})

test.serial('changes', async (t) => {
  const { container } = t.context.container

  const res = await container.changes()
  // console.log(res)
  t.truthy(res)
})

// notice Tty will cause bug https://github.com/fussybeaver/bollard/issues/492
test.serial('logs', async (t) => {
  const { container } = t.context.container

  const logsRes = container.logs({
    stdout: true,
    follow: false,
    stderr: false,
    since: 0,
    until: 0,
    timestamps: true,
    tail: 'all',
  })

  let data: any = undefined
  for await (const line of iterLine(logsRes)) {
    data = line
  }
  t.truthy(data)
})

test.serial('stats', async (t) => {
  const { container } = t.context.container

  const statsStream = container.stats({
    stream: false,
    oneShot: true,
  })

  let res: object | undefined
  for await (const line of iterLine(statsStream)) {
    res = JSON.parse(line)
  }

  t.truthy(res)
})

test.after('remove', async (t) => {
  const { container } = t.context.container

  await container.remove({
    force: true,
    v: false,
    link: false,
  })
})
