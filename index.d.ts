import { Readable, ReadableOptions, Writable, WritableOptions } from 'node:stream'

declare module './generated' {
  interface Output {
    createReadStream(options?: ReadableOptions): Readable

    createWriteStream(options?: WritableOptions): Writable
  }

  interface ReadStream {
    save(path: string, options?: ReadableOptions): Promise<void>
  }

  interface LogsResponse {
    createReadStream(options?: ReadableOptions): Readable
  }

  interface StatsStream {
    createReadStream(options?: ReadableOptions): Readable
  }
}

export * from './generated'
