use proc_macro2::Span;
use quote::quote;
use syn::*;

pub fn kson_ser(name: Ident, data: DataEnum) -> proc_macro2::TokenStream {
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

    // ser
    let impl_ser = {
        let branches =
            variant_id_fields
                .iter()
                .map(|(variant, field_idents, fields, ident_string)| {
                    match &fields {
                        // C-style
                        Fields::Named(_fields) => {
                            let seq_len: usize = field_idents.len();

                            let field_strs: Vec<String> = field_idents
                                .iter()
                                .map(std::string::ToString::to_string)
                                .collect();
                            let pairs = field_idents.iter().zip(field_strs.iter()).map(
                                |(ident, ident_string)| {
                                    quote! {
                                        #ident_string, &#ident
                                    }
                                },
                            );

                            quote! {
                                #name::#variant{ #(#field_idents),*} =>  {
                                    s.start_cons(true, #seq_len);

                                    // name
                                    s.put_cons_tag(#ident_string);

                                    // fields
                                    #(s.put_cons_pair(#pairs);)*
                                }
                            }
                        }
                        // Tuple
                        Fields::Unnamed(_fields) => {
                            let seq_len: usize = field_idents.len();

                            let kargs = field_idents
                                .iter()
                                .map(|variant_ident| quote! {&#variant_ident});
                            quote! {
                                #name::#variant(#(#field_idents),*) => {
                                    s.start_cons(false, #seq_len);

                                    // name
                                    s.put_cons_tag(#ident_string);

                                    // fields
                                    #(s.put_cons_item(#kargs);)*
                                }
                            }
                        }
                        // Unit-like
                        Fields::Unit => {
                            quote! {
                                #name::#variant => {
                                    s.start_cons(false, 0);
                                    s.put_cons_tag(#ident_string);
                                }
                            }
                        }
                    }
                });
        quote! {
            fn ser<S: Serializer>(self, s: &mut S) {
                match self {
                    #(#branches)*
                }
            }
        }
    };

    quote! {
        impl Ser for #name {
            #impl_ser
        }
    }
}
