import test from 'ava'
import { Docker } from '../index'

test('version', async (t) => {
  const docker = new Docker()
  const version = await docker.version()
  const v = JSON.parse(version.toString())
  // console.log(v)
  t.truthy(v)
  t.truthy(v.Version)
})
