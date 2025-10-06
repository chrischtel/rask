# rask
    A modular, low-level code generation toolkit for Rust.

### Status
This project is in a very early alpha state and is probably gonna stay here a long time.

### What it is
Rask consists of multiple crates that together form a lightweight, highly modular
backend toolkit for building compilers, JITs, or experimental languages in Rust.  
Think of it as a small and understandable alternative to LLVM or Cranelift —
something you can actually read, hack on, and extend.

The goal is not to compete in performance or maturity, but to build a clean and
educational foundation for real code generation. Each module of Rask focuses on
one specific layer of the backend pipeline.

---

### Crates (planned layout)
- **`rask-common`** — Shared types, target definitions, ABI info, error handling, helpers.
- **`rask-x86_64`** — Instruction encoder for the x86-64 architecture.
- **`rask-obj`** — Object file writer (ELF / COFF / Mach-O).
- **`rask-ir`** — A minimal SSA-like intermediate representation.
- **`rask-backend`** — Codegen layer connecting IR to machine code.
- **`rask-cli`** — Command-line utilities and demo compiler.

Each crate can be used independently or together, depending on what you’re building.
For example, `rask-x86_64` alone is already a small standalone assembler library.

---

### Design philosophy
- **Modular:** Every crate stands on its own and exposes a clean public API.
- **Predictable:** No magic macros or hidden codegen; everything is explicit.
- **Educational:** Clear comments and small steps. The goal is to understand, not obscure.
- **Cross-platform:** Support for multiple targets and ABIs (SystemV, Windows x64, etc).
- **Hackable:** Minimal external dependencies. You can fork and modify easily.

---

### Example (Work in Progress)
```rust
use rask_x86_64::{Encoder, Reg64::*};

fn main() {
    let mut enc = Encoder::new();
    enc.mov_reg_imm64(RAX, 1337);
    enc.ret();

    println!("Machine code: {:02x?}", enc.bytes());
}
````

Output:

```
48 b8 39 05 00 00 00 00 00 00 c3
```

That’s a valid function which, when executed, sets `RAX = 1337` and returns.

---

### Long-term vision

Rask aims to become a complete, open, and comprehensible backend foundation for Rust developers
who want to experiment with:

* custom programming languages and DSLs,
* virtual machines or JIT compilers,
* low-level tooling (assemblers, disassemblers, linkers),
* or simply learning how modern compilers emit machine code.

It should always stay **approachable**, **hackable**, and **fun** — a playground for backend developers.

---

### License

MIT

```

---

Would you like me to add a short “Contributing” section next (still in that same calm, friendly tone — explaining that this is experimental but welcomes PRs and discussions)?
```
