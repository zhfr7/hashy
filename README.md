# hashy

Hashy is a CLI application made entirely with Rust with a library of hashing algorithms like MD5, SHA-2 and SHAKE.\
This is just a small project for me so don't expect all of it to be perfectly memory safe or performant.

- [hashy](#hashy)
  - [Usage](#usage)
    - [Args](#args)
    - [Flags](#flags)
    - [Options](#options)
    - [Examples](#examples)
  - [Binary](#binary)
  - [Algorithms](#algorithms)
  - [Planned algorithms](#planned-algorithms)

* * *

## Usage

`hashy [FLAGS] [OPTIONS] <algorithm> <input>`

### Args
- `algorithm` - algorithm name in kebab case (example: sha-512-224).
- `input` - input message/filepath, for empty use `""`.

### Flags
- `-f (--file)` - input is treated as a filepath if specified.
- `-l (--list)` - lists all the supported algorithms.

### Options
- `-e (--encoding)` - encoding type for output hash.
  - `hex` (default)
  - `hex_upper` - uppercase hexadecimal
  - `base64`
  - `bin` - binary
- `-o (--output)` - output file to write program output to (default: stdout)

### Examples

`hashy md5 "The quick brown fox jumps over the lazy dog"`

Certain algorithms like SHAKE require a length as a parameter.\
This is done by appending the output length in bits to the end of the algorithm name separated by
a dash. e.g:

`hashy shake128-72 "The quick brown fox jumps over the lazy dog"`

would produce a SHAKE128 hash with length 72/8 = 9 bytes.\
However, if a number not divisible by 8 is given, it would result in an error.

## Binary

You need to install rust and cargo to compile this project. [See here](https://www.rust-lang.org/tools/install).

After running `cargo build` from the root project folder, the binary can be found under `target/debug` as `hashy`. Alternatively, you can use `cargo run -- [args]`. 

For the performance-optimized version, use `cargo build --release`, this would speed up the processing time by a lot, but would still be slower than existing solutions like `sha256sum` on Linux.

## Algorithms

- `md` variants
  - `md2`
  - `md4`
  - `md5`
- `sha1`
- `sha2` variants
  - `sha-224`
  - `sha-256`
  - `sha-384`
  - `sha-512`
  - `sha-512-224`, `-512-256`
- `sha3` variants
  - `sha3-224`
  - `sha3-256`
  - `sha3-384`
  - `sha3-512`
  - `shake128-n` (arbitrary output length)
  - `shake256-n`

## Planned algorithms

- MD6
- BLAKE variants
- RIPE variants
- TIGER
- ...