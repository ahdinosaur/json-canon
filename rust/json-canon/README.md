# `json-canon`

## Install

```shell
cargo add json-canon
```

## Example

```rust
use json_canon::to_string;
use serde_json::{json, Error};

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

TODO

## Caveats

Different from [the JavaScript implementation](../../js/json-canon), `serde_json` deserializes `f64::NAN` and `f64::Infinite` as `None`, so if given a Rust struct with these values, the `json-canon` serializer will currently output `"null"`.

## Bench

TODO

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

- [`serde_jcs`](https://github.com/l1h3r/serde_jcs)
