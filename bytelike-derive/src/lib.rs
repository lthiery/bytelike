use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(ByteLike)]
pub fn bytelike(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let constructor = bytelike_constructor(input_str.parse().unwrap());
    let display = bytelike_display(input_str.parse().unwrap());
    let parse = bytelike_parse(input_str.parse().unwrap());
    let ops = bytelike_ops(input_str.parse().unwrap());
    let fromstr = bytelike_fromstr(input_str.parse().unwrap());

    let mut combined = format!("{}{}{}{}{}", constructor, display, parse, ops, fromstr);
    if cfg!(feature = "serde") {
        let serde = bytelike_serde(input_str.parse().unwrap());
        combined = format!("{}{}", combined, serde);
    }
    combined.parse().unwrap()
}

#[proc_macro_derive(ByteLikeConstructor)]
pub fn bytelike_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Extract the inner type
    let inner_type = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field = fields.unnamed.first().unwrap();
                    &field.ty
                },
                _ => panic!("ByteLike can only be derived for tuple structs with exactly one field"),
            }
        },
        _ => panic!("ByteLike can only be derived for tuple structs"),
    };

    // Convert the type to tokens for comparison
    let type_tokens = quote!(#inner_type).to_string();

    // Define units with their multipliers and descriptions
    let units = vec![
        ("b", "1", "bytes"),
        ("kb", "bytelike::KB", "kilobytes"),
        ("kib", "bytelike::KIB", "kibibytes"),
        ("mb", "bytelike::MB", "megabytes"),
        ("mib", "bytelike::MIB", "mebibytes"),
        ("gb", "bytelike::GB", "gigabytes"),
        ("gib", "bytelike::GIB", "gibibytes"),
        ("tb", "bytelike::TB", "terabytes"),
        ("tib", "bytelike::TIB", "tebibytes"),
        ("pb", "bytelike::PB", "petabytes"),
        ("pib", "bytelike::PIB", "pebibytes"),
    ];

    // Generate methods
    let methods = units.iter().map(|(fn_name, multiplier, description)| {
        let method_name = syn::Ident::new(fn_name, Span::call_site());
        let multiplier_expr: syn::Expr = syn::parse_str(multiplier).unwrap();
        let doc_comment = format!("Construct `{}` given an amount of {}.", name, description);

        quote! {
            #[doc = #doc_comment]
            #[inline(always)]
            pub const fn #method_name(size: #inner_type) -> Self {
                Self(size * (#multiplier_expr as #inner_type))
            }
        }
    });

    let from_impls = if type_tokens == "u32" {
        quote! {
            impl From<u8> for #name {
                fn from(size: u8) -> #name {
                    Self(size as #inner_type)
                }
            }

            impl From<u16> for #name {
                fn from(size: u16) -> #name {
                    Self(size as #inner_type)
                }
            }
        }
    } else if type_tokens == "u64" {
        quote! {
            impl From<u8> for #name {
                fn from(size: u8) -> #name {
                    Self(size as #inner_type)
                }
            }

            impl From<u16> for #name {
                fn from(size: u16) -> #name {
                    Self(size as #inner_type)
                }
            }

            impl From<u32> for #name {
                fn from(size: u32) -> #name {
                    Self(size as #inner_type)
                }
            }
        }
    } else {
        quote! {
            impl From<u8> for #name {
                fn from(size: u8) -> #name {
                    Self(size as #inner_type)
                }
            }

            impl From<u16> for #name {
                fn from(size: u16) -> #name {
                    Self(size as #inner_type)
                }
            }

            impl From<u32> for #name {
                fn from(size: u32) -> #name {
                    Self(size as #inner_type)
                }
            }

            impl From<u64> for #name {
                fn from(size: u64) -> #name {
                    Self(size as #inner_type)
                }
            }
        }
    };

    let expanded = quote! {
        impl #name {
            #(#methods)*
        }

        #from_impls
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeOps)]
pub fn bytelike_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let inner_type = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field = fields.unnamed.first().unwrap();
                    &field.ty
                },
                _ => panic!("ByteLike can only be derived for tuple structs with exactly one field"),
            }
        },
        _ => panic!("ByteLike can only be derived for tuple structs"),
    };

    let expanded = quote! {
        impl AsRef<#inner_type> for #name {
            fn as_ref(&self) -> &#inner_type {
                &self.0
            }
        }

        impl #name {
            /// Provides `ByteLikeRange` with explicit lower and upper bounds.
            pub fn range<I: Into<Self>>(start: I, stop: I) -> bytelike::ByteLikeRange<#inner_type, Self> {
                bytelike::ByteLikeRange::new(Some(start.into()), Some(stop.into()))
            }

            /// Provides `ByteLikeRange` with explicit lower bound. Upper bound is set to maximum value.
            pub fn range_start<I: Into<Self>>(start: I) -> bytelike::ByteLikeRange<#inner_type, Self> {
                bytelike::ByteLikeRange::new(Some(start.into()), None)
            }

            /// Provides `ByteLikeRange` with explicit upper bound. Lower bound is set to 0.
            pub fn range_stop<I: Into<Self>>(stop: I) -> bytelike::ByteLikeRange<#inner_type, Self> {
                bytelike::ByteLikeRange::new(None, Some(stop.into()))
            }
        }

        impl From<#inner_type> for #name {
            fn from(value: #inner_type) -> Self {
                Self(value)
            }
        }

        impl std::ops::Add<#name> for #name {
            type Output = #name;

            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign<#name> for #name {
            #[inline(always)]
            fn add_assign(&mut self, rhs: #name) {
                self.0 += rhs.0
            }
        }

        impl<T> std::ops::Add<T> for #name
        where
            T: Into<#inner_type>,
        {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: T) -> #name {
                #name(self.0 + rhs.into())
            }
        }

        // ... rest of the operator implementations ...
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeDisplay)]
pub fn bytelike_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.pad(&bytelike::to_string(self.0 as u64, true))
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

