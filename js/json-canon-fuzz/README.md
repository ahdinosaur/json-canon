# `json-canon-fuzz`

[Fuzzer](https://en.wikipedia.org/wiki/Fuzzing) to test whether your JSON serialization is [canonical](https://datatracker.ietf.org/doc/rfc8785/).

## Install

```shell
npm install -g json-canon-fuzz
```

## Usage

```txt
Usage
  $ json-canon-fuzz {type} {count} {path}

Arguments
  - type: either `json` or `numbers`
  - count: how many lines to generate (default: Infinity)
  - path: where to output the generated lines (default: stdout)

Examples
  $ json-canon-fuzz json 100000
```

### Numbers

The test output consists of lines

```txt
hex-ieee,expected\n
```

where `hex-ieee` holds 1-16 ASCII hexadecimal characters representing an IEEE-754 double precision value while `expected` holds the expected serialized value.

Each line is terminated by a single new-line character.

Sample lines:

```txt
4340000000000001,9007199254740994
4340000000000002,9007199254740996
444b1ae4d6e2ef50,1e+21
3eb0c6f7a0b5ed8d,0.000001
3eb0c6f7a0b5ed8c,9.999999999999997e-7
8000000000000000,0
0,0
```

The generation is deterministic. After generating a file, the program will output the hash of the file.

The following table records the expected hashes:

| SHA-256 checksum                                                 | Number of lines | Size in bytes |
| ---------------------------------------------------------------- | --------------- | ------------- |
| be18b62b6f69cdab33a7e0dae0d9cfa869fda80ddc712221570f9f40a5878687 | 1000            | 37967         |
| b9f7a8e75ef22a835685a52ccba7f7d6bdc99e34b010992cbc5864cd12be6892 | 10000           | 399022        |
| 22776e6d4b49fa294a0d0f349268e5c28808fe7e0cb2bcbe28f63894e494d4c7 | 100000          | 4031728       |
| 49415fee2c56c77864931bd3624faad425c3c577d6d74e89a83bc725506dad16 | 1000000         | 40357417      |
| b9f8a44a91d46813b21b9602e72f112613c91408db0b8341fb94603d9db135e0 | 10000000        | 403630048     |
| 0f7dda6b0837dde083c5d6b896f7d62340c8a2415b0c7121d83145e08a755272 | 100000000       | 4036326174    |

### JSON

The test output consists of lines

```txt
json\n
```

where `json` is a valid JSON object serialized according to [RFC8785: JSON Canonicalization Scheme (JCS)](https://tools.ietf.org/html/rfc8785).

Each line is terminated by a single new-line character.

Sample lines:

```txt
false
{"J���\toR�B\u001d":false,"]E�\u001e\u001d\u0019":4206124647,"�":"�Ĉ/�\u000e\b������5ɵ3\u0014","�\u0001Ƨ�\u0002ŵ":4.633184220509346,"�h\u0002\u000b�u��\\�":"�7� ���xB�K���!\u0019�ujҡ\u0019�Nx�D<��m\u0010*�\u0016p�\u001b�^ aLNt����P�","��X|Av[^E":-1306679260}
0.7290520820698927
null
"����Z\u0000��/�Bc_+�m�\u0002�\u0006�u�������RṚ�H\u000f�K\u0019��8\"�^ռI�\u0011ו\u001c�:D��n\u0005�s�m\u0013H��\t\b� H̑�\u001a}����\u0005 y&BO"
{}
```

The generation is not deterministic.

## References

- [`json-canonicalization/test-data`](https://github.com/cyberphone/json-canonicalization/tree/master/testdata)
