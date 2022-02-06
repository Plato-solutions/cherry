use std::convert::TryFrom;

use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Result, Type, Visibility};

use crate::attrs::{Getter, Insertable, Queryable};
use crate::backend::{Backend, Implementation};
use std::borrow::Cow;
use std::marker::PhantomData;

use crate::table::TableField;
mod parse;

pub struct Schema<B: Backend> {
    pub ident: Ident,
    pub vis: Visibility,
    pub table: String,
    pub fields: Vec<TableField<B>>,
    pub queryable: Option<Queryable>,
    pub datasource: Ident,
}


pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let parsed = Schema::try_from(input)?;
    let impl_schema = Implementation::impl_schema(&parsed);

    Ok(quote! {
        #impl_schema
    })
}



