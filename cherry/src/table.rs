#![cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
//! Lightweight derive macros for bringing orm-like features to sqlx.
//!
//! # Example: Table
//! ```rust,ignore
//! #[derive(ormx::Table)]
//! #[ormx(table = "users", id = user_id, insertable)]
//! struct User {
//!     #[ormx(column = "id")]
//!     user_id: u32,
//!     first_name: String,
//!     last_name: String,
//!     #[ormx(get_optional(&str))]
//!     email: String,
//!     #[ormx(default, set)]
//!     last_login: Option<NaiveDateTime>,
//! }
//! ```
//!
//! # Example: Patch
//! ```rust,ignore
//! #[derive(ormx::Patch)]
//! #[ormx(table_name = "users", table = User, id = "id")]
//! struct UpdateName {
//!     first_name: String,
//!     last_name: String,
//! }
//! ```
//!
//! # Documentation
//! See the docs of [derive(Table)](derive.Table.html) and [Patch](trait.Patch.html).

use futures::future::BoxFuture;
use futures::stream::BoxStream;

use sqlx::{Executor, Result};

pub use cherry_macros::*;
use crate::{connection, Schema};
use crate::types::{Pool};
use async_trait::async_trait;



#[doc(hidden)]
pub mod exports {
    // pub use crate::query2::map::*;
    pub use futures;
}

// #[cfg(any(feature = "mysql", feature = "postgres"))]
// mod query2;

#[cfg(feature = "mysql")]
pub type Db = sqlx::MySql;
#[cfg(feature = "postgres")]
pub type Db = sqlx::Postgres;
#[cfg(feature = "sqlite")]
pub type Db = sqlx::Sqlite;


#[cfg(feature = "mysql")]
pub type ConnectOptions = sqlx::mysql::MySqlConnectOptions;
#[cfg(feature = "postgres")]
pub type ConnectOptions = sqlx::postgres::PgConnectOptions;
#[cfg(feature = "sqlite")]
pub type ConnectOptions = sqlx::sqlite::SqliteConnectOptions;

/// A database table in which each row is identified by a unique ID.
#[async_trait]
pub trait Table : Schema
    where
        Self: Sized + Send + Sync + 'static,
{
    /// Type of the ID column of this table.
    type Id: 'static + Copy + Send;

    /// Returns the id of this row.
    fn id(&self) -> Self::Id;

    /// Returns connection to datasource
    fn pool() -> Result<&'static Pool>  {
        Ok(connection::get(Self::datasource())
            .map_err(|err|{sqlx::error::Error::Configuration(err.into())})?
        )
    }
    // /// Returns connection to datasource
    // async fn begin() -> Result<Transaction<'static>>  {
    //     Ok(connection::get(Self::datasource())
    //         .map_err(|err|{sqlx::error::Error::Configuration(err.into())})?
    //         .begin().await.map_err(|err|{sqlx::error::Error::Configuration(err.into())})?
    //     )
    // }

    /// Insert a row into the database.
    fn insert(
        row: impl Insert<Table = Self>,
    ) -> BoxFuture<'static,Result<Self>> {
        row.insert()
    }

    /// Insert a row into the database.
    fn insert_with(
        db: &mut <Db as sqlx::Database>::Connection,
        row: impl Insert<Table = Self>,
    ) -> BoxFuture<Result<Self>> {
        row.insert_with(db)
    }

    /// Queries the row of the given id.
    fn get<'a>(
        id: Self::Id,
    ) -> BoxFuture<'a, Result<Self>>;

    /// Stream all rows from this table.
    fn stream_all<'a>(
    ) -> BoxStream<'a, Result<Self>>;

    fn stream_all_paginated<'a>(
        offset: i64,
        limit: i64,
    ) -> BoxStream<'a, Result<Self>>;

    /// Load all rows from this table.
    fn all<'a>(
    ) -> BoxFuture<'a, Result<Vec<Self>>> {
        use futures::TryStreamExt;

        Box::pin(Self::stream_all().try_collect())
    }

    fn all_paginated<'a>(
        offset: i64,
        limit: i64,
    ) -> BoxFuture<'a, Result<Vec<Self>>> {
        use futures::TryStreamExt;

        Box::pin(Self::stream_all_paginated( offset, limit).try_collect())
    }
    /// Applies a patch to this row.
    fn patch<P>(
        &mut self,
        patch: P,
    ) -> BoxFuture<Result<()>>
        where
            P: Patch<Table = Self>,
    {
        Box::pin(async move {
            let patch: P = patch;
            patch.patch_row(self.id()).await?;
            patch.apply_to(self);
            Ok(())
        })
    }

    /// Updates all fields of this row, regardless if they have been changed or not.
    fn update(
        &self,
    ) -> BoxFuture<Result<()>>;

    /// Updates all fields of this row, regardless if they have been changed or not.
    fn update_with<'a, 'c: 'a>(
        &'a self,
        db: impl Executor<'c, Database = Db> + 'a,
    ) -> BoxFuture<'a, Result<()>>;

    // Refresh this row, querying all columns from the database.
    fn reload(
        &mut self,
    ) -> BoxFuture<Result<()>> {
        Box::pin(async move {
            *self = Self::get(self.id()).await?;
            Ok(())
        })
    }

    /// Delete a row from the database
    fn delete_row<'a>(
        id: Self::Id,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            Self::delete_row_with( Self::pool()?,id).await
        })
    }

    /// Delete a row from the database
    fn delete_row_with<'a, 'c: 'a>(
        db: impl Executor<'c, Database = Db> + 'a,
        id: Self::Id,
    ) -> BoxFuture<'a, Result<()>>;


    /// Deletes this row from the database.
    fn delete<'a>(
        self,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            Self::delete_row_with(Self::pool()?, self.id()).await
        })
    }

    /// Deletes this row from the database.
    fn delete_with<'a, 'c: 'a>(
        self,
        db: impl Executor<'c, Database = Db> + 'a,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            Self::delete_row_with(db, self.id()).await
        })
    }
}

/// A type which can be used to "patch" a row, updating multiple fields at once.
pub trait Patch
    where
        Self: Sized + Send + Sync + 'static,
{
    type Table: Table;

    /// Applies the data of this patch to the given entity.
    /// This does not persist the change in the database.
    fn apply_to(self, entity: &mut Self::Table);

    /// Applies this patch to a row in the database.
    fn patch_row<'a>(
        &'a self,
        id: <Self::Table as Table>::Id,
    ) -> BoxFuture<'a, Result<()>>;

    /// Applies this patch to a row in the database.
    fn patch_row_with<'a, 'c: 'a>(
        &'a self,
        db: impl Executor<'c, Database = Db> + 'a,
        id: <Self::Table as Table>::Id,
    ) -> BoxFuture<'a, Result<()>>;
}

/// A type which can be inserted as a row into the database.
pub trait Insert
    where
        Self: Sized + Send + Sync + 'static,
{
    type Table: Table;

    /// Insert a row into the database, returning the inserted row.
    fn insert(self) -> BoxFuture<'static,Result<Self::Table>>;

    /// Insert a row into the database, returning the inserted row.
    fn insert_with(
        self,
        db: &mut <Db as sqlx::Database>::Connection,
    ) -> BoxFuture<Result<Self::Table>>;
}
