#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    AArch64,
    RiscV64,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abi {
    SystemV,
    Windows,
}

#[derive(Debug, Clone, Copy)]
pub struct Target {
    pub arch: Architecture,
    pub abi: Abi,
    pub pointer_width: u8, // in bits
}

pub fn align_to(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub fn is_power_of_two(value: usize) -> bool {
    value != 0 && (value & (value - 1)) == 0
}
