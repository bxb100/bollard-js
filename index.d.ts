import { Readable, ReadableOptions, Writable, WritableOptions } from "node:stream";

declare module "./generated" {

  interface AttachOutput {

    createReadStream(options?: ReadableOptions): Readable;

    createWriteStream(options?: WritableOptions): Writable;
  }
}

export * from "./generated";