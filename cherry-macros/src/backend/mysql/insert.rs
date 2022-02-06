use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::table::{Table, TableField};

use super::MySqlBackend;
use crate::backend::mysql::MySqlBindings;

pub fn impl_insert(table: &Table<MySqlBackend>) -> TokenStream {
    let insert_ident = match &table.insertable {
        Some(i) => &i.ident,
        None => return quote!(),
    };

    let table_ident = &table.ident;
    let box_future = quote!(cherry::exports::futures::future::BoxFuture);

    let insert_with = insert_with(&table);
    let query_id = query_id(&table);
    let query_default = query_default(&table);
    let construct_row = construct_row(&table);

    quote! {
        impl cherry::Insert for #insert_ident {
            type Table = #table_ident;

            fn insert(
                self,
            ) -> #box_future<'static,sqlx::Result<Self::Table>> {
                Box::pin(async move {
                    let mut pool = #table_ident::pool()?;
                    let mut conn = pool.acquire().await?;

                    Ok(self.insert_with(&mut conn).await?)
                })
            }

            fn insert_with(
                self,
                db: &mut sqlx::MySqlConnection,
            ) -> #box_future<sqlx::Result<Self::Table>> {
                Box::pin(async move {
                    // let mut pool = #table_ident::pool()?;
                    // let mut q = pool.acquire().await?;
                    // let mut conn = db;

                    #insert_with
                    #query_id
                    #query_default
                    Ok(#construct_row)
                })
            }
        }
    }
}

/// build an instance of the table struct from
/// - `_id` (see `query_id` below)
/// - `_generated` (see `query_default` below)
/// - all fields already present in the insert struct
fn construct_row(table: &Table<MySqlBackend>) -> TokenStream {
    let id_ident = &table.id.as_ref().unwrap().field;
    let insert_field_idents = table
        .insertable_fields()
        .map(|f| &f.field)
        .filter(|f| *f != id_ident);
    let default_field_idents = table
        .default_fields()
        .map(|f| &f.field)
        .filter(|f| *f != id_ident);

    quote! {
        Self::Table {
            #id_ident: _id as _,
            #( #insert_field_idents: self.#insert_field_idents, )*
            #( #default_field_idents: _generated.#default_field_idents, )*
        }
    }
}

/// queries default fields from the database, except the ID.
fn query_default(table: &Table<MySqlBackend>) -> TokenStream {
    let mut default_fields = table
        .default_fields()
        .filter(|f| f.field != table.id.as_ref().unwrap().field)
        .peekable();

    if default_fields.peek().is_none() {
        return quote!();
    }

    let query_default_sql = format!(
        "SELECT {} FROM {} WHERE {} = ?",
        default_fields.map(TableField::fmt_for_select).join(", "),
        table.table,
        table.id.as_ref().unwrap().column()
    );

    quote! {
        let _generated = sqlx::query!(#query_default_sql, _id)
            .fetch_one(db)
            .await?;
    }
}


/// inserts the struct into the database
fn insert_with(table: &Table<MySqlBackend>) -> TokenStream {
    let insert_fields: Vec<_> = table.insertable_fields().collect();
    let insert_field_idents = insert_fields.iter().map(|field| &field.field);

    let insert_sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table.table,
        insert_fields.iter().map(|field| field.column()).join(", "),
        MySqlBindings.take(insert_fields.len()).join(", ")
    );

    quote! {
        sqlx::query!(#insert_sql, #( self.#insert_field_idents, )*)
            .execute(db as &mut sqlx::MySqlConnection)
            .await?;
    }
}

/// obtains the id of the inserted row.
///
/// case 1:
///     The ID is database generated, so we query it with LAST_INSERT_ID
/// case 2:
///     The ID is already known, so we can just use it.
fn query_id(table: &Table<MySqlBackend>) -> TokenStream {
    match table.id.as_ref().unwrap().default {
        true => quote! {
            let _id = sqlx::query!("SELECT LAST_INSERT_ID() AS id")
                .fetch_one(db as &mut sqlx::MySqlConnection)
                .await?
                .id;
        },
        false => {
            let id_ident = &table.id.as_ref().unwrap().field;
            quote!(let _id = self.#id_ident;)
        }
    }
}
