use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, Ident, ItemStruct, Type};

mod node;
mod component;

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let state = parse_macro_input!(item as ItemStruct);

    let node = node::expand(state)
    .unwrap_or_else(Error::into_compile_error);
    
    quote::quote! { 
        #node
    }.into()
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let state_ty = parse_macro_input!(attr as Type);
    let state = parse_macro_input!(item as ItemStruct);

    let component = component::expand(state_ty, state)
    .unwrap_or_else(Error::into_compile_error);
    
    quote::quote! { 
        #component
    }.into()
}

#[proc_macro_attribute]
pub fn node_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let node: proc_macro2::TokenStream = node(attr.clone(), item.clone()).into();
    let state = parse_macro_input!(item as ItemStruct);

    let macro_name = Ident::new(
        &format!("{}_macro", state.ident.to_string()).to_case(Case::Snake), 
        state.ident.span(),
    );
    
    #[cfg(debug_assertions)]
    let result = quote::quote! { 
        #[allow(unused_macros)]
        macro_rules! #macro_name {
            () => { #node }
        }
    }.into();

    #[cfg(not(debug_assertions))]
    let result = quote::quote! { 
        #[allow(unused_macros)]
        macro_rules! #macro_name {
            () => { }
        }
        #node
    }.into();

    result
}

#[proc_macro_attribute]
pub fn component_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let component: proc_macro2::TokenStream = component(attr.clone(), item.clone()).into();
    let state = parse_macro_input!(item as ItemStruct);

    let macro_name = Ident::new(
        &format!("{}_macro", state.ident.to_string()).to_case(Case::Snake), 
        state.ident.span(),
    );
    
    #[cfg(debug_assertions)]
    let result = quote::quote! { 
        #[allow(unused_macros)]
        macro_rules! #macro_name {
            () => { #component }
        }
    }.into();

    #[cfg(not(debug_assertions))]
    let result = quote::quote! { 
        #[allow(unused_macros)]
        macro_rules! #macro_name {
            () => { }
        }
        #component
    }.into();

    result
}