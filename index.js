const { Writable, Readable } = require('node:stream')
const { createWriteStream } = require('node:fs')
const {
  Docker,
  Output,
  ReadStream,
  LogsResponse,
  StatsStream,
  CreateImageOutput,
  PushImageInfoStream,
  ExportImageStream,
} = require('./generated')

class ReadableStream extends Readable {
  constructor(reader, options) {
    super(options)
    this.reader = reader
  }

  _read(size) {
    const buf = Buffer.alloc(size)
    this.reader
      .read(buf)
      .then((s) => {
        if (s === 0n) {
          this.push(null)
        } else {
          this.push(buf.subarray(0, Number(s)))
        }
      })
      .catch((e) => {
        this.emit('error', e)
      })
  }
}

class WritableStream extends Writable {
  constructor(writer, options) {
    super(options)
    this.writer = writer
  }

  _write(chunk, encoding, callback) {
    this.writer
      .write(chunk)
      .then(() => {
        callback()
      })
      .catch((e) => {
        callback(e)
      })
  }

  _final(callback) {
    this.writer
      .close()
      .then(() => {
        callback()
      })
      .catch((e) => {
        callback(e)
      })
  }
}

Output.prototype.createReadStream = function (options) {
  return new ReadableStream(this, options)
}

Output.prototype.createWriteStream = function (options) {
  return new WritableStream(this, options)
}

LogsResponse.prototype.createReadStream = function (options) {
  return new ReadableStream(this, options)
}

StatsStream.prototype.createReadStream = function (options) {
  return new ReadableStream(this, options)
}

CreateImageOutput.prototype.createReadStream = function (options) {
  return new ReadableStream(this, options)
}

PushImageInfoStream.prototype.createReadStream = function (options) {
  return new ReadableStream(this, options)
}

ReadStream.prototype.save = function (path, options) {
  const readable = new ReadableStream(this, options)
  const writable = createWriteStream(path)
  const fd = readable.pipe(writable)
  return new Promise((resolve, reject) => {
    fd.on('error', reject)
    fd.on('close', resolve)
  })
}

ExportImageStream.prototype.save = function (path, options) {
  const readable = new ReadableStream(this, options)
  const writable = createWriteStream(path)
  const fd = readable.pipe(writable)
  return new Promise((resolve, reject) => {
    fd.on('error', reject)
    fd.on('close', resolve)
  })
}

const json = (buffer) => JSON.parse(buffer.toString())

Docker.prototype.version = function () {
  return this._version().then(json)
}

module.exports.Docker = Docker
