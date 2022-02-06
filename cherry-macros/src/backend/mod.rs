use std::borrow::Cow;

use proc_macro2::TokenStream;

use crate::patch::Patch;
use crate::table::Table;
use crate::schema::Schema;

mod common;
#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgres")]
mod postgres;

#[cfg(feature = "mysql")]
pub type Implementation = mysql::MySqlBackend;
#[cfg(feature = "postgres")]
pub type Implementation = postgres::PgBackend;
#[cfg(feature = "sqlite")]
compile_error!("sqlite is currently not supported");

pub trait Backend: Sized + Clone {
    const QUOTE: char;
    /// TODO: benchmark HashSet vs linear search
    const RESERVED_IDENTS: &'static [&'static str];

    type Bindings: Iterator<Item = Cow<'static, str>> + Default;

    fn impl_primary_key(table: &Table<Self>) -> TokenStream {
        common::primary_key::<Self>(table)
    }

    /// Generate an `impl <Table>` block, containing getter methods
    fn impl_getters(table: &Table<Self>) -> TokenStream {
        common::getters::<Self>(table)
    }

    /// Generate an `impl <Table>` block, containing setter methods
    fn impl_setters(table: &Table<Self>) -> TokenStream {
        common::setters::<Self>(table)
    }

    /// Generate an `impl Table for <Table>` block
    fn impl_table(table: &Table<Self>) -> TokenStream {
        common::impl_table::<Self>(table)
    }

    /// Generate an `impl IdTable for <IdTable>` block
    fn impl_id_table(table: &Table<Self>) -> TokenStream {
        common::impl_id_table::<Self>(table)
    }

    /// Generate an `impl Table for <Table>` block
    fn impl_schema(table: &Schema<Self>) -> TokenStream {
        common::impl_schema::<Self>(table)
    }

    /// Generate an `impl cherry::Schema for <Query>` block
    #[cfg(feature = "query")]
    fn impl_schema(table: &Table<Self>) -> TokenStream {
        common::schema::impl_schema::<Self>(table)
    }

    /// Implement [Insert] for the helper struct for inserting
    fn impl_insert(table: &Table<Self>) -> TokenStream;

    /// Generate a helper struct for inserting
    fn insert_struct(table: &Table<Self>) -> TokenStream {
        common::insert_struct(table)
    }

    /// Implement [Patch]
    fn impl_patch(patch: &Patch) -> TokenStream {
        common::impl_patch::<Self>(patch)
    }
}
