use std::any::TypeId;
use std::borrow::BorrowMut;

use anyhow::anyhow;
use sql_builder::SqlBuilder;
use sqlx::{Encode, Type};

use crate::{Cherry, connection, query, clause};
use crate::query::builder::QueryBuilder;
use crate::types::{Database, QueryResult, Result, Transaction};

pub struct InsertUpdate<'a> {
    pub(crate) query: QueryBuilder<'a>,
    pub(crate) columns: Vec<&'static str>,
    pub(crate) size: usize,
    pub(crate) fields: Vec<String>,
}

impl<'a> InsertUpdate<'a> {

    fn new<T>(datasource: TypeId) -> Self where T: Cherry {
        Self {
            query: QueryBuilder::new::<T>(datasource, SqlBuilder::insert_into(T::table())),
            columns: T::columns(),
            size: 0,
            fields: vec![]
        }
    }

    pub(crate) fn insert_update<T>(datasource: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = v.len();
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    pub fn field<T: AsRef<str>>(mut self, f: T) -> Self {
        self.field_ref(f);
        self
    }

    pub fn field_ref<T: AsRef<str>>(&mut self, f: T) -> &Self {
        self.fields.push(f.as_ref().to_owned());
        self
    }

    pub fn fields<T: AsRef<str>>(mut self, f: &[T]) -> Self {
        self.fields_ref(f);
        self
    }

    pub fn fields_ref<T: AsRef<str>>(&mut self, f: &[T]) -> &Self {
        f.iter().for_each(|f| {
            self.fields.push(f.as_ref().to_owned());
        });
        self
    }

    fn build_sql(&mut self) -> Result<String> {
        if self.fields.is_empty() {
            return Err(anyhow!("Empty update fields."));
        }

        let holders = vec!["?"; self.columns.len()];
        self.query.sql_builder.fields(self.columns.as_slice());
        (0..self.size).for_each(|_| {
            self.query.sql_builder.values(holders.as_slice());
        });

        let insert = self.query.sql_builder.sql()?.strip_suffix(";")
            .ok_or(anyhow!("Empty sql. This wasn’t supposed to happen."))?
            .to_owned();
        let update = self.fields.iter().map(|x| format!("{0} = new.{0}, ", x))
            .collect::<String>()
            .strip_suffix(",")
            .ok_or(anyhow!("Empty sql. This wasn’t supposed to happen."))?
            .to_owned();
        Ok(format!("{} AS new ON DUPLICATE KEY UPDATE {};", insert, update))
    }
}

impl <'a> crate::statement::Statement<'a> for InsertUpdate<'a> {
    fn query(&'a mut self) -> &'a mut QueryBuilder<'a> {
        &mut self.query
    }
}

impl <'a> crate::statement::Execute<'a> for InsertUpdate<'a> {}

impl <'a> crate::clause::Where<'a> for InsertUpdate<'a> {
    type Statement = InsertUpdate<'a>;
}

impl <'a> crate::clause::Like<'a> for InsertUpdate<'a> {
    type Statement = InsertUpdate<'a>;
}

impl <'a> crate::clause::Order<'a> for InsertUpdate<'a> {
    type Statement = InsertUpdate<'a>;
}
