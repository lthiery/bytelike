use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HumanByte)]
pub fn humanbyte(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let constructor = humanbyte_constructor(input_str.parse().unwrap());
    let display = humanbyte_display(input_str.parse().unwrap());
    let parse = humanbyte_parse(input_str.parse().unwrap());
    let ops = humanbyte_ops(input_str.parse().unwrap());
    let fromstr = humanbyte_fromstr(input_str.parse().unwrap());

    let mut combined = format!("{}{}{}{}{}", constructor, display, parse, ops, fromstr);
    if cfg!(feature = "serde") {
        let serde = humanbyte_serde(input_str.parse().unwrap());
        combined = format!("{}{}", combined, serde);
    }
    combined.parse().unwrap()
}

#[proc_macro_derive(HumanByteConstructor)]
pub fn humanbyte_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Define units with their multipliers and descriptions
    let units = vec![
        ("b", "1", "bytes"),
        ("kb", "::humanbyte::KB", "kilobytes"),
        ("kib", "::humanbyte::KIB", "kibibytes"),
        ("mb", "::humanbyte::MB", "megabytes"),
        ("mib", "::humanbyte::MIB", "mebibytes"),
        ("gb", "::humanbyte::GB", "gigabytes"),
        ("gib", "::humanbyte::GIB", "gibibytes"),
        ("tb", "::humanbyte::TB", "terabytes"),
        ("tib", "::humanbyte::TIB", "tebibytes"),
        ("pb", "::humanbyte::PB", "petabytes"),
        ("pib", "::humanbyte::PIB", "pebibytes"),
    ];

    // Generate methods
    let methods = units.iter().map(|(fn_name, multiplier, description)| {
        // Create an identifier for the method name
        let method_name = syn::Ident::new(fn_name, Span::call_site());

        // Parse the multiplier into an expression
        let multiplier_expr: syn::Expr = syn::parse_str(multiplier).unwrap();

        // Generate the documentation comment
        let doc_comment = format!("Construct `{}` given an amount of {}.", name, description);

        // Generate the method using quote!
        quote! {
            #[doc = #doc_comment]
            #[inline(always)]
            pub const fn #method_name(size: u64) -> Self {
                Self(size * #multiplier_expr)
            }
        }
    });

    let expanded = quote! {
        impl #name {
            #(#methods)*
        }

        impl From<u64> for #name {
            fn from(size: u64) -> #name {
                Self(size)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(HumanByteOps)]
pub fn humanbyte_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl core::ops::Add<#name> for #name {
            type Output = #name;

            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(self.0 + rhs.0)
            }
        }

        impl core::ops::AddAssign<#name> for #name {
            #[inline(always)]
            fn add_assign(&mut self, rhs: #name) {
                self.0 += rhs.0
            }
        }

        impl<T> core::ops::Add<T> for #name
        where
            T: Into<u64>,
        {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: T) -> #name {
                #name(self.0 + (rhs.into()))
            }
        }

        impl<T> core::ops::AddAssign<T> for #name
        where
            T: Into<u64>,
        {
            #[inline(always)]
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into();
            }
        }

        impl core::ops::Sub<#name> for #name {
            type Output = #name;

            #[inline(always)]
            fn sub(self, rhs: #name) -> #name {
                #name(self.0 - rhs.0)
            }
        }

        impl core::ops::SubAssign<#name> for #name {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: #name) {
                self.0 -= rhs.0
            }
        }

        impl<T> core::ops::Sub<T> for #name
        where
            T: Into<u64>,
        {
            type Output = #name;

            #[inline(always)]
            fn sub(self, rhs: T) -> #name {
                #name(self.0 - (rhs.into()))
            }
        }

        impl<T> core::ops::SubAssign<T> for #name
        where
            T: Into<u64>,
        {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: T) {
                self.0 -= rhs.into();
            }
        }

        impl<T> core::ops::Mul<T> for #name
        where
            T: Into<u64>,
        {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: T) -> #name {
                #name(self.0 * rhs.into())
            }
        }

        impl<T> core::ops::MulAssign<T> for #name
        where
            T: Into<u64>,
        {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: T) {
                self.0 *= rhs.into();
            }
        }

        impl core::ops::Add<#name> for u64 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + self)
            }
        }

        impl core::ops::Add<#name> for u32 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as u64))
            }
        }

        impl core::ops::Add<#name> for u16 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as u64))
            }
        }

        impl core::ops::Add<#name> for u8 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as u64))
            }
        }

        impl core::ops::Mul<#name> for u64 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * self)
            }
        }

        impl core::ops::Mul<#name> for u32 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as u64))
            }
        }

        impl core::ops::Mul<#name> for u16 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as u64))
            }
        }

        impl core::ops::Mul<#name> for u8 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as u64))
            }
        }

        impl #name {
            /// Provides `HumanByteRange` with explicit lower and upper bounds.
            pub fn range<I: Into<Self>>(start: I, stop: I) -> ::humanbyte::HumanByteRange<Self> {
                ::humanbyte::HumanByteRange::new(Some(start), Some(stop))
            }

            /// Provides `HumanByteRange` with explicit lower bound. Upper bound is set to `u64::MAX`.
            pub fn range_start<I: Into<Self>>(start: I) -> ::humanbyte::HumanByteRange<Self> {
                ::humanbyte::HumanByteRange::new(Some(start), None)
            }

            /// Provides `HumanByteRange` with explicit lower bound. Upper bound is set to `u64::MAX`.
            pub fn range_stop<I: Into<Self>>(stop: I) -> ::humanbyte::HumanByteRange<Self> {
                ::humanbyte::HumanByteRange::new(None, Some(stop.into()))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(HumanByteDisplay)]
