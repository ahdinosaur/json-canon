const { createWriteStream } = require('fs')
const random = require('slump')

const outputFilePath = join(__dirname, "../../../test-data/test-json.txt")
const numLines = 1e6

const serialize = require('json-canon')

const total = 1e4

const outputFile = createWriteStream(outputFilePath)

for (let i = 0; i < total; i++) {
  // if (i % 1e3 === 0) console.log(i) 

  const obj = random.json()

  let json
  try {
    json = jsonCanon(obj)
  } catch (err) {
    if (err.message === 'Strings must be valid Unicode and not contain any surrogate pairs') {
      continue
    }
    throw err
  }

  outputFile.write(json);
}

outputFile.close()
