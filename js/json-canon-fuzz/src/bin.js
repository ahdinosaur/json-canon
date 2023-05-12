#!/usr/bin/env node

const fuzz = require('./')

const args = process.argv.slice(2)

const type = args[0]
const numLines = parseInt(args[1])
const outputFilePath = args[2]

if (fuzz[type] == null || numLines === NaN) {
  usage()

  process.exit(1)
}

fuzz[type](numLines, outputFilePath)

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
