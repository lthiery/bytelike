## ByteScale

[![CI](https://github.com/lthiery/humanbyte/actions/workflows/rust.yaml/badge.svg)](https://github.com/lthiery/humanbyte/actions/workflows/rust.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/bytescale.svg)](https://crates.io/crates/bytescale)

`ByteScale` is a utility for human-readable byte count representations.

Features:

- Pre-defined constants for various size units (e.g., B, KB, KiB, MB, MiB, GB, GiB, ... PiB).
- `ByteScale` type which presents size units convertible to different size units.
- Arithmetic operations for `ByteScale`.
- FromStr impl for `ByteScale`, allowing to parse from string size representations like 1.5KiB and 521TiB.
- Serde support for binary and human-readable deserializers like JSON.

[API Documentation](https://docs.rs/bytescale)

## Example

### Human readable representations (SI unit and Binary unit)

```rust
use bytescale::ByteScale;

macro_rules! assert_display {
        ($expected:expr, $bytescale:expr) => {
            assert_eq!($expected, format!("{}", $bytescale));
        };
    }

fn test_display() {
    assert_display!("215 B", ByteScale::b(215));
    assert_display!("1.0 KiB", ByteScale::kib(1));
    assert_display!("301.0 KiB", ByteScale::kib(301));
    assert_display!("419.0 MiB", ByteScale::mib(419));
    assert_display!("518.0 GiB", ByteScale::gib(518));
    assert_display!("815.0 TiB", ByteScale::tib(815));
    assert_display!("609.0 PiB", ByteScale::pib(609));
}

fn test_display_alignment() {
    assert_eq!("|357 B     |", format!("|{:10}|", ByteScale(357)));
    assert_eq!("|     357 B|", format!("|{:>10}|", ByteScale(357)));
    assert_eq!("|357 B     |", format!("|{:<10}|", ByteScale(357)));
    assert_eq!("|  357 B   |", format!("|{:^10}|", ByteScale(357)));

    assert_eq!("|-----357 B|", format!("|{:->10}|", ByteScale(357)));
    assert_eq!("|357 B-----|", format!("|{:-<10}|", ByteScale(357)));
    assert_eq!("|--357 B---|", format!("|{:-^10}|", ByteScale(357)));
}

macro_rules! assert_to_string {
    ($expected:expr, $actual:expr, $si:expr) => {
        assert_eq!($expected.to_string(), $actual.to_string_as($si));
    };
}

fn test_to_string_as() {
    use humanbyte::Format;
    assert_to_string!("215 B", ByteScale::b(215), Format::IEC);
    assert_to_string!("215 B", ByteScale::b(215), Format::SI);

    assert_to_string!("1.0 KiB", ByteScale::kib(1), Format::IEC);
    assert_to_string!("1.0 kB", ByteScale::kib(1), Format::SI);

    assert_to_string!("293.9 KiB", ByteScale::kb(301), Format::IEC);
    assert_to_string!("301.0 kB", ByteScale::kb(301), Format::SI);

    assert_to_string!("1.0 MiB", ByteScale::mib(1), Format::IEC);
    assert_to_string!("1.0 MB", ByteScale::mib(1), Format::SI);

    assert_to_string!("1.9 GiB", ByteScale::mib(1907), Format::IEC);
    assert_to_string!("2.0 GB", ByteScale::mib(1908), Format::SI);

    assert_to_string!("399.6 MiB", ByteScale::mb(419), Format::IEC);
    assert_to_string!("419.0 MB", ByteScale::mb(419), Format::SI);

    assert_to_string!("482.4 GiB", ByteScale::gb(518), Format::IEC);
    assert_to_string!("518.0 GB", ByteScale::gb(518), Format::SI);

    assert_to_string!("741.2 TiB", ByteScale::tb(815), Format::IEC);
    assert_to_string!("815.0 TB", ByteScale::tb(815), Format::SI);

    assert_to_string!("540.9 PiB", ByteScale::pb(609), Format::IEC);
    assert_to_string!("609.0 PB", ByteScale::pb(609), Format::SI);
}
```

### Arithmetic operations

```rust
use bytescale::ByteScale;

fn byte_arithmetic_operator() {
    let x = ByteScale::mb(1);
    let y = ByteScale::kb(100);

    let plus = x + y;
    print!("{}", plus);

    let minus = ByteScale::tb(100) + ByteScale::gb(4);
    print!("{}", minus);
}
```
