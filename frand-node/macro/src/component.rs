use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::{Type, Ident, ItemStruct, Result};
use quote::quote;

pub fn expand(
    state_ty: Type,
    component: ItemStruct,
) -> Result<TokenStream> {
    let mp = quote!{ frand_node::__macro_prelude };

    let component_name = &component.ident;

    let mod_name = Ident::new(
        &format!("__{}_mod", component_name.to_string()).to_case(Case::Snake), 
        component_name.span(),
    );

    let component = quote!{
        pub struct #component_name {   
            #[doc(hidden)]
            __performer: #mp::Performer<#state_ty>,
        }
    };

    let component_impl = quote!{
        impl #mp::ComponentBase for #component_name
        where Self: #mp::Component
        {
            type State = #state_ty;
            type Node = <#state_ty as StateBase>::Node;
            type Message = <#state_ty as StateBase>::Message;
        
            fn node(&self) -> &Self::Node { &self.__performer.node() }
            fn input_tx(&self) -> &#mp::Sender<#mp::MessageData> { self.__performer.input_tx() }    
            fn take_output_rx(&mut self) -> Option<#mp::Receiver<#mp::MessageData>> { self.__performer.take_output_rx() }
            fn perform(&mut self) -> #mp::Result<()> { self.__performer.perform::<Self>() }
        
            fn new() -> Self {
                Self { __performer: #mp::Performer::new() }
            }
        }
    };

    Ok(quote! {
        #component

        #[doc(hidden)]
        pub mod #mod_name {
            #[allow(unused_imports)]
            use super::*;
            
            #component_impl
        }    
    })
}