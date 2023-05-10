# `json-canon`

## Install

```shell
npm install json-canon
```

## Example

```js
const canon = require('json-canon')

const json = {
  from_account: "543 232 625-3",
  to_account: "321 567 636-4",
  amount: 500,
  currency: "USD"
}
console.log(canon(json))
// {"amount":500,"currency":"USD","from_account":"543 232 625-3","to_account":"321 567 636-4"}
```

## Usage

```ts
type 

```
### `canon = require('json-canon')`

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
