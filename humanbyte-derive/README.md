## HumanByte-Derive

[![Build Status](https://travis-ci.org/lthiery/humanbyte-derive.svg?branch=master)](https://travis-ci.org/lthiery/humanbyte-derive)
[![Crates.io Version](https://img.shields.io/crates/v/humanbyte-derive.svg)](https://crates.io/crates/humanbyte-derive)

HumanByte Derive is a procedural macro crate for deriving `HumanByte` functions for deriving byte-like new types.

## Usage

Add this to your `Cargo.toml`:

### With serde support

```toml
[dependencies]
humanbyte = { version = "0.1" }
humanbyte-derive = { version = "0.1", features = ["serde"] }
```

### Without serde support

```toml
[dependencies]
humanbyte = { version = "0.1" }
humanbyte-derive = { version = "0.1" }
```

Next, define your new type and derive `HumanByte` for it:

```rust
use humanbyte_derive::HumanByte;

#[derive(HumanByte)]
pub struct ByteSize(u64);
```

This will derive all the necessary functions for your new type. You can then use it like this:

```rust
let size = ByteSize::mb(1);
assert_eq!(size.to_string(), "1.0 MB");
```

See the documentation for ByteSize to see more examples of what you can do with the HumanByte new type (just replace
ByteSize with your own type name).

In addition, if you only want some of the HumanByte functions derived, you can use any of the following derives in an
a la carte fashion:
* HumanByteConstructor
* HumanByteDisplay
* HumanByteParse
* HumanByteOps
* HumanByteFromStr
* HumanByteSerde (requires the `serde` feature)
