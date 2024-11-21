use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::*;
use quote::quote;
use crate::{attrs::Attrs, component_attrs::{ComponentAttrItem, ComponentAttrKeyItem}};

pub fn expand(
    attrs: &Attrs<ComponentAttrKeyItem>,
    component: &ItemStruct,
) -> Result<TokenStream> {
    let mp = quote!{ frand_node::__macro_prelude };

    let component_name = &component.ident;

    let mod_name = Ident::new(
        &format!("{}Mod", component_name.to_string()).to_case(Case::Pascal), 
        component_name.span(),
    );

    let component_attrs: Option<TokenStream> = attrs.find("attrs", |attr| {
        if let ComponentAttrItem::Attrs(attrs) = &attr.item {
            let attrs = attrs.iter();
            Ok(quote! { #(#attrs)* })
        } else {
            Err(Error::new_spanned(&attr.key, "ComponentAttrItem expand error"))
        }     
    })?;

    let state: Option<TokenStream> = attrs.find("state", |attr| {
        if let ComponentAttrItem::Ident(ident) = &attr.item {
            Ok(quote! { #ident })
        } else {
            Err(Error::new_spanned(&attr.key, "ComponentAttrItem expand error"))
        }     
    })?;
    
    let component = quote!{
        #component_attrs
        pub struct #component_name {   
            #[doc(hidden)]
            __performer: #mp::Performer<#state>,
        }
    };

    let component_impl = quote!{
        impl #mp::ComponentBase for #component_name
        where Self: #mp::Component
        {
            type State = #state;
            type Node = <#state as StateBase>::Node;
            type Message = <#state as StateBase>::Message;
        
            fn node(&self) -> &Self::Node { &self.__performer.node() }
            fn input_tx(&self) -> &#mp::Sender<#mp::MessageData> { self.__performer.input_tx() }    
            fn take_output_rx(&mut self) -> Option<#mp::Receiver<#mp::MessageData>> { self.__performer.take_output_rx() }
            fn perform(&mut self) -> #mp::Result<(usize, usize)> { self.__performer.perform::<Self>() }
        
            fn new() -> Self {
                Self { __performer: #mp::Performer::new() }
            }

            fn replace_node(&mut self, node: &Self::Node) {
                self.__performer.replace_node(node);
            }
        }
    };

    Ok(quote! {
        #component

        #[allow(non_snake_case)]
        pub mod #mod_name {
            #[allow(unused_imports)]
            use super::*;
            
            #component_impl
        }    
    })
}