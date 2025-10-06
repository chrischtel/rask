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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegClass {
    General,
    Float,
    Vector,
    Flags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtualReg(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalReg {
    pub id: u8,
    pub class: RegClass,
}

#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy)]
pub struct Target {
    pub arch: Architecture,
    pub abi: Abi,
    pub pointer_width: u8, // bits
}

impl Target {
    pub fn from_arch(arch: Architecture, abi: Abi) -> Self {
        let pointer_width = match arch {
            Architecture::X86_64 | Architecture::AArch64 | Architecture::RiscV64 => 64,
            Architecture::Other => 64,
        };
        Self {
            arch,
            abi,
            pointer_width,
        }
    }

    pub fn endianness(&self) -> Endianness {
        match self.arch {
            Architecture::X86_64 | Architecture::AArch64 | Architecture::RiscV64 => {
                Endianness::Little
            }
            Architecture::Other => Endianness::Little,
        }
    }

    pub fn host() -> Self {
        let arch = if cfg!(target_arch = "x86_64") {
            Architecture::X86_64
        } else if cfg!(target_arch = "aarch64") {
            Architecture::AArch64
        } else {
            Architecture::Other
        };
        let abi = if cfg!(target_os = "windows") {
            Abi::Windows
        } else {
            Abi::SystemV
        };
        Self::from_arch(arch, abi)
    }
}

pub fn align_to(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub fn is_power_of_two(value: usize) -> bool {
    value != 0 && (value & (value - 1)) == 0
}

#[derive(Debug)]
pub enum RaskError {
    InvalidInstruction,
    UnsupportedAbi,
    Io(std::io::Error),
    Other(String),
}

pub type RaskResult<T> = Result<T, RaskError>;

impl From<std::io::Error> for RaskError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::fmt::Display for RaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInstruction => write!(f, "invalid instruction"),
            Self::UnsupportedAbi => write!(f, "unsupported ABI"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}
impl std::error::Error for RaskError {}

pub const WORD_SIZE_X86_64: usize = 8;
pub const WORD_SIZE_AARCH64: usize = 8;
pub const WORD_SIZE_RISCV64: usize = 8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_to_basic() {
        // Test that 0 is always aligned
        for alignment in [1, 2, 4, 8, 16] {
            assert_eq!(align_to(0, alignment), 0);
        }

        // Test that values align up correctly
        assert_eq!(align_to(1, 2), 2);
        assert_eq!(align_to(2, 2), 2);
        assert_eq!(align_to(3, 2), 4);

        assert_eq!(align_to(1, 4), 4);
        assert_eq!(align_to(3, 4), 4);
        assert_eq!(align_to(4, 4), 4);
        assert_eq!(align_to(5, 4), 8);

        assert_eq!(align_to(1, 8), 8);
        assert_eq!(align_to(7, 8), 8);
        assert_eq!(align_to(8, 8), 8);
        assert_eq!(align_to(9, 8), 16);
    }

    #[test]
    fn test_is_power_of_two() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(8));
        assert!(is_power_of_two(16));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(6));
        assert!(!is_power_of_two(10));
    }
}
