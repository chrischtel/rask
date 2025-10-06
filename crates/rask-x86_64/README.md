# rask-x86_64

x86_64 instruction encoder for the Rask project.

## Features

- Encode x86_64 instructions to machine code bytes
- Support for MOV, ADD, SUB, RET instructions
- Proper REX prefix handling for extended registers (R8-R15)
- Type-safe register and operand system

## Example

```rust
use rask_x86_64::{encoder::Encoder, registers::Reg64::*, operand::Operand};

let mut encoder = Encoder::new();
encoder.mov(Operand::Reg(RAX), Operand::Imm(1337));
encoder.add(RAX, RBX);
encoder.ret();

let bytes = encoder.bytes(); // Raw x86_64 machine code
```

## Examples

Run the included examples:

```bash
cargo run --example basic_encoding
cargo run --example rex_prefixes
cargo run --example arithmetic
```
