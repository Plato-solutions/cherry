//! Common functionality used for all database backends

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, Visibility};

pub use cherry::*;

use crate::attrs::Insertable;
use crate::backend::Backend;
use crate::patch::Patch;
use crate::cherry::Table;

mod table;

pub(crate) fn getters<B: Backend>(table: &Table<B>) -> TokenStream {
    let column_list = table.select_column_list();
    let vis = &table.vis;
    let mut getters = TokenStream::new();

    for field in table.fields.iter() {
        let sql = format!(
            "SELECT {} FROM {} WHERE {} = {}",
            column_list,
            table.table,
            field.column(),
            B::Bindings::default().next().unwrap()
        );

        if let Some(getter) = &field.get_one {
            let (func, arg) = getter.or_fallback(&field);
            getters.extend(get_one(vis, &func, &arg, &sql));
        }

        if let Some(getter) = &field.get_optional {
            let (func, arg) = getter.or_fallback(&field);
            getters.extend(get_optional(vis, &func, &arg, &sql));
        }

        if let Some(getter) = &field.get_many {
            let (func, arg) = getter.or_fallback(&field);
            getters.extend(get_many(vis, &func, &arg, &sql));
        }
    }

    let table_ident = &table.ident;
    quote! {
        impl #table_ident {
            #getters
        }
    }
}

pub fn get_one(vis: &Visibility, ident: &Ident, by_ty: &Type, sql: &str) -> TokenStream {
    quote! {
        #vis async fn #ident(
            db: impl sqlx::Executor<'_, Database = cherry::Db>,
            by: #by_ty,
        ) -> sqlx::Result<Self> {
            sqlx::query_as!(Self, #sql, by)
                .fetch_one(db)
                .await
        }
    }
}

pub fn get_optional(vis: &Visibility, ident: &Ident, by_ty: &Type, sql: &str) -> TokenStream {
    quote! {
        #vis async fn #ident(
            db: impl sqlx::Executor<'_, Database = cherry::Db>,
            by: #by_ty,
        ) -> sqlx::Result<Option<Self>> {
            sqlx::query_as!(Self, #sql, by)
                .fetch_optional(db)
                .await
        }
    }
}

pub fn get_many(vis: &Visibility, ident: &Ident, by_ty: &Type, sql: &str) -> TokenStream {
    quote! {
        #vis async fn #ident(
            db: impl sqlx::Executor<'_, Database = cherry::Db>,
            by: #by_ty,
        ) -> sqlx::Result<Vec<Self>> {
            sqlx::query_as!(Self, #sql, by)
                .fetch_all(db)
                .await
        }
    }
}

pub fn setters<B: Backend>(table: &Table<B>) -> TokenStream {
    let vis = &table.vis;
    let mut setters = TokenStream::new();

    for field in table.fields.iter() {
        let field_ident = &field.field;
        let field_ty = &field.ty;

        if let Some(fn_name) = &field.set {
            let mut bindings = B::Bindings::default();
            let sql = format!(
                "UPDATE {} SET {} = {} WHERE {} = {}",
                table.table,
                field.column(),
                bindings.next().unwrap(),
                table.id.column(),
                bindings.next().unwrap(),
            );
            setters.extend(quote! {
                #vis async fn #fn_name(
                    &mut self,
                    db: impl sqlx::Executor<'_, Database = cherry::Db>,
                    value: #field_ty
                ) -> sqlx::Result<()> {
                    sqlx::query!(#sql, value, <Self as cherry::Table>::id(self))
                        .execute(db)
                        .await?;
                    self.#field_ident = value;
                    Ok(())
                }
            })
        }
    }

    let table_ident = &table.ident;
    quote! {
        impl #table_ident {
            #setters
        }
    }
}

//
// pub(crate) fn insert_struct<B: Backend>(table: &Table<B>) -> TokenStream {
//     let Insertable { ident, attrs } = match &table.insertable {
//         Some(i) => i,
//         None => return quote!(),
//     };
//     let vis = &table.vis;
//     let insert_fields = table.insertable_fields().map(|field| {
//         let ident = &field.field;
//         let ty = &field.ty;
//         quote!(#vis #ident: #ty)
//     });
//
//     let from_impl = impl_from_for_insert_struct(table, ident);
//     quote! {
//         #(#attrs)*
//         #vis struct #ident {
//             #( #insert_fields, )*
//         }
//
//         #from_impl
//     }
// }
//
// fn impl_from_for_insert_struct<B: Backend>(table: &Table<B>, insert_struct: &Ident) -> TokenStream {
//     let table_ident = &table.ident;
//
//     let fields = table
//         .insertable_fields()
//         .map(|field| {
//             let ident = &field.field;
//             quote!(#ident: v.#ident,)
//         })
//         .collect::<TokenStream>();
//
//     quote! {
//         impl From<#table_ident> for #insert_struct {
//             fn from(v: #table_ident) -> Self {
//                 Self {
//                     #fields
//                 }
//             }
//         }
//     }
// }
