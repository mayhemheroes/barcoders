#![no_main]
use std::cmp::min;
use libfuzzer_sys::fuzz_target;
use barcoders::sym::code93::Code93;

const CHARS: [&str; 46] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
    "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    "-", ".", " ", "$", "/", "+", "%", "(", ")", "[",
];

fuzz_target!(|input: Vec<usize>| {
    if input.len() > 0 {
        let mut data: Vec<_> = input.iter().map(|x| CHARS[x % CHARS.len()]).collect();
        data = data[..min(256, data.len())].to_vec();
        let data = data.join("");
        let _ = Code93::new(data).unwrap();
    }
});
