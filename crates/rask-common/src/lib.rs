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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_to() {
        assert_eq!(align_to(0, 1), 0);
        assert_eq!(align_to(1, 1), 1);
        assert_eq!(align_to(2, 1), 2);
        assert_eq!(align_to(3, 1), 3);
        assert_eq!(align_to(4, 1), 4);
        assert_eq!(align_to(5, 1), 5);
        assert_eq!(align_to(6, 1), 6);
        assert_eq!(align_to(7, 1), 7);
        assert_eq!(align_to(8, 1), 8);
        assert_eq!(align_to(9, 1), 9);

        assert_eq!(align_to(0, 2), 0);
        assert_eq!(align_to(1, 2), 2);
        assert_eq!(align_to(2, 2), 2);
        assert_eq!(align_to(3, 2), 4);
        assert_eq!(align_to(4, 2), 4);
        assert_eq!(align_to(5, 2), 6);
        assert_eq!(align_to(6, 2), 6);
        assert_eq!(align_to(7, 2), 8);
        assert_eq!(align_to(8, 2), 8);
        assert_eq!(align_to(9, 2), 10);

        assert_eq!(align_to(0, 4), 0);
        assert_eq!(align_to(1, 4), 4);
        assert_eq!(align_to(2, 4), 4);
        assert_eq!(align_to(3, 4), 4);
        assert_eq!(align_to(4, 4), 4);
        assert_eq!(align_to(5, 4), 8);
        assert_eq!(align_to(6, 4), 8);
        assert_eq!(align_to(7, 4), 8);
        assert_eq!(align_to(8, 4), 8);
        assert_eq!(align_to(9, 4), 12);

        assert_eq!(align_to(0, 8), 0);
        assert_eq!(align_to(1, 8), 8);
        assert_eq!(align_to(2, 8), 8);
        assert_eq!(align_to(3, 8), 8);
        assert_eq!(align_to(4, 8), 8);
        assert_eq!(align_to(5, 8), 8);
        assert_eq!(align_to(6, 8), 8);
        assert_eq!(align_to(7, 8), 8);
        assert_eq!(align_to(8, 8), 8);
        assert_eq!(align_to(9, 8), 16);
    }

    #[test]
    fn test_is_power_of_two() {
        assert!(is_power_of_two(0));
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(3));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(5));
        assert!(is_power_of_two(6));
        assert!(is_power_of_two(7));
        assert!(is_power_of_two(8));
        assert!(is_power_of_two(9));

        assert!(!is_power_of_two(10));
        assert!(!is_power_of_two(11));
        assert!(!is_power_of_two(12));
        assert!(!is_power_of_two(13));
        assert!(!is_power_of_two(14));
        assert!(!is_power_of_two(15));
        assert!(!is_power_of_two(16));
        assert!(!is_power_of_two(17));
        assert!(!is_power_of_two(18));
        assert!(!is_power_of_two(19));
    }   
}