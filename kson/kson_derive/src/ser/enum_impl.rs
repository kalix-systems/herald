use proc_macro2::Span;
use quote::quote;
use syn::*;

pub fn kson_ser(
    name: Ident,
    data: DataEnum,
    gens: Generics,
) -> proc_macro2::TokenStream {
    let variant_id_fields: Vec<(Ident, Vec<Ident>, Fields, String)> = data
        .variants // variants of the enum
        .into_iter()
        .map(|variant| {
            // fields of the variant
            let field_idents: Vec<Ident> = match &variant.fields {
                f @ Fields::Named(_) => {
                    let mut field_stuff: Vec<(Ident, String)> = f
                        .iter()
                        .map(|f| f.ident.clone().unwrap())
                        .map(|f| (f.clone(), f.to_string()))
                        .collect();

                    field_stuff.sort_unstable_by(|(_, k1), (_, k2)| k1.cmp(k2));

                    field_stuff.into_iter().map(|(ident, _)| ident).collect()
                }
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
    let branches = variant_id_fields
        .iter()
        .map(|(variant, field_idents, fields, ident_string)| {
            match &fields {
                // C-style
                Fields::Named(_) => {
                    let seq_len: usize = field_idents.len();

                    let field_strs: Vec<String> = field_idents
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect();

                    quote! {
                        #name::#variant{ #(#field_idents),*} =>  {
                            s.start_cons(true, #seq_len);

                            // name
                            s.put_cons_tag(#ident_string);

                            // fields
                            #(s.put_cons_pair(#field_strs, #field_idents);)*
                        }
                    }
                }
                // Tuple
                Fields::Unnamed(_) => {
                    let seq_len: usize = field_idents.len();

                    let kargs = field_idents.iter();

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

    let (impl_generics, ty_generics, where_clause) = gens.split_for_impl();

    quote! {
        impl #impl_generics Ser for #name #ty_generics #where_clause {
            fn ser(&self, s: &mut Serializer) {
                match self {
                    #(#branches)*
                }
            }
        }
    }
}
