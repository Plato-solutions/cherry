pub(crate) mod insert;
pub(crate) mod insert_update;
pub(crate) mod update;
pub(crate) mod delete;
pub(crate) mod select;
pub(crate) mod execute;

use std::any::TypeId;
pub(crate) use insert::Insert;
pub(crate) use insert_update::InsertUpdate;
pub(crate) use update::Update;
pub(crate) use delete::Delete;
pub(crate) use select::Select;
pub(crate) use execute::Execute;


use crate::query::builder::QueryBuilder;
use async_trait::async_trait;
use crate::types::Arguments;


#[async_trait]
pub(crate) trait Statement<'a>
{
    fn query(&'a mut self) -> (&'a mut Self,&'a mut QueryBuilder<'a>);
    fn datasource(&'a self) -> TypeId;
    fn arguments(self) -> Arguments<'a>;
    fn build_sql<'s:'a>(&'s  mut self) -> crate::types::Result<String>;
}
