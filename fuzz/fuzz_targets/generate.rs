#![no_main]
use libfuzzer_sys::fuzz_target;
use barcoders::generators::ascii::ASCII;
use barcoders::sym::ean13::EAN13;

fuzz_target!(|input: ([u8; 13], bool)| {
    let mut data: Vec<_> = input.0.iter().map(|x| (x % 10).to_string()).collect();
    if input.1 {
        data = data[..12].to_vec();
    }
    let data = data.join("");
    let barcode = EAN13::new(data).unwrap();
    let ascii = ASCII::new();
    let _ = ascii.generate(barcode.encode());
});
