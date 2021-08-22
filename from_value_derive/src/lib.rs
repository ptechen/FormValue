extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(From)]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let mut insert_tokens = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    match parsed_input.data {
        Data::Struct(s) => {
            if let Fields::Named(name_fields) = s.fields {
                let a = name_fields.named;
                for field in a {
                    let field = field.ident.as_ref().unwrap();
                    let insert_token = quote! {
                        map.insert(stringify!(#field).to_string(), Value::from(params.#field.to_owned()));
                    };
                    insert_tokens.push(insert_token);
                }
            }
        }
        other => { panic!("Form is not yet implemented for: {:?} ", other) }
    }
    let tokens = quote! {
        impl From<#struct_name> for Value {
            fn from(params: #struct_name) -> Self {
                let mut map = Map::new();
                #(#insert_tokens)*
                Value::Object(map)
            }
        }
    };
    proc_macro::TokenStream::from(tokens)
}
