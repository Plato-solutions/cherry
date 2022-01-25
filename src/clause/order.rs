use crate::query::builder::QueryBuilder;
use crate::sqlx::{Encode, Type};
use crate::types::Database;
use crate::statement::Statement;

pub(crate) trait Order<'a>: Statement<'a>
{
    type Statement;

    fn order_by<V>(&'a mut self, v: V, desc: bool) -> &'a mut Self
    where
        V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        let (self2,query) = self.query();
        query.order_by(v, desc);
        self2
    }
}