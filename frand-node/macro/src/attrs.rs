use std::{collections::HashMap, ops::Deref};

use parse::ParseStream;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{*, parse::Parse};
use token::Bracket;

pub trait AttrsItem: Parse {
    fn key(&self) -> &Ident;
}

pub struct Attrs<I: AttrsItem> {
    map: HashMap<String, I>,
}

impl<I: AttrsItem> Parse for Attrs<I> {
    fn parse(input: ParseStream) -> Result<Self> {        
        let mut map = HashMap::new();

        while input.peek(Ident) {
            let key_item: I = input.parse()?;
            map.insert(key_item.key().to_string(), key_item);
        }

        Ok(Self {
            map,
        })
    }
} 

impl<I: AttrsItem> Deref for Attrs<I> {    
    type Target = HashMap<String, I>;    
    fn deref(&self) -> &Self::Target { &self.map }
}

impl<I: AttrsItem> Attrs<I> {    
    pub fn find<F>(&self, key: &str, f: F) -> Result<Option<TokenStream>>
    where F: FnOnce(&I) -> Result<TokenStream>
    {
        match self.get(key).map(f) {
            Some(Ok(attr)) => Ok(Some(attr)),
            Some(Err(err)) => Err(err),
            None => Ok(None),
        }
    }
}

pub struct Attr {
    pub sharp: Token![#],
    pub bracket: Bracket,
    pub tokens: TokenStream,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            sharp: input.parse()?,
            bracket: bracketed!(content in input),
            tokens: content.parse()?,
        })
    }
} 

impl ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.sharp.to_tokens(tokens);
        self.bracket.surround(tokens, |tokens| self.tokens.to_tokens(tokens));
    }
}