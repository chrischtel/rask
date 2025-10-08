# Rask - x86_64 Instruction Encoder & Code Generation Toolkit

**An experimental x86_64 instruction encoding library for Rust. Early development - expect breaking changes.**

Rask is a learning-focused x86_64 instruction encoder that lets you generate machine code programmatically. If you want to experiment with low-level code generation, learn x86_64 encoding, or prototype JIT ideas - Rask gives you a clean foundation to build on.

⚠️ **Early Development Warning**: Rask is still very early — the API will change, many instructions are missing, and it’s not ready for production.  
But it’s already a great playground if you want to learn how x86_64 encoding actually works.

## Why Rask?

**Educational First** - Every instruction is documented with Intel SDM references. You'll actually understand what's happening.

**Lightweight & Hackable** - When LLVM is overkill and you want to understand the basics, Rask is small enough to read and modify.

**Type-Safe Foundation** - Catch encoding errors at compile time instead of debugging invalid machine code.

**Correct Encoding** - What's implemented generates byte-perfect machine code with comprehensive tests.

## Quick Start

Add Rask to your project:

```toml
[dependencies]
rask-x86_64 = "0.1.0"
rask-common = "0.1.0"
```

Generate your first machine code:

```rust
use rask_x86_64::{encoder::Encoder, registers::Reg64::*, operand::Operand};

let mut encoder = Encoder::new();

// mov rax, 1337
encoder.mov(Operand::Reg(RAX), Operand::Imm(1337));

// add rax, rbx  
encoder.add(RAX, RBX);

// ret
encoder.ret();

let machine_code = encoder.bytes();
// Output: [0x48, 0xb8, 0x39, 0x05, ...]
```

## Supported Instructions

**Memory Operations**
- `mov reg, [mem]` - Load from memory
- `mov [mem], reg` - Store to memory  
- `mov reg, reg` - Register to register
- `mov reg, immediate` - Load immediate values

**Arithmetic**
- `add reg, reg` - 64-bit addition
- `sub reg, reg` - 64-bit subtraction

**Control Flow**
- `ret` - Function return

**Coming Soon:** Jump instructions, more arithmetic, stack operations, function calls

## Advanced Features

**Memory Addressing with Displacement**
```rust
use rask_x86_64::operand::MemOperand;

// mov rax, [rbx + 8]
let mem = MemOperand { base: RBX, disp: 8 };
encoder.mov(Operand::Reg(RAX), Operand::Mem(mem));
```

**Extended Register Support (R8-R15)**
```rust
// Automatic REX prefix handling
encoder.mov(Operand::Reg(R10), Operand::Imm(42));
encoder.add(R8, R9);
```

**Cross-Platform Target Support**
```rust
use rask_common::{Target, Architecture, Abi};

let target = Target::from_arch(Architecture::X86_64, Abi::SystemV);
// Use target info for platform-specific code generation
```

## Examples

Run the included examples to see Rask in action:

```bash
# Basic instruction encoding
cargo run --example basic_encoding

# REX prefix demonstration  
cargo run --example rex_prefixes

# Arithmetic operations
cargo run --example arithmetic
```

## Use Cases

**JIT Compilers** - Generate machine code at runtime for dynamic languages or DSLs

**Assembly Tools** - Build custom assemblers or code analysis tools

**Compiler Backends** - Use as a backend for your programming language

**Educational Projects** - Learn x86_64 instruction encoding with clear, documented examples

**Performance Critical Code** - Generate optimized machine code for specific algorithms

## Architecture

Rask is built as a modular workspace:

- **`rask-common`** - Shared types, target definitions, utilities
- **`rask-x86_64`** - x86_64 instruction encoding
- **`rask-aarch64`** - ARM64 support (planned)

#### _*more crates are coming in the future*_

## Documentation

Each instruction encoder includes comprehensive documentation with:
- Intel SDM references
- Encoding format details
- Example byte sequences
- ModR/M and REX prefix explanations

## Testing

Rask includes extensive test coverage with byte-level verification:

```bash
cargo test
```

Every instruction is tested against known-good byte sequences to ensure correctness.

## Contributing

Rask is designed to be hackable and extensible. Adding new instructions is straightforward:

1. Implement the encoding logic
2. Add comprehensive tests  
3. Document with Intel SDM references
4. Submit a pull request

See our examples for instruction implementation patterns.

## Roadmap

**Current:** x86_64 core instructions, memory operations
**Next:** Jump instructions, function calls, stack operations  
**Future:** ARM64 support, high-level code generation, optimization passes

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.