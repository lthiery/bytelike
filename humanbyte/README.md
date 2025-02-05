[![Continuous Integration](https://github.com/lthiery/humanbyte/actions/workflows/rust.yaml/badge.svg)](https://github.com/lthiery/humanbyte/actions/workflows/rust.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/humanbyte.svg)](https://crates.io/crates/humanbyte)

# HumanByte

HumanByte is a procedural macro crate for deriving `HumanByte` functions for new types of the form `Foo(u64)`.

It provides human-friendly way of constructing and displaying the type with byte units.

## Usage

Add this to your `Cargo.toml`:

### with serde support

```toml
[dependencies]
humanbyte = { version = "0.1", features = ["serde"] }
```

### without serde support

```toml
[dependencies]
humanbyte = { version = "0.1" }
```

### no_std compatible

```toml
[dependencies]
humanbyte = { version = "0.1", no-default-features = true }
```

Next, define your new type and derive `HumanByte` for it:

```rust
use humanbyte_derive::HumanByte;

#[derive(HumanByte)]
pub struct Foo(u64);
```

This will derive all the necessary functions for your new type. You can then use it like this:

```rust
let size = Foo::mb(1);
assert_eq!(size.to_string(), "1.0 MB");
```

See the documentation for [bytescale][bytescale] to see more examples of what you can do with the HumanByte new type.

In addition, if you only want some of the HumanByte functions derived, you can use any of the following derives in an
a la carte fashion:
* HumanByteConstructor
* HumanByteDisplay
* HumanByteParse
* HumanByteOps
* HumanByteFromStr
* HumanByteSerde (requires the `serde` feature)

[bytescale]: https://docs.rs/bytescale/latest/bytescale

