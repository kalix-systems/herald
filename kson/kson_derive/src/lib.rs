#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

mod de;
mod ser;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

fn ser_struct_impl(
    name: Ident,
    sd: DataStruct,
    gens: Generics,
) -> TokenStream {
    let kser = ser::struct_impl::kson_ser(name.clone(), sd, gens);
    let modname = parse_str::<Ident>(&format!("__{}__kser__", name.to_string()))
        .expect("failed to parse module identifier");

    let imp = quote! {
        mod #modname {
            use super::*;
            use kson::{
                *,
                de::{De, Deserializer},
                ser::{Ser, Serializer},
                errors::KsonError,
                prelude::*
            };

            #kser
        }
    };

    imp.into()
}

fn de_struct_impl(
    name: Ident,
    sd: DataStruct,
    gens: Generics,
) -> TokenStream {
    let kde = de::struct_impl::kson_de(name.clone(), sd, gens);
    let modname = parse_str::<Ident>(&format!("__{}__kde__", name.to_string()))
        .expect("failed to parse module identifier");

    let imp = quote! {
        mod #modname {
            use super::*;
            use kson::{
                *,
                de::{De, Deserializer},
                ser::{Ser, Serializer},
                errors::KsonError,
                prelude::*
            };

            #kde
        }
    };

    imp.into()
}

fn ser_enum_impl(
    name: Ident,
    sd: DataEnum,
    gens: Generics,
) -> TokenStream {
    let kser = ser::enum_impl::kson_ser(name.clone(), sd, gens);
    let modname = parse_str::<Ident>(&format!("__{}__kser__", name.to_string()))
        .expect("failed to parse module identifier");

    let imp = quote! {
        mod #modname {
            use super::*;
            use kson::{
                *,
                de::{De, Deserializer},
                ser::{Ser, Serializer},
                errors::KsonError,
                prelude::*
            };

            #kser
        }
    };

    imp.into()
}

fn de_enum_impl(
    name: Ident,
    sd: DataEnum,
    gens: Generics,
) -> TokenStream {
    let kde = de::enum_impl::kson_de(name.clone(), sd, gens);
    let modname = parse_str::<Ident>(&format!("__{}__kde__", name.to_string()))
        .expect("failed to parse module identifier");

    let imp = quote! {

        mod #modname {
            use super::*;
            use kson::{
                *,
                de::{De, Deserializer},
                ser::{Ser, Serializer},
                errors::KsonError,
                prelude::*
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

    // calculate generics
    let mut generics = ast.generics.clone();
    let params: Vec<Ident> = generics.type_params().map(|t| t.ident.clone()).collect();
    let clause = generics.make_where_clause();
    for t in params {
        let pred: WherePredicate = parse_quote! {
            #t: Ser
        };
        clause.predicates.push(pred);
    }

    match ast.data.clone() {
        Data::Struct(sd) => ser_struct_impl(name, sd, generics),
        Data::Enum(ed) => ser_enum_impl(name, ed, generics),
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

    // calculate generics
    let mut generics = ast.generics.clone();
    let params: Vec<Ident> = generics.type_params().map(|t| t.ident.clone()).collect();
    let clause = generics.make_where_clause();
    for t in params {
        let pred: WherePredicate = parse_quote! {
            #t: De
        };
        clause.predicates.push(pred);
    }

    // let (impl_gens, ty_gens, where_clause) = generics.split_for_impl();
    // eprintln!(
    //     "{:#?}",
    //     quote! {
    //         impl #impl_gens De for #name #ty_gens #where_clause {}
    //     }
    //     .to_string()
    // );

    match ast.data.clone() {
        Data::Struct(sd) => de_struct_impl(name, sd, generics),
        Data::Enum(ed) => de_enum_impl(name, ed, generics),
        _ => quote! {}.into(),
    }
}
