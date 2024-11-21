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
            performer: #mp::Performer<Self>,
        }
    };

    let component_impl = quote!{
        impl #mp::Deref for TestComponent {
            type Target = #mp::Performer<Self>;
            fn deref(&self) -> &Self::Target { &self.performer }
        }

        impl #mp::DerefMut for TestComponent {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.performer }
        }

        impl #mp::ComponentBase for #component_name
        {
            type State = #state;
            type Node = <#state as StateBase>::Node;
            type Message = <#state as StateBase>::Message;
        
            fn node(&self) -> &Self::Node { &self.performer.node() }
            
            fn new() -> Self {
                Self { performer: #mp::Performer::new() }
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