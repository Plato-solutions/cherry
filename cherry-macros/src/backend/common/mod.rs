//! Common functionality used for all database backends

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Ident, Type, Visibility};

pub use table::*;
pub use schema::*;

use crate::attrs::{Insertable};
use crate::backend::Backend;
use crate::patch::Patch;
use crate::table::Table;

mod table;
mod schema;

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
            let (func, arg) = getter.or_fallback(field);
            getters.extend(get_one(vis, &func, &arg, &sql));
        }

        if let Some(getter) = &field.get_optional {
            let (func, arg) = getter.or_fallback(field);
            getters.extend(get_optional(vis, &func, &arg, &sql));
        }

        if let Some(getter) = &field.get_many {
            let (func, arg) = getter.or_fallback(field);
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
            by: #by_ty,
        ) -> sqlx::Result<Self> {
            sqlx::query_as!(Self, #sql, by)
                .fetch_one(Self::pool()?)
                .await
        }
    }
}

pub fn get_optional(vis: &Visibility, ident: &Ident, by_ty: &Type, sql: &str) -> TokenStream {
    quote! {
        #vis async fn #ident(
            by: #by_ty,
        ) -> sqlx::Result<Option<Self>> {
            sqlx::query_as!(Self, #sql, by)
                .fetch_optional(Self::pool()?)
                .await
        }
    }
}

pub fn get_many(vis: &Visibility, ident: &Ident, by_ty: &Type, sql: &str) -> TokenStream {
    quote! {
        #vis async fn #ident(
            by: #by_ty,
        ) -> sqlx::Result<Vec<Self>> {
            sqlx::query_as!(Self, #sql, by)
                .fetch_all(Self::pool()?)
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
                    value: #field_ty
                ) -> sqlx::Result<()> {
                    sqlx::query!(#sql, value, <Self as cherry::Table>::id(self))
                        .execute(Self::pool()?)
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

pub(crate) fn impl_patch<B: Backend>(patch: &Patch) -> TokenStream {
    let patch_ident = &patch.ident;
    let table_path = &patch.table;
    let field_idents = &patch
        .fields
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<&Ident>>();
    let query_args = &patch
        .fields
        .iter()
        .map(|field| {
            let field_ident = &field.ident;
            let field_ty = &field.ty;
            match field.custom_type {
                false => quote!(#field_ident),
                true => quote!(#field_ident as #field_ty)
            }
        })
        .collect::<Vec<TokenStream>>();

    let mut bindings = B::Bindings::default();
    let mut assignments = Vec::with_capacity(patch.fields.len());
    for field in &patch.fields {
        let fragment = format!("{} = {}", field.column, bindings.next().unwrap());
        assignments.push(fragment);
    }
    let assignments = assignments.join(", ");

    let sql = format!(
        "UPDATE {} SET {} WHERE {} = {}",
        &patch.table_name,
        assignments,
        patch.id,
        bindings.next().unwrap()
    );

    let box_future = crate::utils::box_future();
    quote! {
        impl cherry::Patch for #patch_ident {
            type Table = #table_path;

            fn apply_to(self, entity: &mut Self::Table) {
                #( entity.#field_idents = self.#field_idents; )*
            }

            fn patch_row<'a>(
                &'a self,
                id: <Self::Table as cherry::Table>::Id,
            ) -> #box_future<'a, sqlx::Result<()>> {
                let db = Table::pool()?;
                Ok(self.patch_row_with(db,id)?)
            }

            fn patch_row_with<'a, 'c: 'a>(
                &'a self,
                db: impl sqlx::Executor<'c, Database = cherry::Db> + 'a,
                id: <Self::Table as cherry::Table>::Id,
            ) -> #box_future<'a, sqlx::Result<()>> {
                Box::pin(async move {
                    sqlx::query!(#sql, #( self.#query_args, )* id)
                        .execute(Table::pool()?)
                        .await?;
                    Ok(())
                })
            }
        }
    }
}

pub(crate) fn insert_struct<B: Backend>(table: &Table<B>) -> TokenStream {
    let Insertable { ident, attrs } = match &table.insertable {
        Some(i) => i,
        None => return quote!(),
    };
    let vis = &table.vis;
    let insert_fields = table.insertable_fields().map(|field| {
        let ident = &field.field;
        let other = &field.other_attrs;
        let ty = &field.ty;
        quote!( #other #vis #ident: #ty )
    });

    let from_impl = impl_from_for_insert_struct(table, ident);
    quote! {
        #(#attrs)*
        #vis struct #ident {
            #( #insert_fields, )*
        }

        #from_impl
    }
}

fn impl_from_for_insert_struct<B: Backend>(table: &Table<B>, insert_struct: &Ident) -> TokenStream {
    let table_ident = &table.ident;

    let fields = table
        .insertable_fields()
        .map(|field| {
            let ident = &field.field;
            quote!(#ident: v.#ident,)
        })
        .collect::<TokenStream>();

    quote! {
        impl From<#table_ident> for #insert_struct {
            fn from(v: #table_ident) -> Self {
                Self {
                    #fields
                }
            }
        }
    }
}


