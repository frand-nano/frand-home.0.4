use parse::ParseStream;
use syn::{*, parse::Parse, punctuated::Punctuated};
use token::Paren;
use crate::attrs::{Attr, AttrsItem};

pub struct ComponentAttrKeyItem {
    pub key: Ident,
    pub _paren: Paren,
    pub item: ComponentAttrItem,
    pub _comma: Option<Token![,]>,
}

pub enum ComponentAttrItem {
    Ident(Ident),
    Attrs(Punctuated<Attr, Token![,]>),
    #[allow(dead_code)] None,
}

impl AttrsItem for ComponentAttrKeyItem {
    fn key(&self) -> &Ident { &self.key }
}

impl Parse for ComponentAttrKeyItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let key = input.parse()?;
        let _paren = parenthesized!(content in input);

        let lookahead = content.lookahead1();

        let item = if lookahead.peek(Ident) {
            ComponentAttrItem::Ident(content.parse()?)
        } else if lookahead.peek(Token![#]) {
            ComponentAttrItem::Attrs(Punctuated::parse_terminated(&content)?)
        } else {
            Err(lookahead.error())?
        };

        Ok(Self {
            key,
            _paren,
            item,
            _comma: input.parse()?,
        })
    }
} 