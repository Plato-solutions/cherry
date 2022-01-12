use std::any::TypeId;
use std::borrow::BorrowMut;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, connection};
use crate::query::builder::QueryBuilder;
use crate::types::{Database, QueryResult, Result, Transaction};

pub struct Delete<'a> {
    pub(crate) query: QueryBuilder<'a>,
}

impl<'a> Delete<'a> {

    pub(crate) fn new<T: Cherry>(datasource: TypeId) -> Self {
        Self {
            query: QueryBuilder::new::<T>(datasource, SqlBuilder::delete_from(T::table()))
        }
    }

}

impl <'a>crate::statement::Statement<'a> for Delete<'a> {
    fn query(&'a mut self) -> &'a mut QueryBuilder<'a> {
        &mut self.query
    }
}

impl <'a>crate::statement::Execute<'a> for Delete<'a> {}

impl <'a>crate::clause::Where<'a> for Delete<'a> {
    type Statement = Delete<'a>;
}

impl <'a>crate::clause::Like<'a> for Delete<'a> {
    type Statement = Delete<'a>;
}

impl <'a>crate::clause::Order<'a> for Delete<'a> {
    type Statement = Delete<'a>;
}

