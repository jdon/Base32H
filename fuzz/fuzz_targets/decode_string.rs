#![no_main]
use libfuzzer_sys::fuzz_target;

use base32h::decode_string;

fuzz_target!(|data: String| {
    let _ = decode_string(&data);
});
