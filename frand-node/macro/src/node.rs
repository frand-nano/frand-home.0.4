use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::*;
use quote::quote;

pub type NodeId = u32;

pub fn expand(
    state: &ItemStruct,
) -> Result<TokenStream> {
    let mp = quote!{ frand_node::macro_prelude };

    let node_attrs = &state.attrs;
    let node_name = state.ident.clone();

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

    let indexes: Vec<_> = (0..fields.len() as NodeId).into_iter().collect();
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

    let node = quote! {
        #[allow(dead_code)]
        #(#node_attrs)*
        #[derive(Debug, PartialEq)]
        pub struct #node_name {
            emitter: #mp::Emitter,     
            #(pub #names: #node_tys,)*
        }
    };

    let message_forward = quote! {
        #[allow(non_snake_case)]
        pub mod #message_name {
            pub use super::#mod_name::Message;
            #(#[allow(unused_imports)] pub use Message::#names;)*
        }
    };

    let node_impl = quote! {
        impl Clone for #node_name {
            fn clone(&self) -> Self {
                log::debug!("{}::clone", stringify!(#node_name));
                Self { 
                    emitter: self.emitter.clone(),  
                    #(#names: self.#names.clone(),)*
                }
            }
        }

        impl Default for #node_name {
            fn default() -> Self { Self::new(vec![], None, #mp::Reporter::None) }
        }

        impl #mp::ElementBase for #node_name {
            type State = State;
            type Node = Self;
            type Message = Message;
        }
        
        impl #mp::NodeBase for #node_name {
            fn new(
                mut key: Vec<#mp::NodeId>,
                id: Option<#mp::NodeId>,  
                reporter: #mp::Reporter,
            ) -> Self {
                if let Some(id) = id { key.push(id); }

                Self { 
                    #(#names: #node_tys::new(key.clone(), Some(#indexes), reporter.clone()),)*
                    emitter: #mp::Emitter::new(key, reporter),
                }
            }       

            fn emit(&self, state: Self::State) {
                self.emitter.emit(state);
            }

            fn emit_packet(&self, packet: #mp::Packet) {
                self.emitter.emit_packet(packet);
            }    

            fn emit_future<Fu>(&self, future: Fu) 
            where Fu: 'static + #mp::Future<Output = Self::State> {
                self.emitter.emit_future(future);
            }
        }

        impl #mp::Stater<State> for #node_name {    
            fn apply(&mut self, state: State) {
                #(self.#names.apply(state.#names);)*
            }

            fn apply_packet(&mut self, packet: &#mp::Packet) {
                let depth = self.emitter.depth();
                match packet.get_id(depth) {
                    #(Some(#indexes) => Ok(self.#names.apply_packet(packet)),)*
                    Some(_) => Err(packet.error(depth, "unknown id")),
                    None => Ok(self.apply(packet.read_state())),
                }     
                .unwrap_or_else(|err| panic!("{}::apply_packet() deserialize Err({err})", stringify!(#node_name)));
            }        
        }
    };

    let state = quote! {
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
        #[derive(Debug, Clone)]
        pub enum Message {
            #(#[allow(non_camel_case_types)] #names(#[allow(dead_code)] #message_tys),)*
            #[allow(non_camel_case_types)] State(#[allow(dead_code)] State),
        }

        impl #mp::MessageBase for Message {
            fn from_packet(depth: usize, packet: &#mp::Packet) -> Self {
                match packet.get_id(depth) {
                    #(Some(#indexes) => Ok(Message::#names(#message_tys::from_packet(depth + 1, packet))),)*
                    Some(_) => Err(packet.error(depth, "unknown id")),
                    None => Ok(Self::State(packet.read_state())),
                }     
                .unwrap_or_else(|err| panic!("{}::from_packet() Err({err})", stringify!(#node_name)))
            }
        }
    };
    
    Ok(quote!{
        #node

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