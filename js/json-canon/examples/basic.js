const serialize = require('../src/index')

const json = {
  from_account: "543 232 625-3",
  to_account: "321 567 636-4",
  amount: 500,
  currency: "USD"
}

console.log(serialize(json))
// {"amount":500,"currency":"USD","from_account":"543 232 625-3","to_account":"321 567 636-4"}
