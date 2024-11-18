use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Error, Ident, ItemStruct};

mod node;

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut state = parse_macro_input!(item as ItemStruct);
    let mod_name = state.ident.clone();
    state.ident = Ident::new("State", Span::mixed_site());

    let node = node::expand(&mod_name, &state)
    .unwrap_or_else(Error::into_compile_error);

    let macro_name = Ident::new(
        &format!("{}Macro", mod_name.to_string()).to_case(Case::Snake), 
        mod_name.span(),
    );
    
    quote::quote! { 
        #[allow(unused_macros)]
        macro_rules! #macro_name {
            () => {
                #[allow(non_snake_case)]
                pub mod #mod_name {
                    #[allow(unused_imports)]
                    use frand_node::*;

                    #[allow(unused_imports)]
                    use super::*;

                    #node
                }
            }
        }
    }.into()
}