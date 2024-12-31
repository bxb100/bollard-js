import { Readable, ReadableOptions, Writable, WritableOptions } from 'node:stream'

declare module './generated' {
  interface Output {
    createReadStream(options?: ReadableOptions): Readable

    createWriteStream(options?: WritableOptions): Writable
  }

  interface DownloadStream {
    save(path: string, options?: ReadableOptions): Promise<void>
  }
}

export * from './generated'
