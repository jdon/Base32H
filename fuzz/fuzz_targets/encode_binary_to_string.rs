#![no_main]
use libfuzzer_sys::fuzz_target;

use base32h::encode_binary_to_string;

fuzz_target!(|data: &[u8]| {
    let _ = encode_binary_to_string(data);
});
