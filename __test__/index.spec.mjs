import test from 'ava'

import { Docker } from '../index.js'

test('version', async (t) => {
  const docker = new Docker()
  const version = await docker.version()
  const v = JSON.parse(version.toString())
  t.is(typeof v, 'object')
})
