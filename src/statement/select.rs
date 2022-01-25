use std::any::TypeId;
use std::marker::PhantomData;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, connection, clause};
use crate::query::builder::QueryBuilder;
use crate::statement::Execute;
use crate::types::{Arguments, Database, Result};

pub struct Select<'a, T> {
    _keep: PhantomData<T>,
    pub(crate) query: QueryBuilder<'a>,
}

impl<'a, T> Select<'a, T> where T: Cherry {

    pub(crate) fn new(datasource: TypeId) -> Self {
        Self {
            _keep: PhantomData,
            query: QueryBuilder::new::<T>(datasource, SqlBuilder::select_from(T::table()))
        }
    }

    fn build_sql(&mut self) -> Result<String> {
        Ok(self.query.sql_builder.sql()?)
    }

    pub fn field<S: ToString>(mut self, f: S) -> Self {
        self.query.sql_builder.field(f);
        self
    }

    pub fn fields<S: ToString>(mut self, fields: &[S]) -> Self {
        self.query.sql_builder.fields(fields);
        self
    }

    pub fn fields_all(mut self) -> Self {
        self.query.sql_builder.fields(&T::columns());
        self
    }

    pub fn order_asc<S: ToString>(mut self, f: S) -> Self {
        self.query.sql_builder.order_asc(f);
        self
    }

    pub fn order_desc<S: ToString>(mut self, f: S) -> Self {
        self.query.sql_builder.order_desc(f);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.query.sql_builder.limit(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.query.sql_builder.offset(offset);
        self
    }

    pub async fn fetch(mut self) -> Result<Option<T>> {
        let row = sqlx::query_with(
            self.build_sql()?.as_str(),
            self.query.arguments
        ).fetch_optional(connection::get(self.query.datasource)?).await?;
        match row {
            Some(row) => Ok(Some(T::from_row(&row)?)),
            _ => Ok(None)
        }
    }

    pub async fn fetch_all(mut self) -> Result<Vec<T>> {
        let rows = sqlx::query_with(
            self.build_sql()?.as_str(),
            self.query.arguments
        ).fetch_all(connection::get(self.query.datasource)?).await?;
        let mut vec = Vec::with_capacity(rows.len());
        for row in rows {
            vec.push(T::from_row(&row)?);
        }
        Ok(vec)
    }

}


impl <'a,T>crate::statement::Statement<'a> for Select<'a, T> {
    fn query(&'a mut self) -> (&'a mut Select<'a,T>, &'a mut QueryBuilder<'a>){
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

impl <'a,T> crate::clause::Where<'a> for Select<'a, T> {
    type Statement = Select<'a, T>;
}

impl <'a,T> crate::clause::Like<'a> for Select<'a, T> {
    type Statement = Select<'a, T>;
}

impl <'a,T> crate::clause::Order<'a> for Select<'a, T> {
    type Statement = Select<'a, T>;
}