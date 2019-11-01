use quote::quote;
use syn::*;

fn read_struct_tag(
    should_be_map: bool,
    exp_len: usize,
    ident_string: &str,
) -> proc_macro2::TokenStream {
    let map_cond = if should_be_map {
        quote!(!is_map)
    } else {
        quote!(is_map)
    };
    quote! {
        |(d,is_map,len)| {
            if #map_cond {
                e!(
                    WrongMinorType {
                        expected: "cons-map",
                        found: "array-map".into()
                    },
                    d.data.clone(),
                    d.ix
                )
            } else if len == #exp_len {
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

pub fn kson_de(name: Ident, data: DataStruct) -> proc_macro2::TokenStream {
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
                        #(#field_names: de::check_entry(#field_strings)?,)*
                    })
                }
            };

            quote! {
                d.read_cons(#read_tag, #read_items)
            }
        }
        // Tuple structs
        Fields::Unnamed(fields) => {
            let fields: Vec<Type> = Fields::Unnamed(fields)
                .iter()
                .map(|field| field.ty.clone())
                .collect();
            let fields_len: usize = fields.len();

            // de
            let ident_string = name.to_string();

            let exp_len = fields_len;

            let read_tag = read_struct_tag(false, exp_len, &ident_string);

            let read_items = quote! {
                |d,()| Ok(#name(#(#fields::de(d)?,)*))
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

    quote! {
        impl De for #name {
            fn de<D: Deserializer>(d: &mut D) -> Result<Self, Error> {
                #impl_de
            }
        }
    }
}
