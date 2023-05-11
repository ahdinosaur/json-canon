# `json-canon`

[![Crates.io version](https://img.shields.io/crates/v/json-canon.svg?style=flat-square)](https://crates.io/crates/json-canon) [![Download](https://img.shields.io/crates/d/json-canon.svg?style=flat-square)](https://crates.io/crates/json-canon) [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/json-canon)

Serialize JSON into a canonical format.

## Install

```shell
cargo add json-canon
```

## Example

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

See [`./examples/basic.rs`](./examples/basic.rs)

## Usage

See [docs](https://docs.rs/json-canon/)

## Caveats

Different from [the JavaScript implementation](../../js/json-canon), `serde_json` deserializes `f64::NAN` and `f64::Infinite` as `None`, so if given a Rust struct with these values, the `json-canon` serializer will currently output `"null"`.

## Bench

```
from_elem/basic/[{"_id":"59ef4a83ee8364808d761beb","about":"Nisi reprehenderit nulla ad officia pari...
                        time:   [28.019 µs 28.032 µs 28.047 µs]
                        thrpt:  [35.654 Kelem/s 35.673 Kelem/s 35.690 Kelem/s]
```

## License

```txt
Copyright 2023 Michael Williams

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

## Related

- [`l1h3r/serde_jcs`](https://github.com/l1h3r/serde_jcs)
