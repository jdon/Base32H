#![no_main]
use libfuzzer_sys::fuzz_target;

use base32h::decode_string_to_binary;

fuzz_target!(|data: String| {
    let _ = decode_string_to_binary(&data);
});
