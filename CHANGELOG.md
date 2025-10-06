# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [UNRELEASED] -  NONE

### Added
- **rask-common**
- New feature or functionality
- **rask-x86_64**
- New instruction support
- New examples

### Changed
- **rask-common**
- API changes or improvements
- **rask-x86_64**
- Behavior modifications

### Deprecated
- Features that will be removed in future versions

### Removed
- Features that have been removed

### Fixed
- **rask-common**
- Bug fixes
- **rask-x86_64**
- Encoding corrections
- Test fixes

### Security
- Security-related changes

---

## [0.1.0] - 2025-10-6 

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
- Added `add r64, r64`, `sub r64, r64`, and `ret` instruction encoders.
- Unified the `Encoder` API to expose high-level instruction functions.
- Expanded doc comments with Intel SDM references.

### Changed
- Restructured into a unified Cargo workspace with modular sub-crates (`rask-common`, `rask-x86_64`).
- Standardized module naming and internal API consistency.

---

## Template for New Releases

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- **rask-common**
- New feature or functionality
- **rask-x86_64**
- New instruction support
- New examples

### Changed
- **rask-common**
- API changes or improvements
- **rask-x86_64**
- Behavior modifications

### Deprecated
- Features that will be removed in future versions

### Removed
- Features that have been removed

### Fixed
- **rask-common**
- Bug fixes
- **rask-x86_64**
- Encoding corrections
- Test fixes

### Security
- Security-related changes
```

### Release Checklist
When preparing a new release:

1. [ ] Update version numbers in all `Cargo.toml` files
2. [ ] Move items from `[Unreleased]` to the new version section
3. [ ] Add release date in YYYY-MM-DD format
4. [ ] Update `[Unreleased]` section for future development
5. [ ] Run `cargo test` to ensure all tests pass
6. [ ] Run all examples to verify functionality
7. [ ] Update README.md if needed
8. [ ] Create git tag: `git tag vX.Y.Z`
9. [ ] Push changes: `git push origin main --tags`
10. [ ] Publish to crates.io: `cargo publish -p rask-common && cargo publish -p rask-x86_64`