pub fn humanbyte_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.pad(&::humanbyte::to_string(self.0, ::humanbyte::Format::IEC))
            }
        }

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(HumanByteFromStr)]
pub fn humanbyte_fromstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl core::str::FromStr for #name {
            type Err = ::humanbyte::String;

            fn from_str(value: &str) -> core::result::Result<Self, Self::Err> {
                if let Ok(v) = value.parse::<u64>() {
                    return Ok(Self(v));
                }
                let number = ::humanbyte::take_while(value, |c| c.is_ascii_digit() || c == '.');
                match number.parse::<f64>() {
                    Ok(v) => {
                        let suffix = ::humanbyte::skip_while(&value[number.len()..], char::is_whitespace);
                        match suffix.parse::<::humanbyte::Unit>() {
                            Ok(u) => Ok(Self((v * u64::from(u) as f64) as u64)),
                            Err(error) => Err(::humanbyte::format!(
                                "couldn't parse {:?} into a known SI unit, {}",
                                suffix, error
                            )),
                        }
                    }
                    Err(error) => Err(::humanbyte::format!(
                        "couldn't parse {:?} into a ByteSize, {}",
                        value, error
                    )),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(HumanByteParse)]
pub fn humanbyte_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            /// Returns the size as a string with an optional SI unit.
            #[inline(always)]
            pub fn to_string_as(&self, format: ::humanbyte::Format) -> ::humanbyte::String {
                ::humanbyte::to_string(self.0, format)
            }

            /// Returns the inner u64 value.
            #[inline(always)]
            pub const fn as_u64(&self) -> u64 {
                self.0
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(HumanByteSerde)]
pub fn humanbyte_serde(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl<'de> ::humanbyte::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: ::humanbyte::serde::Deserializer<'de>,
            {
                struct ByteSizeVistor;

                impl<'de> ::humanbyte::serde::de::Visitor<'de> for ByteSizeVistor {
                    type Value = #name;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        formatter.write_str("an integer or string")
                    }

                    fn visit_i64<E: ::humanbyte::serde::de::Error>(self, value: i64) -> core::result::Result<Self::Value, E> {
                        if let Ok(val) = u64::try_from(value) {
                            Ok(#name(val))
                        } else {
                            Err(E::invalid_value(
                                ::humanbyte::serde::de::Unexpected::Signed(value),
                                &"integer overflow",
                            ))
                        }
                    }

                    fn visit_u64<E: ::humanbyte::serde::de::Error>(self, value: u64) -> core::result::Result<Self::Value, E> {
                        Ok(#name(value))
                    }

                    fn visit_str<E: ::humanbyte::serde::de::Error>(self, value: &str) -> core::result::Result<Self::Value, E> {
                        if let Ok(val) = value.parse() {
                            Ok(val)
                        } else {
                            Err(E::invalid_value(
                                ::humanbyte::serde::de::Unexpected::Str(value),
                                &"parsable string",
                            ))
                        }
                    }
                }

                if deserializer.is_human_readable() {
                    deserializer.deserialize_any(ByteSizeVistor)
                } else {
                    deserializer.deserialize_u64(ByteSizeVistor)
                }
            }
        }
        impl ::humanbyte::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: ::humanbyte::serde::Serializer,
            {
                if serializer.is_human_readable() {
                    <str>::serialize(self.to_string().as_str(), serializer)
                } else {
                    self.0.serialize(serializer)
                }
            }
        }
    };

    TokenStream::from(expanded)
}
