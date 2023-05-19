# `json-canon`

Serialize JSON into a canonical format.

Safe for generating a consistent cryptographic hash or signature across platforms.

Follows [RFC8785: JSON Canonicalization Scheme (JCS)](https://tools.ietf.org/html/rfc8785)

![JSON cannon](https://i.imgur.com/OdH7hw1.png)

## Features

The JSON Canonicalization Scheme concept in a nutshell:

- Serialization of primitive JSON data types using methods compatible with ECMAScript's `JSON.stringify()`
- Lexicographic sorting of JSON `Object` properties in a *recursive* process
- JSON `Array` data is also subject to canonicalization, *but element order remains untouched*

## Serializers

### JavaScript: [`json-canon`](./js/json-canon)

[![npm version](https://img.shields.io/npm/v/json-canon.svg?style=flat-square)](https://www.npmjs.com/package/json-canon) [![download](https://img.shields.io/npm/dt/json-canon?style=flat-square)](https://www.npmjs.com/package/json-canon) [![ci status](https://img.shields.io/github/actions/workflow/status/ahdinosaur/json-canon/js.yml?branch=main&style=flat-square)](https://github.com/ahdinosaur/json-canon/actions/workflows/js.yml)

```js
const serialize = require('json-canon')

const json = {
  from_account: "543 232 625-3",
  to_account: "321 567 636-4",
  amount: 500,
  currency: "USD"
}

console.log(serialize(json))
// {"amount":500,"currency":"USD","from_account":"543 232 625-3","to_account":"321 567 636-4"}
```

### Rust: [`json-canon`](./rust/json-canon)

[![crates.io version](https://img.shields.io/crates/v/json-canon.svg?style=flat-square)](https://crates.io/crates/json-canon) [![download](https://img.shields.io/crates/d/json-canon.svg?style=flat-square)](https://crates.io/crates/json-canon) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/json-canon) [![ci status](https://img.shields.io/github/actions/workflow/status/ahdinosaur/json-canon/rust.yml?branch=main&style=flat-square)](https://github.com/ahdinosaur/json-canon/actions/workflows/rust.yml)

```rust
use json_canon::to_string;
use serde_json::json;

let data = json!({
    "from_account": "543 232 625-3",
    "to_account": "321 567 636-4",
    "amount": 500,
    "currency": "USD"
});

println!("{}", to_string(&data)?);
// {"amount":500,"currency":"USD","from_account":"543 232 625-3","to_account":"321 567 636-4"}
```

## Fuzzers

- JavaScript: [`json-canon-fuzz`](./js/json-canon-fuzz)

## References

- [`cyberphone/ietf-json-canon`](https://github.com/cyberphone/ietf-json-canon)
- [`cyberphone/json-canonicalization`](https://github.com/cyberphone/json-canonicalization)
