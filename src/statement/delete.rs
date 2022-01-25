use std::any::TypeId;
use std::borrow::BorrowMut;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, connection};
use crate::query::builder::QueryBuilder;
use crate::types::{Arguments, Database, QueryResult, Result, Transaction};

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
    fn query(&'a mut self) -> (&'a mut Delete<'a>, &'a mut QueryBuilder<'a>){
        (self,&mut self.query)
    }

    fn datasource(&'a self) -> TypeId {
        self.query.datasource
    }

    fn arguments(self) -> Arguments<'a> {
        self.query.arguments
    }

    fn build_sql<'s:'a>(&'s  mut self) -> crate::types::Result<String> {
        let (self2,query) = self.query();
        Ok(query.sql_builder.sql()?)
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

