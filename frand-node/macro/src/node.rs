use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::*;
use quote::quote;
use crate::{attrs::Attrs, node_attrs::{NodeAttrItem, NodeAttrKeyItem}};

pub type MessageDataId = u32;

pub fn expand(
    attrs: &Attrs<NodeAttrKeyItem>,
    state: &ItemStruct,
) -> Result<TokenStream> {
    let mp = quote!{ frand_node::macro_prelude };

    let state_name = &state.ident;

    let message_name = Ident::new(
        &format!("{}Message", state_name.to_string()).to_case(Case::Pascal), 
        state_name.span(),
    );

    let mod_name = Ident::new(
        &format!("{}Mod", state_name.to_string()).to_case(Case::Pascal), 
        state_name.span(),
    );

    let fields: Vec<&Field> = match &state.fields {
        Fields::Named(fields_named) => fields_named.named.iter().collect(),
        _ => Vec::default(),
    };  

    let state_id = fields.len() as MessageDataId;

    let indexes: Vec<_> = (0..fields.len() as MessageDataId).into_iter().collect();
    let names: Vec<_> = fields.iter().filter_map(|field| field.ident.as_ref()).collect();
    let tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    let ty_nodes: Vec<_> = tys.iter().map(|ty| 
        quote!{ <#ty as #mp::StateBase>::Node }
    ).collect();

    let ty_messages: Vec<_> = tys.iter().map(|ty| 
        quote!{ <#ty as #mp::StateBase>::Message }
    ).collect();

    let state_attrs: Option<TokenStream> = attrs.find("state_attrs", |attr| {
        if let NodeAttrItem::Attrs(attrs) = &attr.item {
            let attrs = attrs.iter();
            Ok(quote! { #(#attrs)* })
        } else {
            Err(Error::new_spanned(&attr.key, "NodeAttrItem expand error"))
        }     
    })?;

    let node_attrs: Option<TokenStream> = attrs.find("node_attrs", |attr| {
        if let NodeAttrItem::Attrs(attrs) = &attr.item {
            let attrs = attrs.iter();
            Ok(quote! { #(#attrs)* })
        } else {
            Err(Error::new_spanned(&attr.key, "NodeAttrItem expand error"))
        }     
    })?;

    let message_attrs: Option<TokenStream> = attrs.find("message_attrs", |attr| {
        if let NodeAttrItem::Attrs(attrs) = &attr.item {
            let attrs = attrs.iter();
            Ok(quote! { #(#attrs)* })
        } else {
            Err(Error::new_spanned(&attr.key, "NodeAttrItem expand error"))
        }     
    })?;

    let state = quote! {
        #state_attrs
        #[derive(
            Debug, Clone, Default, PartialEq, 
            #mp::Serialize, 
            #mp::Deserialize,
        )]
        #state
    };

    let message_forward = quote! {
        #[allow(non_snake_case)]
        pub mod #message_name {
            #(#[allow(unused_imports)] pub use super::#mod_name::Message::#names;)*
        }
    };

    let state_impl = quote! {
        impl #mp::StateBase for #state_name {
            type Node = #mod_name::Node;
            type Message = #mod_name::Message;
        }
    };

    let node = quote! {
        #[allow(dead_code)]
        #node_attrs
        #[derive(Debug, Clone, PartialEq)]
        pub struct Node {
            #(pub #names: #ty_nodes,)*
            callback: #mp::Callback<#state_name>,
        }

        impl Default for Node {
            fn default() -> Self {
                Self::new(&#mp::CallbackSender::None, vec![], None)
            }
        }

        impl #mp::NodeBase for Node {
            type State = #state_name;

            fn emit(&self, state: &#state_name) -> #mp::Result<()> {
                self.callback.emit(state)
            }

            fn new(
                sender: &#mp::CallbackSender,   
                mut key: Vec<#mp::MessageDataId>,
                id: Option<#mp::MessageDataId>,  
            ) -> Self {
                if let Some(id) = id { key.push(id); }

                Self { 
                    callback: #mp::Callback::new(sender, key.clone(), Some(#state_id)), 
                    #(#names: #ty_nodes::new(sender, key.clone(), Some(#indexes)),)*
                }
            }

            fn reset_sender(&self, sender: &#mp::CallbackSender) {
                self.callback.reset_sender(sender);
                #(self.#names.reset_sender(sender);)*
            }

            fn apply(&mut self, data: &#mp::MessageData) -> #mp::Result<()> {                
                let depth = self.callback.depth()-1;
                match data.get_id(depth) {
                    #(Some(#indexes) => self.#names.apply(data),)*
                    Some(#state_id) => Ok(self.apply_state(data.read_state()?)),
                    Some(_) => Err(data.error(depth, 
                        format!("{}::apply() unknown id", stringify!(#state_name)),
                    )),
                    None => Err(data.error(depth, 
                        format!("{}::apply() data has no more id", stringify!(#state_name)),
                    )),
                }     
            }

            fn apply_state(&mut self, state: #state_name) {
                #(self.#names.apply_state(state.#names);)*
            }
        }
    };

    let message = quote! {
        #message_attrs
        #[derive(Debug, Clone)]
        pub enum Message {
            #(#[allow(non_camel_case_types)] #names(#[allow(dead_code)] #ty_messages),)*
            #[allow(non_camel_case_types)] State(#[allow(dead_code)] #state_name),
        }

        impl #mp::MessageBase for Message {
            type State = #state_name;

            fn deserialize(depth: usize, data: #mp::MessageData) -> #mp::Result<Self> {
                match data.get_id(depth) {
                    #(Some(#indexes) => Ok(Message::#names(#ty_messages::deserialize(depth + 1, data)?)),)*
                    Some(#state_id) => Ok(Self::State(data.read_state()?)),
                    Some(_) => Err(data.error(depth, 
                        format!("{}::Message::deserialize() unknown id", stringify!(#state_name)),
                    )),
                    None => Err(data.error(depth, 
                        format!("{}::Message::deserialize() data has no more id", stringify!(#state_name)),
                    )),
                }     
            }
        }
    };

    
    Ok(quote!{
        #state

        #message_forward
        
        #[allow(non_snake_case)]
        pub mod #mod_name {
            #[allow(unused_imports)]
            use super::*;

            #state_impl
            #node
            #message
        }        
    })
}