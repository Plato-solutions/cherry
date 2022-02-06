use std::convert::TryFrom;

use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{DeriveInput, Result, Type, Visibility};

use crate::attrs::{Getter, Insertable, Queryable};
use crate::backend::{Backend, Implementation};
use std::borrow::Cow;
use std::marker::PhantomData;
use std::str::FromStr;
use inflector::cases::titlecase::to_title_case;


mod parse;
use crate::schema::Schema;

pub struct Table<B: Backend> {
    pub ident: Ident,
    pub vis: Visibility,
    pub table: String,
    pub id: Option<TableField<B>>,
    pub fields: Vec<TableField<B>>,
    pub insertable: Option<Insertable>,
}

#[derive(Clone)]
pub struct TableField<B: Backend> {
    pub field: Ident,
    pub ty: Type,
    pub column_name: String,
    pub custom_type: bool,
    pub reserved_ident: bool,
    pub pk: bool,
    pub default: bool,
    pub get_one: Option<Getter>,
    pub get_optional: Option<Getter>,
    pub get_many: Option<Getter>,
    pub set: Option<Ident>,
    pub _phantom: PhantomData<*const B>,
}

impl<B: Backend> Table<B> {
    pub fn fields_except_id(&self) -> impl Iterator<Item = &TableField<B>> + Clone {
        let id = self.id.as_ref().expect("fields_except_id called when table is not IDable").field.clone();
        self.fields.iter().filter(move |field| field.field != id)
    }

    pub fn insertable_fields(&self) -> impl Iterator<Item = &TableField<B>> + Clone {
        self.fields.iter().filter(|field| !field.default)
    }

    pub fn default_fields(&self) -> impl Iterator<Item = &TableField<B>> + Clone {
        self.fields.iter().filter(|field| field.default)
    }

    pub fn primary_key_fields(&self) -> impl Iterator<Item = &TableField<B>> + Clone {
        self.fields.iter().filter(|field| field.pk)
    }

    pub fn select_column_list(&self) -> String {
        self.fields
            .iter()
            .map(|field| field.fmt_for_select())
            .join(", ")
    }

    pub fn table_trait(&self) -> TokenStream {
        let pk = format!("{}{}","Pk",to_title_case(self.table.to_string().to_owned().as_str()));
        let stream: proc_macro2::TokenStream = pk.parse().unwrap();
        stream
    }

    pub fn primary_key_trait(&self) -> TokenStream {
        let pk = format!("{}{}","Pk",to_title_case(self.table.to_string().to_owned().as_str()));
        let stream: proc_macro2::TokenStream = pk.parse().unwrap();
        stream
    }

    pub fn primary_key_types_trait(&self) -> TokenStream {
        self.primary_key_fields().map(|field|{
            let field_pk = &field.fmt_for_pk();
            quote!{type #field_pk;}
        })
            .collect()
    }

    pub fn primary_key_types(&self) -> TokenStream {
        self.primary_key_fields().map(|field|{

            let field_ty = &field.ty;
            let field_pk = &field.fmt_for_pk();
            quote!{
                type #field_pk = #field_ty;
            }
        })
            .collect()
    }

    // pub fn primary_key_arguments(&self) -> TokenStream {
    //     TokenStream::from_str(&*self.primary_key_fields().map(|field| {
    //         let field_ty = &field.ty;
    //         let field_pk = &field.fmt_for_pk();
    //         quote!{#field_pk: #field_ty,}
    //     })
    //         .join(", ")).unwrap()
    // }

    // pub fn primary_key_where(&self) -> TokenStream {
    //     self.primary_key_fields().map(|field|{
    //         let field_pk = &field.fmt_for_pk();
    //         quote!(type #field_pk = #field_ty;
    //         )
    //     })
    //         .collect()
    // }
}

impl<B: Backend> TableField<B> {
    pub fn fmt_for_select(&self) -> String {
        if self.custom_type {
            format!(
                "{} AS {}{}: _{}",
                self.column(),
                B::QUOTE,
                self.field,
                B::QUOTE
            )
        } else if self.field == self.column_name {
            self.column().into()
        } else {
            format!("{} AS {}", self.column(), self.field)
        }
    }

    pub fn column(&self) -> Cow<str> {
        if self.reserved_ident {
            format!("{}{}{}", B::QUOTE, self.column_name, B::QUOTE).into()
        } else {
            Cow::Borrowed(&self.column_name)
        }
    }

    pub fn fmt_for_argument(&self) -> TokenStream {
        let pk_field = &self.field;
        let pk_type = self.fmt_for_pk();

        quote!{
            #pk_field = Self::Id,
        }
    }

    pub fn fmt_for_pk(&self) -> TokenStream {
        let pk = format!("{}{}","Pk",to_title_case(self.field.to_string().to_owned().as_str()));
        let stream: proc_macro2::TokenStream = pk.parse().unwrap();
        stream
    }
}

impl Getter {
    pub fn or_fallback<B: Backend>(&self, field: &TableField<B>) -> (Ident, Type) {
        let ident = self
            .func
            .clone()
            .unwrap_or_else(|| Ident::new(&format!("by_{}", field.field), Span::call_site()));
        let arg = self.arg_ty.clone().unwrap_or_else(|| {
            let ty = &field.ty;
            syn::parse2(quote!(&#ty)).unwrap()
        });
        (ident, arg)
    }
}

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let parsed = Table::try_from(input)?;


    let impl_table = Implementation::impl_table(&parsed);
    let insert_struct = Implementation::insert_struct(&parsed);
    let impl_insert = Implementation::impl_insert(&parsed);

    if parsed.id.is_none() {
        Ok(quote! {
            #impl_table
            #insert_struct
            #impl_insert
        })
    } else {
        let impl_id_table = Implementation::impl_id_table(&parsed);
        let getters = Implementation::impl_getters(&parsed);
        let setters = Implementation::impl_setters(&parsed);

        Ok(quote! {
            #impl_table
            #impl_id_table
            #insert_struct
            #impl_insert
            #getters
            #setters
        })
    }



}



