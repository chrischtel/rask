# rask-common

Common types and utilities for the Rask code generation toolkit.

## Features

- Target architecture definitions (`X86_64`, `AArch64`, `RiscV64`)
- ABI specifications (`SystemV`, `Windows`)
- Register classification system
- Memory alignment utilities
- Error handling types

## Example

```rust
use rask_common::{Target, Architecture, Abi};

let target = Target::from_arch(Architecture::X86_64, Abi::SystemV);
let aligned = rask_common::align_to(123, 8); // 128
```