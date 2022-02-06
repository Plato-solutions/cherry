use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    backend::Backend,
    schema::{Schema},
};

pub fn impl_schema<B: Backend>(table: &Schema<B>) -> TokenStream {
    let table_ident = &table.ident;
    let datasource = &table.datasource;

    let name = name::<B>(table);
    let columns = columns::<B>(table);
    let arguments = arguments::<B>(table);
    let from_row = from_row::<B>(table);

    quote! {
        impl cherry::Schema for #table_ident {
            fn datasource() -> std::any::TypeId { std::any::TypeId::of::<#datasource>() }

            #name
            #columns
            #arguments
            #from_row
        }
    }
}


fn name<B: Backend>(table: &Schema<B>) -> TokenStream {
    let table_name = &table.table;

    quote! {
        fn table() -> &'static str {
            #table_name
        }
    }
}

fn columns<B: Backend>(table: &Schema<B>) -> TokenStream {
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

fn arguments<B: Backend>(table: &Schema<B>) -> TokenStream {
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

fn from_row<B: Backend>(table: &Schema<B>) -> TokenStream {
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


// fn transaction() -> TokenStream {
//     quote! {
//         fn transaction(
//         ) -> Transaction {
//             Self::pool()?.begin().await?
//         }
//     } //    fn select<'a, T>(&'static self) -> Select<'a, T> where T: Schema + 'static {
// }
//
// fn select() -> TokenStream {
//     quote! {
//         fn select(
//         ) -> Select {
//             Self::datasource().select();
//         }
//     }
// }