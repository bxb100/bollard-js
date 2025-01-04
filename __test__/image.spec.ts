import test from 'ava'
import { Docker } from '../index'
import { iterLine } from './common'
import * as fs from 'node:fs'

const IMAGE = 'hello-world'

test.serial('image create', async (t) => {
  const docker = new Docker()
  // build by `docker export hello-world > hello-world.tar`
  // official image see https://hub.docker.com/_/hello-world
  const tar = fs.readFileSync('__test__/fixtures/hello-world.tar')
  const createImageOutput = docker.createImage(
    {
      fromImage: '',
      tag: 'latest',
      fromSrc: '-',
      platform: '',
      repo: IMAGE,
      changes: ['CMD ["./hello"]'],
    },
    tar,
  )

  let res: object | undefined
  for await (const line of iterLine(createImageOutput)) {
    res = JSON.parse(line)
    // console.log(res)
  }

  t.truthy(res)
})

test.serial('image inspect', async (t) => {
  const docker = new Docker()
  const image = docker.getImage(IMAGE)
  const inspect = await image.inspect()

  t.truthy(inspect.Id)
})

test.serial('image history', async (t) => {
  const docker = new Docker()
  const image = docker.getImage(IMAGE)
  const history = await image.history()

  // at least one have Imported from -
  t.true(history.length > 0)
})

test.serial('image tag', async (t) => {
  const docker = new Docker()
  const image = docker.getImage(IMAGE)
  // Run `docker run -d --restart always --name registry -p 5000:5000 registry:2` first
  await image.tag({
    repo: 'localhost:5000/hello-world',
    tag: 'latest',
  })

  t.pass()
})

test.serial('image push', async (t) => {
  const docker = new Docker()
  const image = docker.getImage('localhost:5000/hello-world')

  const pushImageInfoStream = image.push({
    tag: 'latest',
  })

  let res: object | undefined
  for await (const line of iterLine(pushImageInfoStream)) {
    res = JSON.parse(line)
    // console.log(res)
  }

  t.truthy(res)
})

test.serial('image get', async (t) => {
  const docker = new Docker()
  const image = docker.getImage('localhost:5000/hello-world')

  const stream = image.get()
  const file = './__test__/fixtures/test-img-hello-world.tar'
  await stream.save(file)
  t.true(fs.existsSync(file))
  fs.rmSync(file)
})

// test.serial('image remove', async (t) => {
//   const docker = new Docker()
//   const image = docker.getImage(IMAGE)
//
//   await image.remove()
//   t.pass()
// })
