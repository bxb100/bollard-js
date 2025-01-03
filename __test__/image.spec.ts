import test from 'ava'
import { Docker } from '../index'
import { iterLine } from './common'
import * as fs from 'node:fs'

const IMAGE = 'hello-world'

test.serial('create_image', async (t) => {
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

test.serial('inspect_image', async (t) => {
  const docker = new Docker()
  const image = docker.getImage(IMAGE)
  const inspect = await image.inspect()

  t.truthy(inspect.Id)
})

test.serial('image_history', async (t) => {
  const docker = new Docker()
  const image = docker.getImage(IMAGE)
  const history = await image.history()

  // at least one have Imported from -
  t.true(history.length > 0)
})
