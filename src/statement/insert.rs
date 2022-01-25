use std::any::TypeId;
use std::borrow::BorrowMut;

use sql_builder::SqlBuilder;

use crate::{Cherry, connection, query};
use crate::query::builder::QueryBuilder;
use crate::types::{Arguments, QueryResult, Result, Transaction};

pub struct Insert<'a> {
    pub(crate) query: QueryBuilder<'a>,
    pub(crate) columns: Vec<&'static str>,
    pub(crate) replace: Option<(String, String)>,
    pub(crate) size: usize,
}

impl<'a> Insert<'a> {

    pub(crate) fn new<T>(datasource: TypeId) -> Self where T: Cherry {
        Self {
            query: QueryBuilder::new::<T>(datasource, SqlBuilder::insert_into(T::table())),
            columns: T::columns(),
            replace: None,
            size: 0,
        }
    }

    pub(crate) fn insert<T>(datasource: TypeId, v: &'a T) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = 1;
        v.arguments(&mut t.query.arguments);
        t
    }

    pub(crate) fn insert_bulk<T>(datasource: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = v.len();
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    pub(crate) fn insert_ignore<T>(datasource: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = v.len();
        t.replace = Some(("INSERT".into(), "INSERT IGNORE".into()));
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    pub(crate) fn insert_replace<T>(datasource: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = v.len();
        t.replace = Some(("INSERT INTO".into(), "REPLACE INTO".into()));
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    fn build_sql(&mut self) -> Result<String> {
        let holders = vec!["?"; self.columns.len()];
        self.query.sql_builder.fields(self.columns.as_slice());
        (0..self.size).for_each(|_| {
            self.query.sql_builder.values(holders.as_slice());
        });
        let mut sql = self.query.sql_builder.sql()?;
        if let Some((src, target)) = &self.replace {
            sql = sql.replacen(src.as_str(), target.as_str(), 1);
        }
        Ok(sql)
    }
}

impl <'a>crate::statement::Statement<'a> for Insert<'a> {

    fn query(&'a mut self) -> (&'a mut Insert<'a>, &'a mut QueryBuilder<'a>){
        (self,self.query.borrow_mut())
    }

    fn datasource(&'a self) -> TypeId {
        self.query.datasource
    }

    fn arguments(mut self) -> Arguments<'a> {
        self.query.arguments
    }

    fn build_sql<'s:'a>(&'s  mut self) -> crate::types::Result<String> {
        let (self2,query) = self.query();
        Ok(query.sql_builder.sql()?)
    }
}

impl <'a>crate::statement::Execute<'a> for Insert<'a> {}