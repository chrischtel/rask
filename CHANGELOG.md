# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]
> Development is ongoing. No official releases yet.

### Added
- **rask-common**
  - Defined base types: `Architecture`, `Abi`, and `Target`.
  - Added utility helpers: `align_to()` and `is_power_of_two()`.
  - Introduced `RaskError` type and `RaskResult` alias.
  - Added word size constants for major architectures.
  - Added `Endianness` enum and basic target information helpers.
  - Added unit tests for alignment and power-of-two validation.

- **rask-x86_64**
  - Initial crate setup with dependency on `rask-common`.
  - Implemented `mov_reg_imm64` instruction encoding.
  - Added high-level `mov` instruction entry point using the new `Operand` system.
  - Introduced the `Operand` enum, allowing pattern matching between registers, immediates, and future memory operands.
  - Added `mov_reg_reg` implementation with proper REX prefix and ModR/M encoding logic.

### Changed
- Restructured into a unified Cargo workspace with modular sub-crates (`rask-common`, `rask-x86_64`).
- Standardized module naming and internal API consistency.

---

## [0.1.0] - *Unreleased Prototype*
> The first milestone for internal testing and bootstrapping.

No published crates yet.
