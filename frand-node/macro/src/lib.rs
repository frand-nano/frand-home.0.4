use attrs::Attrs;
use component_attrs::ComponentAttrKeyItem;
use convert_case::{Case, Casing};
use node_attrs::NodeAttrKeyItem;
use proc_macro::TokenStream;
use syn::*;

mod node;
mod node_attrs;
mod component;
mod component_attrs;
mod attrs;

#[proc_macro_attribute]
pub fn node(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as Attrs<NodeAttrKeyItem>);
    let state = parse_macro_input!(item as ItemStruct);

    let node = node::expand(&attrs, &state)
    .unwrap_or_else(Error::into_compile_error);
    
    quote::quote! { 
        #node
    }.into()
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as Attrs<ComponentAttrKeyItem>);
    let state = parse_macro_input!(item as ItemStruct);

    let component = component::expand(&attrs, &state)
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