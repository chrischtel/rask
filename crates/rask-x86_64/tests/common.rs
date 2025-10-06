use rask_x86_64::encoder::{self, Encoder};

/// Helper to format mismatches clearly when comparing byte sequences.
pub fn assert_bytes(actual: &[u8], expected: &[u8]) {
    if actual != expected {
        println!("Expected: {:02x?}", expected);
        println!("Actual:   {:02x?}", actual);
        panic!("Byte sequence mismatch");
    }
}

/// Helper to create an encoder and return its final bytes.
pub fn encode<F: FnOnce(&mut Encoder)>(f: F) -> Vec<u8> {
    let mut enc = Encoder::new();
    f(&mut enc);
    enc.bytes().to_vec()
}
