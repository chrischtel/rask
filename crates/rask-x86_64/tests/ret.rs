mod common;
use common::*;

#[test]
fn test_ret_instruction() {
    let bytes = encode(|e| e.ret());
    assert_bytes(&bytes, &[0xC3]);
}