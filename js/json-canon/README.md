# `json-canon`

## Install

```shell
npm install json-canon
```

## Example

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

See [`./examples/basic.js`](./examples/basic.js)

## Usage

### `serialize = require('json-canon')`

```ts
function serialize(input: unknown): string;
```

## Bench

```txt
json-canon x 44,441 ops/sec ±0.42% (91 runs sampled)
JSON.stringify x 101,970 ops/sec ±0.37% (90 runs sampled)
fast-json-stable-stringify x 30,066 ops/sec ±0.35% (95 runs sampled)
json-stable-stringify x 23,099 ops/sec ±0.50% (93 runs sampled)
fast-stable-stringify x 35,560 ops/sec ±0.23% (91 runs sampled)
json-stringify-deterministic x 18,521 ops/sec ±0.37% (95 runs sampled)
faster-stable-stringify x 28,588 ops/sec ±0.20% (96 runs sampled)
fast-safe-stringify x 83,523 ops/sec ±0.23% (98 runs sampled)
safe-stable-stringify x 50,807 ops/sec ±0.26% (94 runs sampled)
canonicalize x 28,365 ops/sec ±0.79% (93 runs sampled)
json-canonicalize x 30,372 ops/sec ±0.56% (89 runs sampled)
The fastest is JSON.stringify
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

- [`canonicalize`](https://github.com/erdtman/canonicalize)
- [`json-canonicalize`](https://github.com/snowyu/json-canonicalize.ts)
