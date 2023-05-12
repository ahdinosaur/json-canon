#!/usr/bin/env node

const { createWriteStream } = require('fs')
const { join } = require('path')
const { pipeline } = require('readable-stream')

const fuzz = require('./')

const args = process.argv.slice(2)

const type = args[0]
const numLines = args[1] ? parseInt(args[1]) : Infinity
const outputFilePath = args[2]

if (fuzz[type] === undefined || Number.isNaN(numLines)) {
  usage()

  process.exit(1)
}

const getFuzzStream = fuzz[type]

const fuzzStream = getFuzzStream({ numLines, outputFilePath })
const outputStream = getOutputStream(outputFilePath)

pipeline(fuzzStream, outputStream, function onDone(err) {
  if (err) throw err
})

function usage() {
  console.log('Usage')
  console.log('  $ json-canon-fuzz {type} {count} {path}')
  console.log('')
  console.log('Arguments')
  console.log('  - type: either `json` or `numbers`')
  console.log('  - count: how many lines to generate (default: Infinity)')
  console.log('  - path: where to output the generated lines (default: stdout)')
  console.log('')
  console.log('Examples')
  console.log('  $ json-canon-fuzz json 100000')
}

function getOutputStream(outputFilePath) {
  if (outputFilePath) {
    const outputFileFullPath = join(process.cwd(), outputFilePath)
    return createWriteStream(outputFileFullPath, { encoding: 'utf8' })
  }
  return process.stdout
}

