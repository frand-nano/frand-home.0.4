use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::{Field, Fields, Ident, ItemStruct, Result};
use quote::quote;

pub fn expand(
    _mod_name: &Ident,
    state: &ItemStruct,
) -> Result<TokenStream> {
    let fields: Vec<&Field> = match &state.fields {
        Fields::Named(fields_named) => fields_named.named.iter().collect(),
        _ => Vec::default(),
    };  

    let indexes: Vec<_> = (0..fields.len()).into_iter().collect();
    let names: Vec<_> = fields.iter().filter_map(|field| field.ident.as_ref()).collect();
    let tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    let pascal_names: Vec<_> = names.iter()
    .map(|name| {
        let pascal_name = name.to_string().to_case(Case::Pascal);
        Ident::new(&pascal_name, name.span())
    }).collect();

    let state = quote! {
        #[derive(Debug, Clone, Default)]
        #state
    };

    let node = quote! {
        #[allow(dead_code)]
        #[derive(Debug, Clone)]
        pub struct Node {

        }
    };

    let message = quote! {
        #[derive(Debug, Clone)]
        pub enum Message {
            State(State),
        }
    };

    Ok(quote!{        
        #state
        #node
        #message
    })
}