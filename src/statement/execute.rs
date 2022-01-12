use crate::connection;
use crate::types::{QueryResult, Transaction};
use anyhow::Result;
use crate::query::builder::QueryBuilder;
use crate::statement::Statement;
use async_trait::async_trait;


#[async_trait]
pub(crate) trait Execute<'a>: Statement<'a>
{
    async fn execute(&mut self) -> Result<QueryResult> {
        let pool = connection::get(self.query().datasource)?;
        let result = sqlx::query_with(self.build_sql()?.as_str(), self.query().arguments)
            .execute(pool).await?;
        Ok(QueryResult::from(result))
    }

    async fn execute_tx(&mut self) -> Result<QueryResult> {
        let mut tx = connection::get(self.query().datasource)?.begin().await?;
        let result = sqlx::query_with(self.build_sql()?.as_str(), self.query().arguments)
            .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(QueryResult::from(result))
    }

    async fn execute_with(&mut self, tx: &mut Transaction) -> Result<QueryResult> {
        let result = sqlx::query_with(self.build_sql()?.as_str(), self.query().arguments)
            .execute(tx).await?;
        Ok(QueryResult::from(result))
    }
}