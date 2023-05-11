const test = require('ava')
const random = require('slump')
const { createWriteStr

const jsonCanon = require('../../')

const total = 1e4

for (let i = 0; i < total; i++) {
  // if (i % 1e3 === 0) console.log(i) 

  const json = random.json()

  const expected = canonicalize(json)

  let actual
  try {
    actual = jsonCanon(json)
  } catch (err) {
    if (err.message === 'Strings must be valid Unicode and not contain any surrogate pairs') {
      continue
    }
    throw err
  }

  await nextTick()
}

function nextTick() {
  return new Promise((resolve) => {
    setTimeout(resolve)
  })
}
