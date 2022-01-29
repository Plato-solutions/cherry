use std::any::TypeId;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Schema, connection, gen_execute, gen_where};
use crate::query::query_builder::QueryBuilder;
use crate::types::{Database, QueryResult, Result, Transaction};

pub struct Delete<'a> {
    pub(crate) query: QueryBuilder<'a>,
}

impl<'a> Delete<'a> {

    pub(crate) fn new<T: Schema>(datasource: TypeId) -> Self {
        Self {
            query: QueryBuilder::new::<T>(datasource, SqlBuilder::delete_from(T::table()))
        }
    }

    fn build_sql(&mut self) -> Result<String> {
        Ok(self.query.sql_builder.sql()?)
    }

    gen_where!();
    gen_execute!();

}
