pub(crate) mod insert;
pub(crate) mod insert_update;
pub(crate) mod update;
pub(crate) mod delete;
pub(crate) mod select;
pub(crate) mod execute;

pub(crate) use insert::Insert;
pub(crate) use insert_update::InsertUpdate;
pub(crate) use update::Update;
pub(crate) use delete::Delete;
pub(crate) use select::Select;
pub(crate) use execute::Execute;


use crate::query::builder::QueryBuilder;
use async_trait::async_trait;


#[async_trait]
pub(crate) trait Statement<'a>
{
    fn query(&'a mut self) -> &'a mut QueryBuilder<'a>;
    fn build_sql(&mut self) -> crate::types::Result<String> {
        Ok(self.query().sql_builder.sql()?)
    }
}