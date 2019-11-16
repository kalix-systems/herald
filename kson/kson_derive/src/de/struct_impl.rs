use quote::quote;
use syn::*;

fn read_struct_tag(
    should_be_map: bool,
    exp_len: usize,
    ident_string: &str,
) -> proc_macro2::TokenStream {
    let (map_cond, minor_expected, minor_found) = if should_be_map {
        (quote!(!is_map), quote!("cons-map"), quote!("cons-array"))
    } else {
        (quote!(is_map), quote!("cons-array"), quote!("cons-map"))
    };
    quote! {
        |d,is_map,len| {
            if #map_cond {
                e!(
                    WrongMinorType {
                        expected: #minor_expected,
                        found: #minor_found.into()
                    },
                    d.data.clone(),
                    d.ix
                )
            } else if len != #exp_len {
                e!(
                    WrongConsSize {
                        expected: #exp_len,
                        found: len
                    },
                    d.data.clone(),
                    d.ix
                )
            }

            let err_data = d.data.clone();
            let err_ix = d.ix;
            let ident_str = d.read_str()?;

            if ident_str != #ident_string {
                e!(
                    WrongMinorType {
                        expected: #ident_string,
                        found: ident_str.into()
                    },
                    err_data,
                    err_ix
                )
            }

            Ok(())
        }
    }
}

pub fn kson_de(name: Ident, data: DataStruct, gens: Generics) -> proc_macro2::TokenStream {
    let impl_de = match data.fields {
        // C-style structs
        Fields::Named(fields) => {
            let mut field_stuff: Vec<(Ident, String)> = Fields::Named(fields)
                .iter()
                .map(|field| field.ident.clone().unwrap())
                .map(|field| (field.clone(), field.to_string()))
                .collect();

            field_stuff.sort_unstable_by(|(_, k1), (_, k2)| k1.cmp(k2));

            let (field_names, field_strings): (Vec<Ident>, Vec<String>) =
                field_stuff.into_iter().unzip();

            let exp_len = field_names.len();

            // de
            let ident_string = name.to_string();

            let read_tag = read_struct_tag(true, exp_len, &ident_string);

            let read_items = quote! {
                |d, ()| {
                    Ok(#name {
                        #(#field_names: d.check_entry(#field_strings)?,)*
                    })
                }
            };

            quote! {
                d.read_cons(#read_tag, #read_items)
            }
        }
        // Tuple structs
        Fields::Unnamed(fields) => {
            let fields_len = fields.unnamed.len();

            // de
            let ident_string = name.to_string();

            let exp_len = fields_len;

            let read_tag = read_struct_tag(false, exp_len, &ident_string);

            let params: Vec<_> = (0..fields_len).map(|_| quote! {d.take_val()?}).collect();

            let read_items = quote! {
                |d,()| Ok(#name(#(#params,)*))
            };

            quote! {
                d.read_cons(#read_tag, #read_items)
            }
        }
        // Unit-like structs
        Fields::Unit => {
            let read_tag = read_struct_tag(false, 0, &name.to_string());

            // de
            quote! {
                d.read_cons(#read_tag, |d,()| Ok(Self))
            }
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
