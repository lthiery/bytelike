use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(ByteLike)]
pub fn bytelike(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let constructor = bytelike_constructor(input_str.parse().unwrap());
    let display = bytelike_display(input_str.parse().unwrap());
    let parse = bytelike_parse(input_str.parse().unwrap());
    let arithmetic = bytelike_arithmetic(input_str.parse().unwrap());
    let fromstr = bytelike_fromstr(input_str.parse().unwrap());

    let mut combined = format!(
        "{}{}{}{}{}",
        constructor, display, parse, arithmetic, fromstr
    );
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
    
    let inner_type = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field = fields.unnamed.first().unwrap();
                &field.ty
            }
            _ => panic!("ByteLike can only be derived for tuple structs with exactly one field"),
        },
        _ => panic!("ByteLike can only be derived for tuple structs"),
    };

    let expanded = quote! {
        impl #name {
            #[inline(always)]
            pub const fn b(size: #inner_type) -> Self {
                Self(size)
            }

            #[inline(always)]
            pub const fn kb(size: #inner_type) -> Self {
                Self(size * (bytelike::KB as #inner_type))
            }

            #[inline(always)]
            pub const fn kib(size: #inner_type) -> Self {
                Self(size * (bytelike::KIB as #inner_type))
            }

            #[inline(always)]
            pub const fn mb(size: #inner_type) -> Self {
                Self(size * (bytelike::MB as #inner_type))
            }

            #[inline(always)]
            pub const fn mib(size: #inner_type) -> Self {
                Self(size * (bytelike::MIB as #inner_type))
            }

            #[inline(always)]
            pub const fn gb(size: #inner_type) -> Self {
                Self(size * (bytelike::GB as #inner_type))
            }

            #[inline(always)]
            pub const fn gib(size: #inner_type) -> Self {
                Self(size * (bytelike::GIB as #inner_type))
            }

            #[inline(always)]
            pub const fn tb(size: #inner_type) -> Self {
                Self(size * (bytelike::TB as #inner_type))
            }

            #[inline(always)]
            pub const fn tib(size: #inner_type) -> Self {
                Self(size * (bytelike::TIB as #inner_type))
            }

            #[inline(always)]
            pub const fn pb(size: #inner_type) -> Self {
                Self(size * (bytelike::PB as #inner_type))
            }

            #[inline(always)]
            pub const fn pib(size: #inner_type) -> Self {
                Self(size * (bytelike::PIB as #inner_type))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeArithmetic)]
pub fn bytelike_arithmetic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let inner_type = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field = fields.unnamed.first().unwrap();
                &field.ty
            }
            _ => panic!("ByteLike can only be derived for tuple structs with exactly one field"),
        },
        _ => panic!("ByteLike can only be derived for tuple structs"),
    };

    let expanded = quote! {
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

        impl<T> std::ops::AddAssign<T> for #name
        where
            T: Into<#inner_type>,
        {
            #[inline(always)]
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into();
            }
        }

        impl<T> std::ops::Mul<T> for #name
        where
            T: Into<#inner_type>,
        {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: T) -> #name {
                #name(self.0 * rhs.into())
            }
        }

        impl<T> std::ops::MulAssign<T> for #name
        where
            T: Into<#inner_type>,
        {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: T) {
                self.0 *= rhs.into();
            }
        }

        // Commutative operations for primitive types
        impl std::ops::Add<#name> for #inner_type {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + self)
            }
        }

        impl std::ops::Add<#name> for u32 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as #inner_type))
            }
        }

        impl std::ops::Add<#name> for u16 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as #inner_type))
            }
        }

        impl std::ops::Add<#name> for u8 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as #inner_type))
            }
        }

        impl std::ops::Mul<#name> for #inner_type {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * self)
            }
        }

        impl std::ops::Mul<#name> for u32 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as #inner_type))
            }
        }

        impl std::ops::Mul<#name> for u16 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as #inner_type))
            }
        }

        impl std::ops::Mul<#name> for u8 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as #inner_type))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeDisplay)]
pub fn bytelike_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.pad(&bytelike::to_string(self.0 as u64, true))
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field = fields.unnamed.first().unwrap();
                &field.ty
            }
            _ => panic!("ByteLike can only be derived for tuple structs with exactly one field"),
        },
        _ => panic!("ByteLike can only be derived for tuple structs"),
    };

    let expanded = quote! {
        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if let Ok(v) = value.parse::<#inner_type>() {
                    return Ok(Self(v));
                }
                let number = bytelike::take_while(value, |c| c.is_ascii_digit() || c == '.');
                match number.parse::<f64>() {
                    Ok(v) => {
                        let suffix = bytelike::skip_while(value, |c| {
                            c.is_whitespace() || c.is_ascii_digit() || c == '.'
                        });
                        match suffix.parse::<bytelike::Unit>() {
                            Ok(u) => Ok(Self((v * u64::from(u) as f64) as #inner_type)),
                            Err(error) => Err(format!(
                                "couldn't parse {:?} into a known SI unit, {}",
                                suffix, error
                            )),
                        }
                    }
                    Err(error) => Err(format!(
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

    let expanded = quote! {
        impl #name {
            #[inline(always)]
            pub fn to_string_as(&self, si_unit: bool) -> String {
                bytelike::to_string(self.0, si_unit)
            }

            #[inline(always)]
            pub const fn as_u64(&self) -> u64 {
                self.0
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
