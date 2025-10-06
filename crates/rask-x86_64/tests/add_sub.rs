use rask_x86_64::encoder::Encoder;
use rask_x86_64::registers::Reg64::*;

#[test]
fn test_add_and_sub_basic() {
    let mut e = Encoder::new();
    e.add(RAX, RBX);
    e.sub(R8, R9);

    let expected = [
        0x48, 0x01, 0xD8, // add rax, rbx
        0x4D, 0x29, 0xC8, // sub r8, r9
    ];

    assert_eq!(e.bytes(), &expected);
}

#[test]
fn test_ret() {
    let mut e = Encoder::new();
    e.ret();
    assert_eq!(e.bytes(), &[0xC3]);
}
