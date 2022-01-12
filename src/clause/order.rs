use crate::query::builder::QueryBuilder;
use crate::sqlx::{Encode, Type};
use crate::types::Database;
use crate::statement::Statement;

pub(crate) trait Order<'a>: Statement<'a>
{
    type Statement;

    fn order_by<V>(&mut self, v: V, desc: bool) -> &mut Self
    where
        V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.query().order_by(v, desc);
        self
    }
}