mod common;
use common::*;
use rask_x86_64::operand::Operand::*;
use rask_x86_64::registers::Reg64::*;

#[test]
fn test_mov_rax_and_r10_imm64() {
    let bytes = encode(|e| {
        e.mov(Reg(RAX), Imm(1337));
        e.mov(Reg(R10), Imm(42));
    });

    // 48 b8 39 05 00 00 00 00 00 00    mov rax, 1337
    // 49 ba 2a 00 00 00 00 00 00 00    mov r10, 42
    let expected = [
        0x48, 0xB8, 0x39, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0xBA, 0x2A, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    assert_bytes(&bytes, &expected);
}

#[test]
fn test_rex_prefix_changes_with_high_registers() {
    let bytes_low = encode(|e| e.mov(Reg(RAX), Imm(0)));
    let bytes_high = encode(|e| e.mov(Reg(R8), Imm(0)));

    assert_eq!(bytes_low[0], 0x48); // REX.W only
    assert_eq!(bytes_high[0], 0x49); // REX.W + REX.B
}
