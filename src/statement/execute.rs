use crate::connection;
use crate::types::{QueryResult, Transaction};
use anyhow::Result;
use crate::query::builder::QueryBuilder;
use crate::statement::Statement;
use async_trait::async_trait;


#[async_trait]
pub(crate) trait Execute<'a>: Statement<'a>
{
    async fn execute<'s:'a>(&'s mut self) -> Result<QueryResult> {
        let datasource =  self.datasource();
        let pool = connection::get(datasource)?;
        let result = sqlx::query_with(self.build_sql()?.as_str(), self.arguments())
            .execute(pool).await?;
        Ok(QueryResult::from(result))
    }

    async fn execute_tx<'s:'a>(&'s mut self) -> Result<QueryResult> {
        let datasource =  self.datasource();
        let mut tx = connection::get(datasource)?.begin().await?;
        let result = sqlx::query_with(self.build_sql()?.as_str(), self.arguments())
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(QueryResult::from(result))
    }

    async fn execute_with<'s:'a>(&'s mut self, tx: &mut Transaction) -> Result<QueryResult> {
        let result = sqlx::query_with(self.build_sql()?.as_str(), self.arguments())
            .execute(tx).await?;
        Ok(QueryResult::from(result))
    }
}