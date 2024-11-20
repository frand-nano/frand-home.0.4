use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::{Field, Fields, Ident, ItemStruct, Result};
use quote::quote;

pub type MessageDataId = u32;

pub fn expand(
    state: ItemStruct,
) -> Result<TokenStream> {
    let state_name = &state.ident;

    let mod_name = Ident::new(
        &format!("{}_mod", state_name.to_string()).to_case(Case::Snake), 
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
        quote!{ <#ty as StateBase>::Node }
    ).collect();
    let ty_messages: Vec<_> = tys.iter().map(|ty| 
        quote!{ <#ty as StateBase>::Message }
    ).collect();

    let pascal_names: Vec<_> = names.iter()
    .map(|name| {
        let pascal_name = name.to_string().to_case(Case::Pascal);
        Ident::new(&pascal_name, name.span())
    }).collect();

    let state = quote! {
        #[derive(
            Debug, Clone, Default, PartialEq, 
            frand_node::macro_prelude::reexport_serde::Serialize, 
            frand_node::macro_prelude::reexport_serde::Deserialize,
        )]
        #state

        impl StateBase for #state_name {
            type Node = #mod_name::Node;
            type Message = #mod_name::Message;
        }
    };

    let node = quote! {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq)]
        pub struct Node {
            #(pub #names: #ty_nodes,)*
            callback: Callback<#state_name>,
        }

        impl NodeBase<#state_name> for Node {
            fn emit(&self, state: &#state_name) -> Result<()> {
                self.callback.emit(state)
            }

            fn new(
                callback: &Rc<dyn Fn(MessageData)>,   
                mut ids: Vec<MessageDataId>,
                id: Option<MessageDataId>,  
            ) -> Self {
                if let Some(id) = id { ids.push(id); }

                Self { 
                    callback: Callback::new(callback, ids.clone(), Some(#state_id)), 
                    #(#names: #ty_nodes::new(callback, ids.clone(), Some(#indexes)),)*
                }
            }

            #[doc(hidden)]
            fn __apply(&mut self, data: MessageData) -> Result<()> {
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
            #(#pascal_names(#ty_messages),)*
            State(#state_name),
        }

        impl MessageBase for Message {
            fn deserialize_message(data: MessageData) -> Result<Self, ComponentError> {
                match data.next() {
                    #((Some(#indexes), data) => Ok(Message::#pascal_names(#ty_messages::deserialize_message(data)?)),)*
                    (Some(#state_id), data) => Ok(Self::State(data.deserialize()?)),
                    (Some(_), data) => Err(data.error(
                        format!("{}::Message::deserialize_message() unknown id", stringify!(#state_name)),
                    )),
                    (None, data) => Err(data.error(
                        format!("{}::Message::deserialize_message() data has no more id", stringify!(#state_name)),
                    )),
                }     
            }
        }
    };

    
    Ok(quote!{
        #state

        pub mod #mod_name {
            #[allow(unused_imports)]
            use super::*;

            #[allow(unused_imports)]
            use frand_node::macro_prelude::*;
            
            #node
            #message
        }        
    })
}