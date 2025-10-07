mod common;
use common::*;
use rask_x86_64::operand::{MemOperand, Operand};
use rask_x86_64::registers::Reg64::*;

#[test]
fn test_mov_reg_mem() {
    let bytes = encode(|e| {
        // mov rax, [rbx]
        let mem = MemOperand { base: RBX, disp: 0 };
        e.mov(Operand::Reg(RAX), Operand::Mem(mem));
    });

    // REX.W + 8B /r: 48 8b 03
    let expected = [0x48, 0x8B, 0x03];
    assert_bytes(&bytes, &expected);
}

#[test]
fn test_mov_mem_reg() {
    let bytes = encode(|e| {
        // mov [rbx], rax
        let mem = MemOperand { base: RBX, disp: 0 };
        e.mov(Operand::Mem(mem), Operand::Reg(RAX));
    });

    // REX.W + 89 /r: 48 89 03
    let expected = [0x48, 0x89, 0x03];
    assert_bytes(&bytes, &expected);
}

#[test]
fn test_mov_with_displacement() {
    let bytes = encode(|e| {
        // mov rax, [rbx + 8]
        let mem = MemOperand { base: RBX, disp: 8 };
        e.mov(Operand::Reg(RAX), Operand::Mem(mem));
    });

    // REX.W + 8B /r + disp8: 48 8b 43 08
    let expected = [0x48, 0x8B, 0x43, 0x08];
    assert_bytes(&bytes, &expected);
}

#[test]
fn test_mov_with_large_displacement() {
    let bytes = encode(|e| {
        // mov rax, [rbx + 1000]
        let mem = MemOperand {
            base: RBX,
            disp: 1000,
        };
        e.mov(Operand::Reg(RAX), Operand::Mem(mem));
    });

    // REX.W + 8B /r + disp32: 48 8b 83 e8 03 00 00
    let expected = [0x48, 0x8B, 0x83, 0xE8, 0x03, 0x00, 0x00];
    assert_bytes(&bytes, &expected);
}

#[test]
fn test_mov_with_extended_registers() {
    let bytes = encode(|e| {
        // mov r10, [r11]
        let mem = MemOperand { base: R11, disp: 0 };
        e.mov(Operand::Reg(R10), Operand::Mem(mem));
    });

    // REX.W+R+B + 8B /r: 4d 8b 13
    let expected = [0x4D, 0x8B, 0x13];
    assert_bytes(&bytes, &expected);
}
