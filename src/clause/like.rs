use crate::query::builder::QueryBuilder;
use crate::statement::Statement;
use crate::sqlx::{Encode, Type};
use crate::types::Database;

pub(crate) trait Like<'a>: Statement<'a>
{
    type Statement;
    fn and_where_like<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.query().and_where_like(f, v);
        self
    }

    fn and_where_not_like<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.query().and_where_not_like(f, v);
        self
    }
}