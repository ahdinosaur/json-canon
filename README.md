# `json-canon`

Serialize JSON into a canonical format.

Safe for generating a consistent cryptographic hash or signature across platforms.

Follows [RFC8785: JSON Canonicalization Scheme (JCS)](https://tools.ietf.org/html/rfc8785)

> JCS [builds] on the serialization formats for JSON
> primitives as defined by ECMAScript [[ES](https://ecma-international.org/ecma-262/)],
> constraining JSON data to the I-JSON [[RFC7493](https://tools.ietf.org/html//rfc7493)] subset,
> and through a platform independent property sorting scheme.
>
> Public RFC: https://tools.ietf.org/html/rfc8785
>
> The JSON Canonicalization Scheme concept in a nutshell:
>
> - Serialization of primitive JSON data types using methods compatible with ECMAScript's `JSON.stringify()`
> - Lexicographic sorting of JSON `Object` properties in a *recursive* process
> - JSON `Array` data is also subject to canonicalization, *but element order remains untouched*

## Usage

### JavaScript

[`json-canon`](./js/json-canon)

### Rust

TODO

## References

- [`cyberphone/ietf-json-canon`](https://github.com/cyberphone/ietf-json-canon)
- [`cyberphone/json-canonicalization`](https://github.com/cyberphone/json-canonicalization)
- [`erdtman/canonicalize`](https://github.com/erdtman/canonicalize)
- [`l1h3r/serde_jcs`](https://github.com/l1h3r/serde_jcs)
