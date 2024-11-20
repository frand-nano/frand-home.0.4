use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, Ident, ItemStruct};

mod node;

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let state = parse_macro_input!(item as ItemStruct);

    let macro_name = Ident::new(
        &format!("{}_macro", state.ident.to_string()).to_case(Case::Snake), 
        state.ident.span(),
    );

    let node = node::expand(state)
    .unwrap_or_else(Error::into_compile_error);
    
    quote::quote! { 
        #[allow(unused_macros)]
        macro_rules! #macro_name {
            () => {
                #node
            }
        }
    }.into()
}
