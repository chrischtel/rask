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
