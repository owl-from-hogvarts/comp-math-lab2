# avr-std-stub

[![Crates.io](https://img.shields.io/crates/v/avr-std-stub.svg)](https://crates.io/crates/avr-std-stub)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

[API Documentation](https://docs.rs/avr-std-stub)

Contains simple implementations of required language items that `libstd` normally defines on other targets.

This fixes the following error when compiling for Rust:

```
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`

error: aborting due to 2 previous errors
```

## Usage

Add the following to your crate's `Cargo.toml`:

```toml
[dependencies]
avr-std-stub = "1.0"
```

Then add the following to your crate's `lib.rs` or `main.rs`

```rust
extern crate avr_std_stub;

```

**NOTE**: You **must** add an `extern crate` declaration, otherwise the crate will not be linked
and the definitions it provides will not be used.
