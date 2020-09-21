#![no_main]
use libfuzzer_sys::fuzz_target;

use base32h::encode_to_string;

fuzz_target!(|data: u128| {
    let _ = encode_to_string(data);
});
