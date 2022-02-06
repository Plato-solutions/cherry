use std::str::FromStr;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use crate::backend::Backend;
use crate::table::Table;


pub fn impl_table<B: Backend>(table: &Table<B>) -> TokenStream {
    let table_ident = &table.ident;
    let pk_trait = &table.primary_key_trait();
    let primary_key_types_traits = table.primary_key_types_trait();
    let primary_key_types = table.primary_key_types();
    let column_list = table.select_column_list();

    let stream_all = stream_all(table, &column_list);
    let stream_all_paginated = stream_all_paginated::<B>(table, &column_list);

    quote! {
        pub trait #pk_trait : cherry::Table //@TODO have pk_trait extend a base primary key trait, and hide all of the dynamic pieces within it to preserve Table have a shared base trait
        {
            #primary_key_types_traits

            fn pk(&self) -> Self::PkId; //@TODO work through making these PK values and types derived

        }

        impl #pk_trait for #table_ident {
            #primary_key_types

            fn pk(&self) -> Self::PkId {
                self.id
            }
        }

        impl cherry::Table for #table_ident {
            #stream_all
            #stream_all_paginated
        }
    }
}

pub fn impl_id_table<B: Backend>(table: &Table<B>) -> TokenStream {
    if table.primary_key_fields().peekable().peek().is_some() {

    }

    if table.id.is_none() {
        return quote!{}
    }
    let pk_trait = &table.primary_key_trait();
    let table_ident = &table.ident;

    let id_ident = &table.id.as_ref().unwrap().field;
    let id_ty = &table.id.as_ref().unwrap().ty;
    let column_list = table.select_column_list();

    let get = get::<B>(table, &column_list);
    let update = update::<B>(table);
    let delete = delete::<B>(table);

    quote! {
        impl #table_ident {
            #get
            #update
            #delete
        }
    }
}

fn get<B: Backend>(table: &Table<B>, column_list: &str) -> TokenStream {
    let box_future = crate::utils::box_future();
    let get_sql = format!(
        "SELECT {} FROM {} WHERE {} = {}",
        column_list,
        table.table,
        table.id.as_ref().unwrap().column(),
        B::Bindings::default().next().unwrap()
    );

    let pk_trait = &table.primary_key_trait();

    let pm_args:TokenStream = table.primary_key_fields().map(|field| {
            let field_pk = &field.fmt_for_pk();
            quote!{id: <Self as #pk_trait>::#field_pk,}
        })
            .collect();




    quote! {
        pub fn get<'a>(
            #pm_args
            //@TODO need to get primary_key fields here
        ) -> #box_future<'a, sqlx::Result<Self>> {
            Box::pin(async move {
                sqlx::query_as!(Self, #get_sql, id)
                    .fetch_one(Self::pool()?)
                    .await
            })
        }

        // Refresh this row, querying all columns from the database.
        fn reload(
            &mut self,
        ) -> #box_future<sqlx::Result<()>> {
            Box::pin(async move {
                *self = Self::get(self.id).await?;
                Ok(())
            })
        }
    }
}

fn update<B: Backend>(table: &Table<B>) -> TokenStream {
    let box_future = crate::utils::box_future();
    let mut bindings = B::Bindings::default();
    let mut assignments = vec![];
    for field in table.fields_except_id() {
        let fragment = format!("{} = {}", field.column(), bindings.next().unwrap());
        assignments.push(fragment);
    }
    let assignments = assignments.join(", ");

    let update_sql = format!(
        "UPDATE {} SET {} WHERE {} = {}",
        table.table,
        assignments,
        table.id.as_ref().unwrap().column(),
        bindings.next().unwrap()
    );
    let id_argument = &table.id.as_ref().unwrap().field;
    let other_arguments = table.fields_except_id().map(|field| {
        let ident = &field.field;
        let mut out = quote!(self.#ident);

        if field.custom_type {
            let ty = &field.ty;
            out.append_all(quote!(as #ty))
        }

        out
    });

    quote! {
        pub fn update<'a>(
            &'a self,
        ) -> #box_future<'a, sqlx::Result<()>> {
            Box::pin(async move {
                let mut pool = Self::pool()?;
                let mut conn = pool.acquire().await?;
                self.update_with(&mut conn).await?;
                Ok(())
            })
        }

        fn update_with<'a, 'c: 'a>(
        &'a self,
        db: impl sqlx::Executor<'c, Database = cherry::Db> + 'a,
        ) -> #box_future<'a, sqlx::Result<()>> {
            Box::pin(async move {
                sqlx::query!(#update_sql, #( #other_arguments, )* self.#id_argument)
                    .execute(db)
                    .await?;
                Ok(())
            })
        }
    }
}

