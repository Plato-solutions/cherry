// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub(crate) mod schema;
pub(crate) mod datasource;
pub(crate) mod query;
pub(crate) mod table;


pub use cherry_macros::*;

pub use {
    schema::Schema,
    datasource::DataSource,
    table::{Table,Insert,Patch,Db,Insertable,IdTable}
};

pub mod types;
pub mod connection;

pub mod error {
    pub use anyhow::Error;
}

#[doc(hidden)]
pub mod exports {
    pub use futures;
}

pub mod sqlx {
    pub use sqlx::{Database, Decode, Encode, Arguments, Row, types::Type};

    #[cfg(feature = "json")]
    pub use sqlx::types::Json;
    #[cfg(feature = "uuid")]
    pub use sqlx::types::Uuid;

    // #[cfg(feature = "mysql")]
    // pub use sqlx::mysql::{MySql, MySqlArguments, MySqlQueryResult, MySqlRow};
    // #[cfg(feature = "postgres")]
    // pub use sqlx::postgres::{PgArguments, PgQueryResult, PgRow, Postgres};
    // #[cfg(feature = "sqlite")]
    // pub use sqlx::sqlite::{Sqlite, SqliteArguments, SqliteQueryResult, SqliteRow};
    // #[cfg(feature = "mssql")]
    // pub use sqlx::mssql::{Mssql, MssqlArguments, MssqlQueryResult, MssqlRow};
}

#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite", feature = "mssql")))]
compile_error!("one of the features ['mysql', 'postgres', 'sqlite', 'mssql'] must be enabled");


