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

pub fn parse_events(input: T) -> Result<Vec<Ident>> {
    let list = punctuated::Punctuated::<Ident, Token![,]>::parse_terminated
        .parse2(input)
        .map_err(|e| Error::new(e.span(), "expected a comma-separated list of events"))?;
    Ok(list.into_iter().collect())
}
