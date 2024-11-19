use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::{Field, Fields, Ident, ItemStruct, Result};
use quote::quote;

pub fn expand(
    mod_name: &Ident,
    state: &ItemStruct,
) -> Result<TokenStream> {
    let fields: Vec<&Field> = match &state.fields {
        Fields::Named(fields_named) => fields_named.named.iter().collect(),
        _ => Vec::default(),
    };  

    let state_id = fields.len();

    let indexes: Vec<_> = (0..fields.len()).into_iter().collect();
    let names: Vec<_> = fields.iter().filter_map(|field| field.ident.as_ref()).collect();
    let tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();
    let ty_nodes: Vec<_> = tys.iter().map(|ty| 
        quote!{ <#ty as StateBase>::Node }
    ).collect();
    let ty_messages: Vec<_> = tys.iter().map(|ty| 
        quote!{ <<#ty as StateBase>::Node as NodeBase<#ty>>::Message }
    ).collect();

    let pascal_names: Vec<_> = names.iter()
    .map(|name| {
        let pascal_name = name.to_string().to_case(Case::Pascal);
        Ident::new(&pascal_name, name.span())
    }).collect();

    let state = quote! {
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
        #state

        impl StateBase for State {
            type Node = Node;
        }
    };

    let node = quote! {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq)]
        pub struct Node {
            #(pub #names: #ty_nodes,)*
            callback: Callback<State>,
        }

        impl NodeBase<State> for Node {
            type Message = Message;

            fn new(
                context: &Context,   
                mut ids: Vec<usize>,
                id: Option<usize>,  
            ) -> Self {
                if let Some(id) = id { ids.push(id); }

                Self { 
                    callback: Callback::new(context, ids.clone(), Some(#state_id)), 
                    #(#names: #ty_nodes::new(context, ids.clone(), Some(#indexes)),)*
                }
            }

            fn emit(&self, state: &State) {
                self.callback.emit(state);
            }
        }
    };

    let message = quote! {
        #[derive(Debug, Clone)]
        pub enum Message {
            #(#pascal_names(#ty_messages),)*
            State(State),
        }

        impl MessageBase for Message {
            fn deserialize(mut data: MessageData) -> Result<Self, MessageError> {
                match data.pop_id() {
                    #(Some(#indexes) => Ok(Message::#pascal_names(#ty_messages::deserialize(data)?)),)*
                    Some(#state_id) => Ok(Self::State(data.deserialize()?)),
                    Some(_) => Err(data.error(
                        format!("{}::Message::deserialize() unknown id", stringify!(#mod_name)),
                    )),
                    None => Err(data.error(
                        format!("{}::Message::deserialize() data has no more id", stringify!(#mod_name)),
                    )),
                }     
            }
        }
    };

    Ok(quote!{        
        #state
        #node
        #message
    })
}