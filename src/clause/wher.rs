use crate::statement::Statement;
use crate::sqlx::{Encode, Type};
use crate::types::Database;
pub(crate) trait Where<'a>: Statement<'a>
{
    type Statement:Sized;
    fn and_where_eq<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
            Self: Sized
    {
        let (self2,query) = self.query();
        query.and_where_eq(f, v);
        self2
    }

    fn and_where_ne<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_ne(f, v);
        self2
    }

    fn and_where_ge<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_ge(f, v);
        self2
    }

    fn and_where_le<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_le(f, v);
        self2
    }

    fn and_where_gt<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_gt(f, v);
        self2
    }

    fn and_where_lt<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_lt(f, v);
        self2
    }

    fn and_where_is_null<S>(&'a mut self, f: S) -> &'a mut Self
        where
            S: ToString,
    {
        let (self2,query) = self.query();
        query.and_where_is_null(f);
        self2
    }

    fn and_where_is_not_null<S>(&'a mut self, f: S) -> &'a mut Self
        where
            S: ToString,
    {
        let (self2,query) = self.query();
        query.and_where_is_not_null(f);
        self2
    }

    fn and_where_between<S, V>(&'a mut self, f: S, min: V, max: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_between(f, min, max);
        self2
    }

    fn and_where_between_options<S, V>(&'a mut self, f: S, min: Option<V>, max: Option<V>) -> &'a mut Self
    where
        S: ToString,
        V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_between_options(f, min, max);
        self2
    }

    fn and_where_not_between<S, V>(&'a mut self, f: S, min: V, max: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_not_between(f, min, max);
        self2
    }

    fn and_where_in<S, V>(&'a mut self, f: S, v: &'a [V]) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_in(f, v);
        self2
    }

    fn and_where_not_in<S, V>(&'a mut self, f: S, v: &'a [V]) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a,
    {
        let (self2,query) = self.query();
        query.and_where_not_in(f, v);
        self2
    }



    // ***********************************************************************

    fn or_where_eq<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_eq(f, v);
        self2
    }

    fn or_where_ne<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_ne(f, v);
        self2
    }

    fn or_where_ge<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_ge(f, v);
        self2
    }

    fn or_where_le<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_le(f, v);
        self2
    }

    fn or_where_gt<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_gt(f, v);
        self2
    }

    fn or_where_lt<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_lt(f, v);
        self2
    }

    fn or_where_is_null<S>(&'a mut self, f: S) -> &'a mut Self
        where
            S: ToString,
    {
        let (self2,query) = self.query();
        query.or_where_is_null(f);
        self2
    }

    fn or_where_is_not_null<S, V>(&'a mut self, f: S) -> &'a mut Self
        where
            S: ToString,
    {
        let (self2,query) = self.query();
        query.or_where_is_not_null(f);
        self2
    }

    fn or_where_between<S, V>(&'a mut self, f: S, min: V, max: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_between(f, min, max);
        self2
    }

    fn or_where_not_between<S, V>(&'a mut self, f: S, min: V, max: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_not_between(f, min, max);
        self2
    }

    fn or_where_in<S, V>(&'a mut self, f: S, v: &'a [V]) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_in(f, v);
        self2
    }

    fn or_where_not_in<S, V>(&'a mut self, f: S, v: &'a [V]) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a,
    {
        let (self2,query) = self.query();
        query.or_where_not_in(f, v);
        self2
    }
}