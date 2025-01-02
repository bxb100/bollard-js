import test from 'ava'
import { Docker } from '../index'
import { iterLine } from './common'

test('create_image', async (t) => {
  t.timeout(120_000, 'make sure image call pull')

  const docker = new Docker()
  const createImageOutput = docker.createImage({
    fromImage: 'ghcr.io/jonashackt/hello-world:latest',
    tag: '',
    fromSrc: '',
    platform: '',
    repo: '',
    changes: [],
  })

  let res: object | undefined
  for await (const line of iterLine(createImageOutput)) {
    res = JSON.parse(line)
  }

  t.truthy(res)
})
