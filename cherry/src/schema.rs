use crate::types::{Arguments, Result, Row};

use std::any::TypeId;
use crate::{connection};
use crate::query::select::Select;
use crate::types::{Pool};
use crate::types::Transaction;
use async_trait::async_trait;

#[async_trait]
pub trait Schema: Sized + Send + Unpin {
    /// Return database table name
    fn table() -> &'static str;

    /// Return table's column names
    fn columns() -> Vec<&'static str>;

    /// Add each column's value as argument from appropriate field
    fn arguments<'a>(&'a self, arguments: &mut Arguments<'a>);

    /// Process row into data structure
    fn from_row(row: &Row) -> Result<Self>;

    /// Returns datasource to store and receive data for structure
    fn datasource() -> TypeId;

    // async fn begin<'a>(&'static self) -> Result<Transaction<'a>> {
    //     Ok(connection::get(Self::datasource())
    //         .map_err(|err|{sqlx::error::Error::Configuration(err.into())})?
    //         .begin().await?)
    // }

    /// Insert a row into the database.
    fn select<'a>(
    ) -> Select<'a,Self> {
        Select::new(Self::datasource())
    }
}
