# hashy

Hashy is a CLI application made entirely with Rust with a library of hashing algorithms like MD5, SHA-2 and SHAKE.
It might serve useful for checksumming files or just comparing how different hashing algorithms behave.

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

## Disclaimer

These algorithm implementations only serve as an exercise for me and are _not recommended for production-critical
use_ (like hashing user passwords)!

## Usage

`hashy [FLAGS] [OPTIONS] <algorithm> <input>`

### Args
- `algorithm`: Algorithm name in kebab case (example: sha-512-224).
  Certain algorithms require extra parameters. See algorithm list for more info.
- `input`: Filepath, treated as input text if `-t` flag is passed.
  Only defaults to `stdin` if omitted and command is piped into.

### Flags
- `-t (--text)`: Input is treated as text if specified.
- `-l (--list)`: Lists all supported algorithms.
- `-v (--verbose)`: Show verbose output, like time taken to digest.

### Options
- `-e (--encoding)`: Encoding type for output hash.
  - `hex` (default)
  - `hex_upper`: Uppercase hexadecimal.
  - `base64`
  - `bin`: Literal binary representation (0s and 1s).

### Examples

Getting the MD5 checksum of a file `~/test.txt`:

```console
$ hashy md5 ~/test.txt
```

Getting the MD5 checksum of a message:

```console
$ hashy md5 -t "The quick brown fox jumps over the lazy dog"
```

Getting the SHAKE128 checksum of a message (without output length of 72 bits):

```console
$ hashy shake128-72 "The quick brown fox jumps over the lazy dog"
```

## Binary

You need to install rust and cargo to compile this project. [See here](https://www.rust-lang.org/tools/install).

After running `cargo build` from the root project folder, the binary can be found under `target/debug` as `hashy`. Alternatively, you can use `cargo run -- [args]`. 

For the performance-optimized version, use `cargo build --release`, this would speed up the processing time by a lot, but would still be slower than existing solutions like `sha256sum` on Linux.

## Algorithms

- `md` variants
  - `md2`
  - `md4`
  - `md5`
  - `md6-n`
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
  - `shake128-n`
  - `shake256-n`

`n` denotes arbitrary output length (in bits, must be multiple of 8).

## Planned algorithms

- BLAKE variants
- RIPE variants
- TIGER
- ...
