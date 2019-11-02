#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

mod de;
mod ser;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

fn ser_struct_impl(name: Ident, sd: DataStruct) -> TokenStream {
    let kser = ser::struct_impl::kson_ser(name.clone(), sd.clone());
    let modname = parse_str::<Ident>(&format!("__{}__kser__", name.to_string()))
        .expect("failed to parse module identifier");

    let imp = quote! {
        mod #modname {
            use super::*;
            use ::kson::{
                *,
                de::{De, Deserializer},
                ser::{Ser, Serializer},
                errors::KsonError
            };

            #kser
        }
    };

    imp.into()
}

fn de_struct_impl(name: Ident, sd: DataStruct) -> TokenStream {
    let kde = de::struct_impl::kson_de(name.clone(), sd);
    let modname = parse_str::<Ident>(&format!("__{}__kde__", name.to_string()))
        .expect("failed to parse module identifier");

    let imp = quote! {
        mod #modname {
            use super::*;
            use ::kson::{
                *,
                de::{De, Deserializer},
                ser::{Ser, Serializer},
                errors::KsonError
            };

            #kde
        }
    };

    imp.into()
}

fn ser_enum_impl(name: Ident, sd: DataEnum) -> TokenStream {
    let kser = ser::enum_impl::kson_ser(name.clone(), sd.clone());
    let modname = parse_str::<Ident>(&format!("__{}__kser__", name.to_string()))
        .expect("failed to parse module identifier");

    let imp = quote! {
        mod #modname {
            use super::*;
            use ::kson::{
                *,
                de::{De, Deserializer},
                ser::{Ser, Serializer},
                errors::KsonError
            };

            #kser
        }
    };

    imp.into()
}

fn de_enum_impl(name: Ident, sd: DataEnum) -> TokenStream {
    let kde = de::enum_impl::kson_de(name.clone(), sd);
    let modname = parse_str::<Ident>(&format!("__{}__kde__", name.to_string()))
        .expect("failed to parse module identifier");

    let imp = quote! {

        mod #modname {
            use super::*;
            use ::kson::{
                *,
                de::{De, Deserializer},
                ser::{Ser, Serializer},
                errors::KsonError
            };

            #kde
        }
    };

    imp.into()
}

#[proc_macro_derive(Ser)]
pub fn ser_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_kser_macro(&ast)
}

fn impl_kser_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = ast.ident.clone();

    match ast.data.clone() {
        Data::Struct(sd) => ser_struct_impl(name, sd),
        Data::Enum(ed) => ser_enum_impl(name, ed),
        _ => quote! {}.into(),
    }
}

#[proc_macro_derive(De)]
pub fn de_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_kde_macro(&ast)
}

fn impl_kde_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = ast.ident.clone();

    match ast.data.clone() {
        Data::Struct(sd) => de_struct_impl(name, sd),
        Data::Enum(ed) => de_enum_impl(name, ed),
        _ => quote! {}.into(),
    }
}
