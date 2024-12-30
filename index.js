const { Writable, Readable } = require('node:stream')

class ReadStream extends Readable {
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

class WriteStream extends Writable {
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

const { Docker, Output } = require('./generated.js')

Output.prototype.createReadStream = function (options) {
  return new ReadStream(this, options)
}

Output.prototype.createWriteStream = function (options) {
  return new WriteStream(this, options)
}

module.exports.Docker = Docker
