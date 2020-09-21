# Base32H

An implementation of base32. Built following the spec and test cases from https://base32h.github.io/

## Installation

```
cargo add base32h
```

## Usage
```
use base32h::{encode_binary_to_string, encode_to_string, decode_string_to_binary, decode_string};

assert_eq!(encode_to_string(1099511627775).unwrap(), "ZZZZZZZZ".to_owned());
assert_eq!(decode_string("ZZZZZZZZ"), Some(1099511627775));

assert_eq!(encode_binary_to_string(&[255, 255, 255, 255, 255, 255]), "0000007ZZZZZZZZZ".to_owned());
assert_eq!(decode_string_to_binary("zZzZzZzZ"), Vec::from([255, 255, 255, 255, 255]));
```

### Test

Run:
`cargo test`

### Benchmarks

Run:
`cargo bench`

### Fuzzing

Install cargo fuzz:

`cargo install cargo-fuzz`

Run one of the fuzzing targets (Must be on nightly):
* `cargo fuzz run encode_binary_to_string -j 8` // 8 is the number of threads to use
* `cargo fuzz run decode_string_to_binary -j 8`
* `cargo fuzz run encode_to_string -j 8`
* `cargo fuzz run decode_string -j 8`
