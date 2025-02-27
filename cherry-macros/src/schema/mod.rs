use std::convert::TryFrom;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Result, Visibility};

use crate::attrs::{Queryable};
use crate::backend::{Backend, Implementation};

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


impl<B: Backend> Schema<B> {
    pub fn mapped_fields(&self) -> impl Iterator<Item = &TableField<B>> + Clone {
        self.fields.iter().filter(|field| !field.unmapped)
    }

    pub fn unmapped_fields(&self) -> impl Iterator<Item = &TableField<B>> + Clone {
        self.fields.iter().filter(|field| field.unmapped)
    }
}

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let parsed = Schema::try_from(input)?;
    let impl_schema = Implementation::impl_schema(&parsed);

    Ok(quote! {
        #impl_schema
    })
}



