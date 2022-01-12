#![cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
extern crate proc_macro;

mod datasource;
mod attrs;
mod backend;
mod utils;

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(DataSource, attributes(cherry))]
pub fn derive_datasource(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match table::derive(input) {
        Ok(ok) => ok,
        Err(err) => err.to_compile_error(),
    }
        .into()
}