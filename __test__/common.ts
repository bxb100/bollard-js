//! copy from https://github.com/openai/openai-node/blob/master/src/internal/decoders/line.ts

type Bytes = string | ArrayBuffer | Uint8Array | Buffer | null | undefined

/**
 * A re-implementation of httpx's `LineDecoder` in Python that handles incrementally
 * reading lines from text.
 *
 * https://github.com/encode/httpx/blob/920333ea98118e9cf617f246905d7b202510941c/httpx/_decoders.py#L258
 */
export class LineDecoder {
  // prettier-ignore
  static NEWLINE_CHARS = new Set(['\n', '\r']);
  static NEWLINE_REGEXP = /\r\n|[\n\r]/g

  buffer: string[]
  trailingCR: boolean
  textDecoder: any // TextDecoder found in browsers; not typed to avoid pulling in either "dom" or "node" types.

  constructor() {
    this.buffer = []
    this.trailingCR = false
  }

  decode(chunk: Bytes): string[] {
    let text = this.decodeText(chunk)

    if (this.trailingCR) {
      text = '\r' + text
      this.trailingCR = false
    }
    if (text.endsWith('\r')) {
      this.trailingCR = true
      text = text.slice(0, -1)
    }

    if (!text) {
      return []
    }

    const trailingNewline = LineDecoder.NEWLINE_CHARS.has(text[text.length - 1] || '')
    let lines = text.split(LineDecoder.NEWLINE_REGEXP)

    // if there is a trailing new line then the last entry will be an empty
    // string which we don't care about
    if (trailingNewline) {
      lines.pop()
    }

    if (lines.length === 1 && !trailingNewline) {
      this.buffer.push(lines[0]!)
      return []
    }

    if (this.buffer.length > 0) {
      lines = [this.buffer.join('') + lines[0], ...lines.slice(1)]
      this.buffer = []
    }

    if (!trailingNewline) {
      this.buffer = [lines.pop() || '']
    }

    return lines
  }

  decodeText(bytes: Bytes): string {
    if (bytes == null) return ''
    if (typeof bytes === 'string') return bytes

    // Node:
    if (typeof Buffer !== 'undefined') {
      if (bytes instanceof Buffer) {
        return bytes.toString()
      }
      if (bytes instanceof Uint8Array) {
        return Buffer.from(bytes).toString()
      }

      throw new Error(
        `Unexpected: received non-Uint8Array (${bytes.constructor.name}) stream chunk in an environment with a global "Buffer" defined, which this library assumes to be Node. Please report this error.`,
      )
    }

    // Browser
    if (typeof TextDecoder !== 'undefined') {
      if (bytes instanceof Uint8Array || bytes instanceof ArrayBuffer) {
        this.textDecoder ??= new TextDecoder('utf8')
        return this.textDecoder.decode(bytes)
      }

      throw new Error(
        `Unexpected: received non-Uint8Array/ArrayBuffer (${(bytes as any).constructor.name}) in a web platform. Please report this error.`,
      )
    }

    throw new Error(`Unexpected: neither Buffer nor TextDecoder are available as globals. Please report this error.`)
  }

  flush(): string[] {
    if (!this.buffer.length && !this.trailingCR) {
      return []
    }

    const lines = [this.buffer.join('')]
    this.buffer = []
    this.trailingCR = false
    return lines
  }
}

//! copy from https://github.com/openai/openai-node/blob/master/src/streaming.ts

/**
 * Most browsers don't yet have async iterable support for ReadableStream,
 * and Node has a very different way of reading bytes from its "ReadableStream".
 *
 * This polyfill was pulled from https://github.com/MattiasBuelens/web-streams-polyfill/pull/122#issuecomment-1627354490
 */
export function readableStreamAsyncIterable(stream: any): AsyncIterableIterator<Bytes> {
  if (stream[Symbol.asyncIterator]) return stream;

  return {
    async next() {
      try {
        const buf = Buffer.alloc(65536)
        let s: bigint = await stream.read(buf)
        if (s === 0n) {
          return { done: true, value: undefined }
        }
        return { done: false, value: buf.subarray(0, Number(s)) }
      } catch (e) {
        throw e
      }
    },
    [Symbol.asyncIterator]() {
      return this
    },
  }
}

export async function* iterLine(read: any): AsyncGenerator<string> {
  const lineDecoder = new LineDecoder()
  const iter = readableStreamAsyncIterable(read)
  for await (const chunk of iter) {
    for (const line of lineDecoder.decode(chunk)) {
      if (line) {
        yield line
      }
    }
  }

  for (const line of lineDecoder.flush()) {
    yield line
  }
}
