#![no_main]

use libfuzzer_sys::fuzz_target;
use fibonacci_stable::parse_input;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // parse_input не должна паниковать ни на каком вводе
        let _ = parse_input(s);
    }
});
