# hashy

Hashy is a CLI application made entirely with Rust with a library of hashing algorithms like MD2, MD4 and MD5. This is just a small project for me so don't expect all of it to be perfectly memory safe or performant.

- [hashy](#hashy)
  - [Usage](#usage)
  - [Algorithms](#algorithms)
  - [Planned algorithms](#planned-algorithms)

* * *

## Usage

`hashy -a <algorithm> [-f] [-e <encoding>] <input>`

- `algorithm` - algorithm name in kebab case (example: sha-512-224).
- `-f` flag - input is treated as a filepath if specified.
- `-e` option - encoding type for output hash. Can be one of the following:
  - `hex` (default)
  - `hex_upper` - uppercase hexadecimal
  - `base64`
  - `bin` - binary
- `input` - input message/filepath, for empty use `""`.

After running `cargo build` from the root project folder, the binary can be found under `target/debug` as `hashy`. 

Alternatively, you can use `cargo run -- [args]`.

## Algorithms

- MD2
- MD4
- MD5
- SHA1
- SHA2 variants
  - SHA-224, -256, -384, -512, -512/224, -512/256

## Planned algorithms

- MD6
- SHA variants
- BLAKE variants
- RIPE variants
- TIGER
- ...