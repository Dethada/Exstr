#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

pub fn get_strings(buf: Vec<u8>, min_count: u64) -> String {
    let mut count: u64 = 0;
    let mut last_index = 0;
    let mut result = String::new();
    for (i, byte) in buf.iter().enumerate() {
        let c = *byte;
        if (c > 31 && c < 127) || (c == 9 || c == 13) {
            count += 1;
        } else {
            if count >= min_count {
                result.push_str(&String::from_utf8_lossy(&buf[last_index..i-1]));
                result.push('\n');
            }
            count = 0;
            last_index = i + 1;
        }
    }
    if count >= min_count {
        result.push_str(&String::from_utf8_lossy(&buf[last_index..]));
    }
    result
}