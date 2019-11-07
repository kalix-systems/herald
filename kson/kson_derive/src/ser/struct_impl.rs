use proc_macro2::Literal;
use quote::quote;
use syn::*;

pub fn kson_ser(name: Ident, data: DataStruct, gens: Generics) -> proc_macro2::TokenStream {
    let impl_ser = match data.fields {
        // C-style structs
        Fields::Named(fields) => {
            let mut field_stuff: Vec<(Ident, String)> = Fields::Named(fields)
                .iter()
                .map(|field| field.ident.clone().unwrap())
                .map(|field| (field.clone(), field.to_string()))
                .collect();

            field_stuff.sort_unstable_by(|(_, k1), (_, k2)| k1.cmp(k2));

            let (field_idents, field_strs): (Vec<Ident>, Vec<String>) =
                field_stuff.into_iter().unzip();

            let ident_string = name.to_string();

            let length = field_idents.len();

            let pairs = field_idents
                .iter()
                .zip(field_strs.iter())
                .map(|(ident, ident_string)| quote! { #ident_string, &self.#ident });

            quote! {
                s.start_cons(true, #length);

                s.put_cons_tag(#ident_string);

                #(s.put_cons_pair(#pairs);)*
            }
        }
        // Tuple structs
        Fields::Unnamed(fields) => {
            let fields: Vec<Type> = Fields::Unnamed(fields)
                .iter()
                .map(|field| field.ty.clone())
                .collect();

            let seq_len: usize = fields.len();

            let ident_string = name.to_string();

            let kargs = (0..fields.len())
                .map(Literal::usize_unsuffixed)
                .map(|index| quote! {self.#index});

            quote! {
                s.start_cons(false, #seq_len);

                // name
                s.put_cons_tag(#ident_string);

                // fields
                #(s.put_cons_item(&#kargs);)*
            }
        }
        // Unit-like structs
        Fields::Unit => {
            let ident_string = name.to_string();

            quote! {
                s.start_cons(false, 0);
                s.put_cons_tag(#ident_string);
            }
        }
    };

    let (impl_generics, ty_generics, where_clause) = gens.split_for_impl();
    quote! {
        impl #impl_generics Ser for #name #ty_generics #where_clause {
            fn ser(&self, s: &mut Serializer) {
                #impl_ser
            }
        }
    }
}
