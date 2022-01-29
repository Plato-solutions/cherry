use std::any::Any;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    backend::Backend,
    table::{Table, TableField},
};

pub fn impl_schema<B: Backend>(table: &Table<B>) -> TokenStream {
    let table_ident = &table.ident;
    let name = name::<B>(table);
    let columns = columns::<B>(table);
    let arguments = arguments::<B>(table);
    let from_row = from_row::<B>(table);

    quote! {
        impl cherry::Schema for #table_ident {
            #name
            #columns
            #arguments
            #from_row
        }
    }
}


fn name<B: Backend>(table: &Table<B>) -> TokenStream {
    let table_name = &table.table;

    quote! {
        fn table() -> &'static str {
            #table_name
        }
    }
}

fn columns<B: Backend>(table: &Table<B>) -> TokenStream {
    let fields : proc_macro2::TokenStream = table.fields
        .iter()
        .map(|s|
            format!(" \"{}\"", s.column())
        ).join(", ").parse().unwrap();

    quote! {
        fn columns() -> Vec<&'static str> {
                vec![ #fields]
            }
    }
}

fn arguments<B: Backend>(table: &Table<B>) -> TokenStream {
    let arguments : proc_macro2::TokenStream = table.fields
        .iter().map(|s|
        format!(" arguments.add(&self.{}); ", s.field)
    ).collect::<String>().parse().unwrap();

    quote! {
        fn arguments<'a>(&'a self, arguments: &mut cherry::types::Arguments<'a>) {
            use cherry::sqlx::Arguments as OtherArguments;
            #arguments
        }
    }
}

fn from_row<B: Backend>(table: &Table<B>) -> TokenStream {
    let from_row : proc_macro2::TokenStream = table.fields
        .iter()
        .map(|field|
            format!(" {0}: row.try_get(\"{1}\")?", field.field, field.column())
        ).join(", ").parse().unwrap();

    quote! {
        fn from_row(row: &cherry::types::Row) -> Result<Self, cherry::error::Error> {
            use cherry::sqlx::Row as OtherRow;
            Ok( Self { #from_row } )
        }
    }
}