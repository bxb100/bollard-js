import test from 'ava'
import { Docker } from '../index'

test('version', async (t) => {
  const docker = new Docker()
  const v = await docker.version()
  // console.log(v)
  t.truthy(v)
  t.truthy(v.Version)
})

test('list containers', async (t) => {
  const docker = new Docker()
  const res = await docker.listContainers()

  // console.log(JSON.stringify(res))
  t.truthy(res)
})

test('list images', async (t) => {
  const docker = new Docker()
  const res = await docker.listImages()

  // console.log(JSON.stringify(res))
  t.truthy(res)
})
