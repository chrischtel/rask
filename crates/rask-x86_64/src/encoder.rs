//! x86-64 instruction encoder
//!
//! This module provides low-level helpers for writing machine-code bytes
//! directly into a `Vec<u8>`.  It currently supports a minimal subset of
//! instructions, starting with `mov r64, imm64` and `ret`.

use crate::{operand::Operand, registers::Reg64};

/// The main byte emitter for x86-64 machine code.
///
/// `Encoder` is intentionally dumb: it simply pushes bytes into an internal
/// buffer.  Higher-level code (assemblers or backends) are responsible for
/// instruction selection and validation.
pub struct Encoder {
    /// Output buffer holding raw machine-code bytes.
    pub buffer: Vec<u8>,
}

impl Encoder {
    /// Constructs an empty encoder.
    #[inline]
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Returns a read-only view of the encoded bytes.
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.buffer
    }

    /// Appends a single byte to the buffer.
    #[inline]
    fn emit(&mut self, byte: u8) {
        self.buffer.push(byte);
    }

    /// Appends a full slice of bytes to the buffer.
    #[inline]
    pub fn emit_all(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    // -------------------------------------------------------------------------
    // Instruction encoders
    // -------------------------------------------------------------------------

    /// Encodes a `MOV r64, imm64` instruction.
    ///
    /// ### Encoding form
    ///
    /// ```text
    /// REX.W + B8+rd  imm64
    /// ```
    ///
    /// * **REX prefix** — 1 byte, of the form `0100WRXB`:
    ///   - **W = 1** → 64-bit operand size
    ///   - **R = 0** → no ModR/M reg-field extension in this form
    ///   - **X = 0** → no SIB index extension
    ///   - **B = (reg_id >> 3)** → extends the low 3-bit reg number to access R8–R15
    ///
    /// * **Opcode** — `0xB8 + (reg_id & 0b111)`
    ///
    /// * **Immediate** — 8-byte little-endian constant
    ///
    /// Example encodings:
    ///
    /// | Instruction      | Bytes (hex)                                |
    /// |------------------|--------------------------------------------|
    /// | `mov rax, 1337`  | 48 B8 39 05 00 00 00 00 00 00             |
    /// | `mov r10, 42`    | 49 BA 2A 00 00 00 00 00 00 00             |
    ///
    /// Reference: Intel SDM Vol. 2A, “MOV—Move” (Opcode B8+rd).
    fn mov_reg_imm64(&mut self, dst: Reg64, value: u64) {
        // Base REX prefix with W=1 (01001000b).
        let mut rex: u8 = 0x48;

        // Set REX.B if the destination register is R8–R15.
        if dst.needs_rex() {
            rex |= 0x01;
        }

        // Emit prefix + opcode + immediate.
        self.emit(rex);
        self.emit(0xB8 + (dst.id() & 0x07));
        self.emit_all(&value.to_le_bytes());
    }

    /// Encodes a `MOV r64, r64` instruction.
    ///
    /// ModR/M Encoding form
    /// Bits 0-2: r/m (destination register or memory operand)
    /// Bits 3-5: reg (source register)
    /// Bits 6-7: mod (addressing mode register-direct or memory)
    ///
    fn mov_reg_reg(&mut self, _dst: Reg64, _src: Reg64) {
        let mut rex: u8 = 0x48; // Base REX prefix with W=1 (01001000b).

        if _src.id() >= 8 {
            rex |= 0x04;
        } // Set REX.R if the source register is R8–R15.
        if _dst.id() >= 8 {
            rex |= 0x01;
        } // Set REX.B if the destination register is R8–R15.

        self.emit(rex);
        self.emit(0x89); // Opcode for MOV r/m64, r64
        let modrm: u8 = (0b11 << 6) | ((_src.id() & 0x07) << 3) | (_dst.id() & 0x07); // ModR/M byte for register to register
        self.emit(modrm);
    }

    /// Encodes a `MOV r64, [mem]` instruction (load from memory).
    ///
    /// ### Encoding form
    /// ```text
    /// REX.W + 8B /r
    /// ```
    ///
    /// * **REX prefix** — 1 byte:
    ///   - **W = 1** → 64-bit operand size
    ///   - **R = (dst_id >> 3)** → extends destination register field
    ///   - **B = (base_id >> 3)** → extends base register field
    ///
    /// * **Opcode** — `0x8B` (MOV r64, r/m64)
    /// * **ModR/M** — depends on addressing mode
    /// * **Displacement** — 0, 1, or 4 bytes depending on addressing mode
    fn mov_reg_mem(&mut self, dst: Reg64, src: &crate::operand::MemOperand) {
        let mut rex: u8 = 0x48; // Base REX.W

        // Set REX.R if destination register needs extension
        if dst.needs_rex() {
            rex |= 0x04;
        }

        // Set REX.B if base register needs extension
        if src.base.needs_rex() {
            rex |= 0x01;
        }

        self.emit(rex);
        self.emit(0x8B); // MOV r64, r/m64

        // Determine addressing mode based on displacement
        let (mod_bits, disp_bytes) = if src.disp == 0 && src.base != crate::registers::Reg64::RBP {
            // [reg] - no displacement (except RBP which requires disp8)
            (0b00, Vec::new())
        } else if src.disp >= -128 && src.disp <= 127 {
            // [reg + disp8] - 8-bit displacement
            (0b01, vec![src.disp as i8 as u8])
        } else {
            // [reg + disp32] - 32-bit displacement
            (0b10, src.disp.to_le_bytes().to_vec())
        };

        // Build ModR/M byte: mod(2) + reg(3) + r/m(3)
        let modrm = (mod_bits << 6) | ((dst.id() & 0x07) << 3) | (src.base.id() & 0x07);
        self.emit(modrm);

        // Emit displacement bytes
        self.emit_all(&disp_bytes);
    }

    /// Encodes a `MOV [mem], r64` instruction (store to memory).
    ///
    /// ### Encoding form
    /// ```text
    /// REX.W + 89 /r
    /// ```
    ///
    /// * **REX prefix** — 1 byte:
    ///   - **W = 1** → 64-bit operand size
    ///   - **R = (src_id >> 3)** → extends source register field
    ///   - **B = (base_id >> 3)** → extends base register field
    ///
    /// * **Opcode** — `0x89` (MOV r/m64, r64)
    /// * **ModR/M** — depends on addressing mode
    /// * **Displacement** — 0, 1, or 4 bytes depending on addressing mode
    fn mov_mem_reg(&mut self, dst: &crate::operand::MemOperand, src: Reg64) {
        let mut rex: u8 = 0x48; // Base REX.W

        // Set REX.R if source register needs extension
        if src.needs_rex() {
            rex |= 0x04;
        }

        // Set REX.B if base register needs extension
        if dst.base.needs_rex() {
            rex |= 0x01;
        }

        self.emit(rex);
        self.emit(0x89); // MOV r/m64, r64

        // Determine addressing mode based on displacement
        let (mod_bits, disp_bytes) = if dst.disp == 0 && dst.base != crate::registers::Reg64::RBP {
            // [reg] - no displacement (except RBP which requires disp8)
            (0b00, Vec::new())
        } else if dst.disp >= -128 && dst.disp <= 127 {
            // [reg + disp8] - 8-bit displacement
            (0b01, vec![dst.disp as i8 as u8])
        } else {
            // [reg + disp32] - 32-bit displacement
            (0b10, dst.disp.to_le_bytes().to_vec())
        };

        // Build ModR/M byte: mod(2) + reg(3) + r/m(3)
        let modrm = (mod_bits << 6) | ((src.id() & 0x07) << 3) | (dst.base.id() & 0x07);
        self.emit(modrm);

        // Emit displacement bytes
        self.emit_all(&disp_bytes);
    }

    /// Encodes an `ADD r64, r64` instruction.
    ///
    /// ### Encoding form
    /// ```text
    /// REX.W + 01 /r
    /// ```
    /// * **REX prefix** — 1 byte, of the form `0100WRXB`:
    ///   - **W = 1** → 64-bit operand size
    ///  - **R = (src_id >> 3)** → extends the low 3-bit reg number of the source register to access R8–R15
    ///  - **X = 0** → no SIB index extension
    /// - **B = (dst_id >> 3)** → extends the low 3-bit reg number of the destination register to access R8–R15
    /// * **Opcode** — `0x01`
    /// * **ModR/M** — ModR/M byte specifying the registers:
    ///   - Bits 0-2: r/m (destination register)
    ///  - Bits 3-5: reg (source register)
    ///  - Bits 6-7: mod (addressing mode, `11` for register-direct)
    /// Example encoding:
    /// | Instruction      | Bytes (hex)                                |
    /// |------------------|--------------------------------------------|
    /// | `add rax, rbx`   | 48 01 D8                                   |
    /// | `add r10, r9`    | 49 01 D1                                   |
    /// Reference: Intel SDM Vol. 2A, "ADD—Add" (Opcode 01 /r).
    /// Note: This implementation currently only supports register-to-register addition.
    /// Memory operands and immediate values are not yet implemented.
    /// TODO: Extend support for other operand types in the future.
    /// Panics if either operand is not a register.
    /// Currently, only `Reg` operands are supported.
    pub fn add(&mut self, _dst: Reg64, _src: Reg64) {
        let mut rex: u8 = 0x48; // Base REX prefix with W=1 (01001000b).

        if _src.id() >= 8 {
            rex |= 0x04;
        } // Set REX.R if the source register is R8–R15.
        if _dst.id() >= 8 {
            rex |= 0x01;
        } // Set REX.B if the destination register is R8–R15.

        self.emit(rex);
        self.emit(0x01); // Opcode for ADD r/m64, r64
        let modrm: u8 = (0b11 << 6) | ((_src.id() & 0x07) << 3) | (_dst.id() & 0x07); // ModR/M byte for register to register
        self.emit(modrm);
    }

    pub fn sub(&mut self, _dst: Reg64, _src: Reg64) {
        let mut rex: u8 = 0x48; // Base REX prefix with W=1 (01001000b).

        if _src.id() >= 8 {
            rex |= 0x04;
        } // Set REX.R if the source register is R8–R15.
        if _dst.id() >= 8 {
            rex |= 0x01;
        } // Set REX.B if the destination register is R8–R15.

        self.emit(rex);
        self.emit(0x29); // Opcode for SUB r/m64, r64
        let modrm: u8 = (0b11 << 6) | ((_src.id() & 0x07) << 3) | (_dst.id() & 0x07); // ModR/M byte for register to register
        self.emit(modrm);
    }

    pub fn mov(&mut self, dst: Operand, src: Operand) {
        match (dst, src) {
            (Operand::Reg(d), Operand::Reg(s)) => self.mov_reg_reg(d, s),
            (Operand::Reg(d), Operand::Imm(imm)) => self.mov_reg_imm64(d, imm as u64),
            (Operand::Mem(ref m), Operand::Reg(r)) => self.mov_mem_reg(m, r),
            (Operand::Reg(r), Operand::Mem(ref m)) => self.mov_reg_mem(r, m),
            (Operand::Mem(_), Operand::Mem(_)) => {
                panic!("MOV from memory to memory is invalid on x86-64");
            }
            (Operand::Imm(_), _) => {
                panic!("Cannot move to an immediate value");
            }
            (Operand::Mem(_), Operand::Imm(_)) => {
                todo!("mov [mem], imm not yet implemented");
            }
        }
    }

    /// Encodes a `RET` (near return) instruction.
    ///
    /// ### Encoding form
    /// ```text
    /// C3
    /// ```
    ///
    /// Pops the return address from the stack and jumps to it.
    pub fn ret(&mut self) {
        self.emit(0xC3);
    }
}
