import { Readable, ReadableOptions, Writable, WritableOptions } from 'node:stream'

declare module './generated' {
  interface CommonRead {
    createReadStream(options?: ReadableOptions): Readable
  }

  interface CommonWrite {
    createWriteStream(options?: WritableOptions): Writable
  }

  interface CommonSave {
    save(path: string, options?: ReadableOptions): Promise<void>
  }

  interface Output extends CommonRead, CommonWrite {}

  interface ReadStream extends CommonSave {}

  interface LogsResponse extends CommonRead {}

  interface StatsStream extends CommonRead {}

  interface CreateImageOutput extends CommonRead {}

  interface PushImageInfoStream extends CommonRead {}

  interface ExportImageStream extends CommonSave {}

  interface Docker {
    version(): Promise<any>
  }
}

export * from './generated'
