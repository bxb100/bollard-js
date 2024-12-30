import { Readable, ReadableOptions, Writable, WritableOptions } from 'node:stream'

declare module './generated' {
  interface Output {
    createReadStream(options?: ReadableOptions): Readable

    createWriteStream(options?: WritableOptions): Writable
  }
}

export * from './generated'
