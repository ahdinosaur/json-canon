const { createWriteStream } = require('fs')
const { join } = require('path')
const random = require('slump')
const serialize = require('json-canon')

const args = process.argv.slice(2)

const outputFilePath = join(process.cwd(), args[0])
const total = 1e4

const outputFile = createWriteStream(outputFilePath)

function next(i = 0) {
  if (i === total) {
    outputFile.close()
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
      return next(i + 1)
    }
    throw err
  }

  write(outputFile, json + '\n', function (err) {
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
