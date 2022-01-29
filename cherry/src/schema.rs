use crate::types::{Arguments, Result, Row};

pub trait Schema: Sized + Send + Unpin {
    /// Return database table name
    fn table() -> &'static str;

    /// Return table's column names
    fn columns() -> Vec<&'static str>;

    /// Add each column's value as argument from appropriate field
    fn arguments<'a>(&'a self, arguments: &mut Arguments<'a>);

    /// Process row into data structure
    fn from_row(row: &Row) -> Result<Self>;
}
