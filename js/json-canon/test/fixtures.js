const test = require('ava')
const { join } = require('path')
const { readFileSync, readdirSync } = require('fs')

const jsonCanon = require('../')

const testDataBaseDir = join(__dirname, '../../../test-data')
const testDataInputDir = join(testDataBaseDir, 'input')
const testDataOutputDir = join(testDataBaseDir, 'output')

readdirSync(testDataInputDir).forEach((name) => {
  test(name, (t) => {
    const input = readJsonSync(join(testDataInputDir, name))
    const expected = readFileSync(join(testDataOutputDir, name), 'utf8').trim()
    const actual = jsonCanon(input)
    t.is(actual, expected)
  })
})

function readJsonSync(path) {
  return JSON.parse(readFileSync(path, 'utf8'))
}
