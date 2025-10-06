use crate::registers::Reg64;


/// Represents any operand that can appear in an instruction.
/// 
/// For now, only `Reg` is implemented.
/// `Mem` will be added later for memory addressing modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    /// A 64-bit general-purpose register.
    Reg(Reg64),

    /// A memory operand (base register + displacement).
    /// TODO: implement encoding logic for this variant.
    Mem(MemOperand),

    /// A 64-bit immediate constant.
    Imm(i64),
}


/// Describes a memory operand.
/// 
/// For now, only the structure is defined — no encoding logic yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemOperand {
    pub base: Reg64,
    pub disp: i32,
}