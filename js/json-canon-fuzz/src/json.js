const { createWriteStream } = require('fs')
const { join } = require('path')
const random = require('slump')
const serialize = require('json-canon')

const args = process.argv.slice(2)

const numLines = parseInt(args[0])
const outputFilePath = args[1]

const outputStream = getOutputStream(outputFilePath)

function next(i = 0) {
  if (i >= numLines) {
    if (outputStream.close != null) {
      outputStream.close()
    }
    return
  }

  // if (i % 1e3 === 0) console.log(i)

  const obj = random.json()

  let json
  try {
    json = serialize(obj)
  } catch (err) {
    if (
      err.message ===
      'Strings must be valid Unicode and not contain any surrogate pairs'
    ) {
      return next(i)
    }
    throw err
  }

  write(outputStream, json + '\n', function (err) {
    if (err) throw err
    next(i + 1)
  })
}

next()

function write(stream, data, cb) {
  if (!stream.write(data, 'utf8')) {
    stream.once('drain', cb)
  } else {
    process.nextTick(cb)
  }
}

function getOutputStream(outputFilePath) {
  if (outputFilePath) {
    const outputFileFullPath = join(process.cwd(), outputFilePath)
    return createWriteStream(outputFileFullPath, { encoding: 'utf8' })
  }
  return process.stdout
}
