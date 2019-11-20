use proc_macro2::Span;
use quote::quote;
use syn::*;

pub fn kson_de(
    name: Ident,
    data: DataEnum,
    gens: Generics,
) -> proc_macro2::TokenStream {
    let variant_id_fields: Vec<(Ident, Vec<Ident>, Fields, String)> = data
        .variants // variants of the enum
        .iter()
        .map(|variant| {
            // fields of the variant
            let field_idents = match &variant.fields {
                Fields::Named(_fields) => variant
                    .fields
                    .iter()
                    .map(|field| field.ident.clone().unwrap())
                    .collect(),
                _ => (0..variant.fields.iter().len())
                    .map(|i| Ident::new(&format!("field{}", i), Span::call_site()))
                    .collect(),
            };
            (
                variant.ident.clone(),
                field_idents,
                variant.fields.clone(),
                variant.ident.to_string(),
            )
        })
        .collect();

    // de
    let impl_de = {
        let fields_struct = quote! {
            struct __fields_data {
                id: String,
                is_map: bool,
                num_fields: usize,
            }
        };

        let read_cons_tag = quote! {
            |d,is_map,num_fields| {
                let id = d.read_string()?;
                Ok(__fields_data {
                    id,
                    is_map,
                    num_fields,
                })
            }
        };

        let pairs = variant_id_fields.into_iter().map(
            |(m_variant, m_field_idents, m_fields, m_ident_string)| {
                let constructor = quote! { #name::#m_variant };

                match &m_fields {
                    // Unit-like variant
                    Fields::Unit => {
                        quote! {
                            #m_ident_string => {
                                if is_map {
                                    Err(E!(
                                        WrongMinorType {
                                            expected: "cons-array",
                                            found: "cons-map".into()
                                        },
                                        d.data.clone(),
                                        d.ix
                                    ))
                                } else if num_fields != 0 {
                                    Err(E!(
                                        WrongConsSize {
                                            expected: 0,
                                            found: num_fields
                                        },
                                        d.data.clone(),
                                        d.ix
                                    ))
                                } else {
                                    Ok(#constructor)
                                }
                            }
                        }
                    }
                    // Named-tuple variant
                    Fields::Unnamed(_fields) => {
                        let exp_len = m_field_idents.len();
                        let params = (0..exp_len).map(|_| quote!(d.take_val()?));

                        quote! {
                            #m_ident_string => {
                                if is_map {
                                    Err(E!(
                                        WrongMinorType {
                                            expected: "cons-array",
                                            found: "cons-map".into()
                                        },
                                        d.data.clone(),
                                        d.ix
                                    ))
                                } else if num_fields != #exp_len {
                                    Err(E!(WrongConsSize {
                                        expected: #exp_len,
                                        found: num_fields
                                    },
                                    d.data.clone(),
                                    d.ix
                                    ))
                                } else {
                                    Ok(#constructor(#(#params,)*))
                                }
                            }
                        }
                    }
                    // C-style struct variant
                    Fields::Named(_fields) => {
                        let mut field_stuff: Vec<(Ident, String)> = m_fields
                            .iter()
                            .map(|field| field.ident.clone().unwrap())
                            .map(|field| (field.clone(), field.to_string()))
                            .collect();

                        field_stuff.sort_unstable_by(|(_, k1), (_, k2)| k1.cmp(k2));

                        let (field_names, field_strings): (Vec<Ident>, Vec<String>) =
                            field_stuff.into_iter().unzip();

                        let exp_len = field_names.len();

                        quote! {
                            #m_ident_string => {
                                if !is_map {
                                    Err(E!(
                                        WrongMinorType {
                                            expected: "cons-map",
                                            found: "cons-array".into()
                                        },
                                        d.data.clone(),
                                        d.ix
                                    ))
                                } else if num_fields != #exp_len {
                                    Err(E!(WrongConsSize {
                                        expected: #exp_len,
                                        found: num_fields
                                    },
                                    d.data.clone(),
                                    d.ix
                                    ))
                                } else {
                                    Ok(#constructor {
                                        #(#field_names: d.check_entry(#field_strings)?,)*
                                    })
                                }
                            }
                        }
                    }
                }
            },
        );

        quote! {
            #fields_struct

            d.read_cons(#read_cons_tag, |d, data| {
                let __fields_data { id, is_map, num_fields } = data;
                match id.as_str() {
                    #(#pairs)*
                    _ =>
                        Err(E!(
                            WrongEnumVariant {
                                found: id
                            },
                            d.data.clone(),
                            d.ix
                        ))
                }
            })
        }
    };

    let (impl_generics, ty_generics, where_clause) = gens.split_for_impl();
    quote! {
        impl #impl_generics De for #name #ty_generics #where_clause {
            fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
                #impl_de
            }
        }
    }
}
