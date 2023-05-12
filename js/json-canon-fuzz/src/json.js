const random = require('slump')
const serialize = require('json-canon')
const { Readable } = require('readable-stream')

module.exports = generateJson

/**
 * @param {object} options
 * @param {number} options.numLines
 * @param {string | undefined} options.outputFilePath
 * @returns {Readable}
 */
function generateJson(options) {
  const { numLines } = options

  let i = 0

  return new Readable({
    read() {
      if (i >= numLines) {
        this.push(null)
        return
      }
      this.push(nextLine(i++))
    },
  })

  function nextLine() {
    const obj = random.json()

    let json
    try {
      json = serialize(obj)
    } catch (err) {
      if (
        err.message ===
        'Strings must be valid Unicode and not contain any surrogate pairs'
      ) {
        return nextLine()
      }
      throw err
    }

    return json + '\n'
  }
}
