use heck::{ToUpperCamelCase, ToSnakeCase};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Meta};

#[proc_macro_derive(DiffFields, attributes(skip_diff))]
pub fn derive_diffs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            let owner = &input.ident;
            let name = format_ident!("{}Diff", input.ident);
            let map_ident =
                |ident: &Ident| format_ident!("{}", ident.to_string().to_upper_camel_case());
            let fields: Vec<_> = fields
                .named
                .iter()
                .flat_map(|field| {
                    if field
                        .attrs
                        .iter()
                        .find(|attr| {
                            if let Meta::Path(ref p) = attr.meta {
                                p.is_ident("skip_diff")
                            } else {
                                true
                            }
                        })
                        .is_some()
                    {
                        None
                    } else {
                        Some((field.ident.as_ref().unwrap(), &field.ty))
                    }
                })
                .collect();
            let variants = fields.iter().map(|(ident, ty)| {
                let name = map_ident(ident);
                quote!(#name(#ty))
            });
            let match_arm = fields.iter().map(|(ident, _)| {
                let variant = map_ident(ident);
                quote!(#name::#variant(v) => { obj.#ident = v; })
            });
            let apply_fn = format_ident!("apply_{}", name.to_string().to_snake_case());
            return TokenStream::from(quote!(
                #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
                pub enum #name {
                    #(#variants),*
                }
                fn #apply_fn(obj: &mut #owner, diff: #name) {
                    match diff {
                        #(#match_arm),*
                    }
                }
            ));
        }
    }
    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `FromRow`",
        )
        .to_compile_error(),
    )
}