fn stream_all<B: Backend>(table: &Table<B>, column_list: &str) -> TokenStream {
    let box_stream = crate::utils::box_stream();
    let all_sql = format!("SELECT {} FROM {}", column_list, table.table);

    quote! {
        fn stream_all<'a>(
        ) -> #box_stream<'a, sqlx::Result<Self>> {
            let pool = Self::pool().unwrap(); //@TODO figure out how to surface this error
            sqlx::query_as!(Self, #all_sql)
                .fetch(pool)
        }
    }
}

fn stream_all_paginated<B: Backend>(table: &Table<B>, column_list: &str) -> TokenStream {
    let box_stream = crate::utils::box_stream();
    let mut bindings = B::Bindings::default();
    let all_sql = format!(
        "SELECT {} FROM {} LIMIT {} OFFSET {}",
        column_list,
        table.table,
        bindings.next().unwrap(),
        bindings.next().unwrap()
    );

    quote! {
        fn stream_all_paginated<'a>(
            offset: i64,
            limit: i64,
        ) -> #box_stream<'a, sqlx::Result<Self>> {
            let pool = Self::pool().unwrap(); //@TODO figure out how to surface this error
            sqlx::query_as!(Self, #all_sql, limit, offset)
                .fetch(pool)
        }
    }
}

fn delete<B: Backend>(table: &Table<B>) -> TokenStream {
    let box_future = crate::utils::box_future();
    let id_ty = &table.id.as_ref().unwrap().ty;
    let delete_sql = format!(
        "DELETE FROM {} WHERE {} = {}",
        table.table,
        table.id.as_ref().unwrap().column(),
        B::Bindings::default().next().unwrap()
    );
    #[cfg(feature = "mysql")]
    let result_import = quote!(sqlx::mysql::MySqlQueryResult);
    #[cfg(feature = "postgres")]
    let result_import = quote!(sqlx::postgres::PgQueryResult);
    #[cfg(feature = "sqlite")]
    let result_import = quote!(sqlx::sqlite::SqliteQueryResult);

    quote! {

        /// Deletes this row from the database.
        pub fn delete<'a>(
            self,
        ) -> #box_future<'a, sqlx::Result<()>> {
            Box::pin(async move {
                Self::delete_row_with(Self::pool()?, self.id).await
            })
        }

        /// Delete a row from the database
        fn delete_row<'a>(
            id: #id_ty,
        ) -> #box_future<'a, sqlx::Result<()>> {
            Box::pin(async move {
                Self::delete_row_with( Self::pool()?,id).await
            })
        }

        pub fn delete_row_with<'a, 'c: 'a>(
            db: impl sqlx::Executor<'c, Database = cherry::Db> + 'a,
            id: #id_ty
        ) -> #box_future<'a, sqlx::Result<()>> {
            use #result_import;

            Box::pin(async move {
                let result = sqlx::query!(#delete_sql, id)
                    .execute(db)
                    .await?;
                if result.rows_affected() == 0 {
                    Err(sqlx::Error::RowNotFound)
                } else {
                    Ok(())
                }
            })
        }
    }
}

