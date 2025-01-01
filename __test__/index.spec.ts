import test from 'ava'
import { Docker } from '../index'

test('version', async (t) => {
  const docker = new Docker()
  const v = await docker.version()
  // console.log(v)
  t.truthy(v)
  t.truthy(v.Version)
})
