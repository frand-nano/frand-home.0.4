use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::{Field, Fields, Ident, ItemStruct, Result};
use quote::quote;

pub type MessageDataId = u32;

pub fn expand(
    state: ItemStruct,
) -> Result<TokenStream> {
    let mp = quote!{ frand_node::__macro_prelude };

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

    let state = quote! {
        #[derive(
            Debug, Clone, Default, PartialEq, 
            #mp::reexport_serde::Serialize, 
            #mp::reexport_serde::Deserialize,
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
        #[derive(Debug, Clone, PartialEq)]
        pub struct Node {
            #(pub #names: #ty_nodes,)*
            callback: #mp::Callback<#state_name>,
        }

        impl #mp::NodeBase for Node {
            type State = #state_name;

            fn emit(&self, state: &#state_name) -> #mp::Result<()> {
                self.callback.emit(state)
            }

            fn new(
                callback: &#mp::Sender<#mp::MessageData>,   
                mut ids: Vec<#mp::MessageDataId>,
                id: Option<#mp::MessageDataId>,  
            ) -> Self {
                if let Some(id) = id { ids.push(id); }

                Self { 
                    callback: #mp::Callback::new(callback, ids.clone(), Some(#state_id)), 
                    #(#names: #ty_nodes::new(callback, ids.clone(), Some(#indexes)),)*
                }
            }

            #[doc(hidden)]
            fn __apply(&mut self, data: #mp::MessageData) -> #mp::Result<()> {
                match data.next() {
                    #((Some(#indexes), data) => self.#names.__apply(data),)*
                    (Some(#state_id), data) => Ok(self.__apply_state(data.deserialize()?)),
                    (Some(_), data) => Err(data.error(
                        format!("{}::apply() unknown id", stringify!(#state_name)),
                    )),
                    (None, data) => Err(data.error(
                        format!("{}::apply() data has no more id", stringify!(#state_name)),
                    )),
                }     
            }

            #[doc(hidden)]
            fn __apply_state(&mut self, state: #state_name) {
                #(self.#names.__apply_state(state.#names);)*
            }
        }
    };

    let message = quote! {
        #[derive(Debug, Clone)]
        pub enum Message {
            #(#[allow(non_camel_case_types)] #names(#[allow(dead_code)] #ty_messages),)*
            #[allow(non_camel_case_types)] State(#[allow(dead_code)] #state_name),
        }

        impl #mp::MessageBase for Message {
            type State = #state_name;

            fn deserialize(data: #mp::MessageData) -> #mp::Result<Self> {
                match data.next() {
                    #((Some(#indexes), data) => Ok(Message::#names(#ty_messages::deserialize(data)?)),)*
                    (Some(#state_id), data) => Ok(Self::State(data.deserialize()?)),
                    (Some(_), data) => Err(data.error(
                        format!("{}::Message::deserialize() unknown id", stringify!(#state_name)),
                    )),
                    (None, data) => Err(data.error(
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