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
        self.query().and_where_eq(f, v);
        self
    }

    fn and_where_ne<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().and_where_ne(f, v);
        self
    }

    fn and_where_ge<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().and_where_ge(f, v);
        self
    }

    fn and_where_le<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().and_where_le(f, v);
        self
    }

    fn and_where_gt<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().and_where_gt(f, v);
        self
    }

    fn and_where_lt<S, V>(&'a mut self, f: S, v: V) -> &'a mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().and_where_lt(f, v);
        self
    }

    fn and_where_is_null<S>(&'a mut self, f: S) -> &'a mut Self
        where
            S: ToString,
    {
        self.query().and_where_is_null(f);
        self
    }

    fn and_where_is_not_null<S>(&mut self, f: S) -> &mut Self
        where
            S: ToString,
    {
        self.query().and_where_is_not_null(f);
        self
    }

    fn and_where_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().and_where_between(f, min, max);
        self
    }

    fn and_where_between_options<S, V>(&mut self, f: S, min: Option<V>, max: Option<V>) -> &mut Self
    where
        S: ToString,
        V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().and_where_between_options(f, min, max);
        self
    }

    fn and_where_not_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().and_where_not_between(f, min, max);
        self
    }

    fn and_where_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a,
    {
        self.query().and_where_in(f, v);
        self
    }

    fn and_where_not_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a,
    {
        self.query().and_where_not_in(f, v);
        self
    }



    // ***********************************************************************

    fn or_where_eq<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().or_where_eq(f, v);
        self
    }

    fn or_where_ne<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().or_where_ne(f, v);
        self
    }

    fn or_where_ge<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().or_where_ge(f, v);
        self
    }

    fn or_where_le<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().or_where_le(f, v);
        self
    }

    fn or_where_gt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().or_where_gt(f, v);
        self
    }

    fn or_where_lt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().or_where_lt(f, v);
        self
    }

    fn or_where_is_null<S>(&mut self, f: S) -> &mut Self
        where
            S: ToString,
    {
        self.query().or_where_is_null(f);
        self
    }

    fn or_where_is_not_null<S, V>(&mut self, f: S) -> &mut Self
        where
            S: ToString,
    {
        self.query().or_where_is_not_null(f);
        self
    }

    fn or_where_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().or_where_between(f, min, max);
        self
    }

    fn or_where_not_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a,
    {
        self.query().or_where_not_between(f, min, max);
        self
    }

    fn or_where_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a,
    {
        self.query().or_where_in(f, v);
        self
    }

    fn or_where_not_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a,
    {
        self.query().or_where_not_in(f, v);
        self
    }
}