#[proc_macro_derive(ByteLikeFromStr)]
pub fn bytelike_fromstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let inner_type = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field = fields.unnamed.first().unwrap();
                    &field.ty
                },
                _ => panic!("ByteLike can only be derived for tuple structs with exactly one field"),
            }
        },
        _ => panic!("ByteLike can only be derived for tuple structs"),
    };

    let expanded = quote! {
        impl core::str::FromStr for #name {
            type Err = alloc::string::String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                use core::str::FromStr;
                if let Ok(v) = #inner_type::from_str(value) {
                    return Ok(Self(v));
                }
                let number = bytelike::take_while(value, |c| c.is_ascii_digit() || c == '.');
                match f64::from_str(number) {
                    Ok(v) => {
                        let suffix = bytelike::skip_while(value, |c| {
                            c.is_ascii_whitespace() || c.is_ascii_digit() || c == '.'
                        });
                        match bytelike::Unit::from_str(suffix) {
                            Ok(u) => Ok(Self((v * u64::from(u) as f64) as #inner_type)),
                            Err(error) => Err(alloc::format!(
                                "couldn't parse {:?} into a known SI unit, {}",
                                suffix, error
                            )),
                        }
                    }
                    Err(error) => Err(alloc::format!(
                        "couldn't parse {:?} into a ByteSize, {}",
                        value, error
                    )),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeParse)]
pub fn bytelike_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Extract the inner type
    let inner_type = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field = fields.unnamed.first().unwrap();
                    &field.ty
                },
                _ => panic!("ByteLike can only be derived for tuple structs with exactly one field"),
            }
        },
        _ => panic!("ByteLike can only be derived for tuple structs"),
    };

    let expanded = quote! {
        impl #name {
            /// Returns the size as a string with an optional SI unit.
            #[inline(always)]
            pub fn to_string_as(&self, si_unit: bool) -> alloc::string::String {
                bytelike::to_string(self.0 as u64, si_unit)
            }

            /// Returns the inner value in its native type
            #[inline(always)]
            pub const fn as_inner(&self) -> #inner_type {
                self.0
            }

            /// Returns the value as u32, if possible
            #[inline(always)]
            pub fn as_u32(&self) -> Option<u32> {
                u32::try_from(self.0).ok()
            }

            /// Returns the value as u64, if possible
            #[inline(always)]
            pub fn as_u64(&self) -> Option<u64> {
                u64::try_from(self.0).ok()
            }

            /// Returns the value as u128
            #[inline(always)]
            pub fn as_u128(&self) -> u128 {
                self.0 as u128
            }

            // For u32 inner type, allow both 32-bit and 64-bit usize conversions
            #[cfg(all(target_pointer_width = "32", same_as = "u32"))]
            #[inline(always)]
            pub fn as_usize(&self) -> Option<usize> {
                Some(self.0 as usize)
            }

            #[cfg(all(target_pointer_width = "64", same_as = "u32"))]
            #[inline(always)]
            pub fn as_usize(&self) -> Option<usize> {
                Some(self.0 as usize)
            }

            // For u64 inner type, only allow 64-bit usize conversions
            #[cfg(all(target_pointer_width = "64", same_as = "u64"))]
            #[inline(always)]
            pub fn as_usize(&self) -> Option<usize> {
                usize::try_from(self.0).ok()
            }

            // For u128 inner type, only allow 64-bit usize conversions
            #[cfg(all(target_pointer_width = "64", same_as = "u128"))]
            #[inline(always)]
            pub fn as_usize(&self) -> Option<usize> {
                usize::try_from(self.0).ok()
            }
        }
    };

    TokenStream::from(expanded)
}
#[proc_macro_derive(ByteLikeSerde)]
pub fn bytelike_serde(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct ByteSizeVistor;

                impl<'de> serde::de::Visitor<'de> for ByteSizeVistor {
                    type Value = #name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("an integer or string")
                    }

                    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                        if let Ok(val) = u64::try_from(value) {
                            Ok(#name(val))
                        } else {
                            Err(E::invalid_value(
                                serde::de::Unexpected::Signed(value),
                                &"integer overflow",
                            ))
                        }
                    }

                    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                        Ok(#name(value))
                    }

                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if let Ok(val) = value.parse() {
                            Ok(val)
                        } else {
                            Err(E::invalid_value(
                                serde::de::Unexpected::Str(value),
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
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
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
