const { Writable, Readable } = require('node:stream')

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

const { Docker, Output, ReadStream } = require('./generated.js')
const { createWriteStream } = require('node:fs')

Output.prototype.createReadStream = function (options) {
  return new ReadableStream(this, options)
}

Output.prototype.createWriteStream = function (options) {
  return new WritableStream(this, options)
}

ReadStream.prototype.save = function (path, options) {
  const writable = createWriteStream(path)
  const readable = new ReadStream(this, options)
  const fd = readable.pipe(writable)
  return new Promise((resolve, reject) => {
    fd.on('error', reject)
    fd.on('close', resolve)
  })
}

module.exports.Docker = Docker
