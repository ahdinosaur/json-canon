'use strict'

const Benchmark = require('benchmark')
const suite = new Benchmark.Suite()

const testData = require('./test.json')

const stringifyPackages = {
  'json-canon': require('../src/index'),
  'JSON.stringify': JSON.stringify,
  'fast-json-stable-stringify': true,
  'json-stable-stringify': true,
  'fast-stable-stringify': true,
  'json-stringify-deterministic': true,
  'faster-stable-stringify': true,
  'fast-safe-stringify': true,
  'safe-stable-stringify': true,
  canonicalize: true,
  'json-canonicalize': require('json-canonicalize').canonicalize,
}

for (const name in stringifyPackages) {
  let func = stringifyPackages[name]
  if (func === true) func = require(name)

  suite.add(name, function () {
    func(testData)
  })
}

suite
  .on('cycle', (event) => console.log(String(event.target)))
  .on('complete', function () {
    console.log('The fastest is ' + this.filter('fastest').map('name'))
  })
  .run({ async: true })
