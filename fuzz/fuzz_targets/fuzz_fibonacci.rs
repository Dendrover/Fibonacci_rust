#![no_main]

use libfuzzer_sys::fuzz_target;
use fibonacci_stable::fibonacci;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 16 {
        let n = u128::from_le_bytes(data[..16].try_into().unwrap());
        // fibonacci возвращает Result — не должна паниковать ни при каком вводе
        let _ = fibonacci(n);
    }
});
