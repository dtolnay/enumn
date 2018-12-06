extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Meta, NestedMeta};

#[proc_macro_derive(N)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        Data::Struct(_) | Data::Union(_) => panic!("input must be an enum"),
    };

    for variant in &variants {
        match variant.fields {
            Fields::Unit => {}
            Fields::Named(_) | Fields::Unnamed(_) => {
                let span = variant.ident.span();
                let err = Error::new(span, "enumn: variant with data is not supported");
                return err.to_compile_error().into();
            }
        }
    }

    // Parse repr attribute like #[repr(u16)].
    let mut repr = None;
    for attr in input.attrs {
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            if list.ident == "repr" {
                if let Some(NestedMeta::Meta(Meta::Word(word))) = list.nested.into_iter().next() {
                    match word.to_string().as_str() {
                        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32"
                        | "i64" | "i128" | "isize" => {
                            repr = Some(attr.tts);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let signature;
    let value;
    match repr {
        Some(ref repr) => {
            signature = quote! {
                fn n(value: #repr)
            };
            value = quote!(value);
        }
        None => {
            repr = Some(quote!(i64));
            signature = quote! {
                fn n<REPR: Into<i64>>(value: REPR)
            };
            value = quote! {
                <REPR as Into<i64>>::into(value)
            };
        }
    }

    let ident = input.ident;
    let declare_discriminants = variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            const #variant: #repr = #ident::#variant as #repr;
        }
    });
    let match_discriminants = variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            discriminant::#variant => Some(#ident::#variant),
        }
    });

    TokenStream::from(quote! {
        impl #ident {
            pub #signature -> Option<Self> {
                struct discriminant;
                impl discriminant {
                    #(#declare_discriminants)*
                }
                match #value {
                    #(#match_discriminants)*
                    _ => None,
                }
            }
        }
    })
}
