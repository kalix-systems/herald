#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

mod de;
mod ser;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

fn struct_impl(name: Ident, sd: DataStruct) -> TokenStream {
    let kser = ser::struct_impl::kson_ser(name.clone(), sd.clone());
    let kde = de::struct_impl::kson_de(name.clone(), sd);
    let modname = format!("__{}__kserde__", name.to_string());

    let imp = quote! {
        mod #modname {
            use super::#name;
            use ::kson::{e,E,de::De,ser::Ser,errors::KsonError};

            #kser
            #kde
        }
    };

    imp.into()
}

fn enum_impl(name: Ident, sd: DataEnum) -> TokenStream {
    let kser = ser::enum_impl::kson_ser(name.clone(), sd.clone());
    let kde = de::enum_impl::kson_de(name.clone(), sd);
    let modname = format!("__{}__kserde__", name.to_string());

    let imp = quote! {
        use super::#name;
        use ::kson::{e,E,de::De,ser::Ser,errors::KsonError};

        mod #modname {
            #kser
            #kde
        }
    };

    imp.into()
}

#[proc_macro_derive(KSerDe)]
pub fn kserde_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_kserde_macro(&ast)
}

fn impl_kserde_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = ast.ident.clone();

    match ast.data.clone() {
        Data::Struct(sd) => struct_impl(name, sd),
        Data::Enum(ed) => enum_impl(name, ed),
        _ => quote! {}.into(),
    }
}
