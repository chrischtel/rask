//! x86-64 instruction encoder
//!
//! This module provides low-level helpers for writing machine-code bytes
//! directly into a `Vec<u8>`.  It currently supports a minimal subset of
//! instructions, starting with `mov r64, imm64` and `ret`.

use crate::registers::Reg64;

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
    pub fn emit(&mut self, byte: u8) {
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
    /// ```
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
    pub fn mov_reg_imm64(&mut self, dst: Reg64, value: u64) {
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

    /// Encodes a `RET` (near return) instruction.
    ///
    /// ### Encoding form
    /// ```
    /// C3
    /// ```
    ///
    /// Pops the return address from the stack and jumps to it.
    pub fn ret(&mut self) {
        self.emit(0xC3);
    }
}
