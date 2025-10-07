use rask_common::RegClass;

/// Represents the 16 general-purpose 64-bit registers available in x86_64 mode.
///
/// The numeric `id()` corresponds to the ModR/M and REX register encoding IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg64 {
    RAX,
    RCX,
    RDX,
    RBX,
    RSP,
    RBP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Reg64 {
    /// Returns the 3- or 4-bit register encoding ID used in ModR/M and REX prefixes.
    #[inline(always)]
    pub fn id(self) -> u8 {
        use Reg64::*;
        match self {
            RAX => 0,
            RCX => 1,
            RDX => 2,
            RBX => 3,
            RSP => 4,
            RBP => 5,
            RSI => 6,
            RDI => 7,
            R8 => 8,
            R9 => 9,
            R10 => 10,
            R11 => 11,
            R12 => 12,
            R13 => 13,
            R14 => 14,
            R15 => 15,
        }
    }

    /// Returns the register class — general-purpose in this case.
    #[inline(always)]
    pub fn class(self) -> RegClass {
        RegClass::General
    }

    /// Returns true if this register requires a REX prefix extension (R8–R15).
    #[inline(always)]
    pub fn needs_rex(self) -> bool {
        self.id() >= 8
    }
}
