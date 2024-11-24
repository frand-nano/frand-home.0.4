use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::*;
use quote::quote;
use crate::{attrs::Attrs, node_attrs::{NodeAttrItem, NodeAttrKeyItem}};

pub type PayloadId = u32;

pub fn expand(
    attrs: &Attrs<NodeAttrKeyItem>,
    state: &ItemStruct,
) -> Result<TokenStream> {
    let mp = quote!{ frand_node::macro_prelude };

    let node_name = state.ident.clone();

    let state_name = Ident::new(
        &format!("{}State", node_name.to_string()).to_case(Case::Pascal), 
        node_name.span(),
    );

    let message_name = Ident::new(
        &format!("{}Message", node_name.to_string()).to_case(Case::Pascal), 
        node_name.span(),
    );

    let mod_name = Ident::new(
        &format!("{}Mod", node_name.to_string()).to_case(Case::Pascal), 
        node_name.span(),
    );

    let fields: Vec<&Field> = match &state.fields {
        Fields::Named(fields_named) => fields_named.named.iter().collect(),
        _ => Vec::default(),
    };  

    let indexes: Vec<_> = (0..fields.len() as PayloadId).into_iter().collect();
    let names: Vec<_> = fields.iter().filter_map(|field| field.ident.as_ref()).collect();
    let tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    let state_tys: Vec<_> = tys.iter().map(|ty| 
        quote!{ <#ty as #mp::ElementBase>::State }
    ).collect();

    let node_tys: Vec<_> = tys.iter().map(|ty| 
        quote!{ <#ty as #mp::ElementBase>::Node }
    ).collect();

    let message_tys: Vec<_> = tys.iter().map(|ty| 
        quote!{ <#ty as #mp::ElementBase>::Message }
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

    let node = quote! {
        #[allow(dead_code)]
        #[derive(Debug, Clone)]
        #node_attrs
        pub struct #node_name {
            depth: usize,
            key: #mp::PayloadKey,
            callback: #mp::RefCell<#mp::CallbackSender>,    
            #(pub #names: #node_tys,)*
        }
    };

    let state_forward = quote! {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        pub type #state_name = #mod_name::State;
    };

    let message_forward = quote! {
        #[allow(non_snake_case)]
        pub mod #message_name {
            #(#[allow(unused_imports)] pub use super::#mod_name::Message::#names;)*
        }
    };

    let node_impl = quote! {
        impl Default for #node_name {
            fn default() -> Self {
                Self::new(&#mp::CallbackSender::None, vec![], None)
            }
        }

        impl PartialEq for #node_name {
            fn eq(&self, other: &Self) -> bool {
                true 
                #(&& self.#names == other.#names)*
            }
        }

        impl #mp::ElementBase for #node_name {
            type State = State;
            type Node = Self;
            type Message = Message;
        }
        
        impl #mp::NodeBase for #node_name {
            fn new(
                callback: &#mp::CallbackSender,   
                mut key: Vec<#mp::PayloadId>,
                id: Option<#mp::PayloadId>,  
            ) -> Self {
                if let Some(id) = id { key.push(id); }

                Self { 
                    depth: key.len(),
                    key: key.clone().into_boxed_slice(),
                    callback: #mp::RefCell::new(callback.clone()),
                    #(#names: #node_tys::new(callback, key.clone(), Some(#indexes)),)*
                }
            }
        }

        impl #mp::Emitter<State> for #node_name {
            fn depth(&self) -> usize { self.depth }
            fn callback(&self) -> #mp::Ref<#mp::CallbackSender> { self.callback.borrow() }
        
            fn set_callback(&self, callback: &#mp::CallbackSender) { 
                *self.callback.borrow_mut() = callback.clone();     
                #(self.#names.set_callback(callback);)*   
            }
        
            fn emit(&self, state: State) {
                self.callback.borrow().send(
                    #mp::Payload::new(&self.key, None, state)
                )
                .unwrap_or_else(|err| match err {
                    #mp::NodeError::Send(err) => {
                        log::debug!("close callback. reason: {err}");
                        *self.callback.borrow_mut() = #mp::CallbackSender::None;
                    },
                    _ => panic!("{err}"),
                })
            }
        }

        impl Stater<State> for #node_name {    
            fn apply(&mut self, payload: &#mp::Payload) {
                let depth = self.depth();
                match payload.get_id(depth) {
                    #(Some(#indexes) => Ok(self.#names.apply(payload)),)*
                    Some(_) => Err(payload.error(depth, "unknown id")),
                    None => Ok(self.apply_state(payload.read_state())),
                }     
                .unwrap_or_else(|err| panic!("{}::apply() deserialize Err({err})", stringify!(#node_name)));
            }
        
            fn apply_state(&mut self, state: State) {
                #(self.#names.apply_state(state.#names);)*
            }
        }
    };

    let state = quote! {
        #state_attrs
        #[derive(
            Debug, Clone, Default, PartialEq, 
            #mp::Serialize, 
            #mp::Deserialize,
        )]
        pub struct State {
            #(pub #names: #state_tys,)*
        }

        impl #mp::ElementBase for State {
            type State = Self;
            type Node = super::#node_name;
            type Message = Message;
        }

        impl #mp::StateBase for State {
            
        }   
    };

    let message = quote! {
        #message_attrs
        #[derive(Debug, Clone)]
        pub enum Message {
            #(#[allow(non_camel_case_types)] #names(#[allow(dead_code)] #message_tys),)*
            #[allow(non_camel_case_types)] State(#[allow(dead_code)] State),
        }

        impl #mp::MessageBase for Message {
            fn from_payload(depth: usize, payload: #mp::Payload) -> Self {
                match payload.get_id(depth) {
                    #(Some(#indexes) => Ok(Message::#names(#message_tys::from_payload(depth + 1, payload))),)*
                    Some(_) => Err(payload.error(depth, "unknown id")),
                    None => Ok(Self::State(payload.read_state())),
                }     
                .unwrap_or_else(|err| panic!("{}::from_payload() Err({err})", stringify!(#node_name)))
            }
        }
    };
    
    Ok(quote!{
        #node

        #state_forward
        #message_forward
        
        #[allow(non_snake_case)]
        pub mod #mod_name {
            #[allow(unused_imports)]
            use super::*;

            #node_impl
            #state
            #message
        }        
    })
}