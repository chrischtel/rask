# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]
> Development is ongoing. No official releases yet.

### Added
- **rask-common**
  - Base architecture for `Architecture`, `Abi`, and `Target` structs.
  - Utility helpers: `align_to()`, `is_power_of_two()`.
  - Error type `RaskError` and alias `RaskResult`.
  - Word size constants for major architectures.
  - `Endianness` enum and placeholder for future target methods.
  - Unit tests for alignment and power-of-two functions.
- **rask-x86_64**
  - Initial project setup.
  - Dependency on `rask-common`.

### Changed
- Project restructured into a Cargo workspace with sub-crates.
- Standardized internal module naming conventions (`rask-common`, `rask-x86_64`).

### Planned
- Implement instruction encoder in `rask-x86_64`.
- Introduce `rask-obj` crate for ELF/COFF object generation.
- Define calling conventions for SystemV and Windows ABI.

---

## [0.1.0] - *Unreleased Prototype*
> The first milestone for internal testing and bootstrapping.

No published crates yet.
