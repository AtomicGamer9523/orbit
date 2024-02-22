use crate::*;
use syn::*;

use parse::Parser;

pub fn parse_name(input: T) -> Result<Ident> {
    let struct_data = parse2::<ItemStruct>(input).map_err(|e|
        Error::new(e.span(), "expected a struct")
    )?;
    if
        struct_data.semi_token.is_none() ||
        struct_data.generics.params.len() > 0 ||
        struct_data.generics.where_clause.is_some() ||
        struct_data.generics.type_params().count() > 0 ||
        struct_data.generics.lifetimes().count() > 0 ||
        struct_data.generics.const_params().count() > 0 ||
        struct_data.fields.len() > 0
    {
        return Err(Error::new_spanned(struct_data,
            "expected a struct with no fields, no generics, and no where clause!"
        ));
    }
    Ok(struct_data.ident)
}

pub struct Event {
    pub name: Ident,
    // pub ty: Type,
    pub sub: bool,
    pub emit: bool
}

impl Event {
    pub fn span(&self) -> proc_macro2::Span {
        self.name.span()
    }
}

impl parse::Parse for Event {
    /// ### Rules
    /// From this: `Message`
    /// Into this: `Event { name: Ident(Message), _type: Type(Message), sub: true, emit: true }`
    ///
    /// From this: `Player(sub)`
    /// Into this: `Event { name: Ident(Player), _type: Type(Player), sub: true, emit: false }`
    ///
    /// From this: `Player(emit)`
    /// Into this: `Event { name: Ident(Player), _type: Type(Player), sub: false, emit: true }`
    ///
    /// From this: `Player(sub, emit)`
    /// Into this: `Event { name: Ident(Player), _type: Type(Player), sub: true, emit: true }`
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut sub = false;
        let mut emit = false;
        if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            let mut sub_emitted = false;
            while !content.is_empty() {
                let ident = content.parse::<Ident>()?;
                if ident == "sub" {
                    if sub_emitted {
                        return Err(Error::new(ident.span(), "expected a comma-separated list of events"));
                    }
                    sub = true;
                    sub_emitted = true;
                } else if ident == "emit" {
                    if sub_emitted {
                        return Err(Error::new(ident.span(), "expected a comma-separated list of events"));
                    }
                    emit = true;
                    sub_emitted = true;
                } else {
                    return Err(Error::new(ident.span(), "expected `sub` or `emit`"));
                }
                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
            }
        } else {
            sub = true;
            emit = true;
        }
        Ok(Event { name, sub, emit })
    }
}

pub fn parse_events(input: T) -> Result<Vec<Event>> {
    punctuated::Punctuated::<Event, Token![,]>::parse_terminated
        .parse2(input)
        .map_err(|e| Error::new(e.span(), format!("expected a comma-separated list of events; {e}")))
        .map(|v| v.into_iter().collect())
}
