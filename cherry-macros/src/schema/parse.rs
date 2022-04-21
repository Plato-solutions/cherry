use std::convert::TryFrom;

use proc_macro2::Span;
use syn::{Data, DeriveInput, Error, Ident, Result};

use crate::attrs::{parse_attrs, Queryable, TableAttr};
use crate::utils::{missing_attr, set_once};
use crate::table::TableField;
use super::{Schema};
use crate::backend::Backend;

macro_rules! none {
    ($($i:ident),*) => { $( let mut $i = None; )* };
}

impl<B: Backend> TryFrom<&syn::DeriveInput> for Schema<B> {
    type Error = Error;

    fn try_from(value: &DeriveInput) -> Result<Self> {
        let data = match &value.data {
            Data::Struct(s) => s,
            _ => panic!("not a struct with named fields"),
        };

        let fields = data
            .fields
            .iter()
            .map(TableField::try_from)
            .collect::<Result<Vec<_>>>()?;

        // fields.retain(|field| !field.unmapped);

        none!(table, datasource, queryable);
        let (attrs, _other_attrs) = parse_attrs::<TableAttr>(&value.attrs)?;
        for attr in attrs {
            match attr {
                TableAttr::Table(x) => set_once(&mut table, x)?,
                TableAttr::Datasource(x) => set_once(&mut datasource, x)?,
                TableAttr::Queryable(x) => {
                    let default = || Queryable {
                        attrs: vec![],
                        ident: Ident::new(&format!("Query{}", value.ident), Span::call_site()),
                    };
                    set_once(&mut queryable, x.unwrap_or_else(default))?;
                }
                _ => {}
            }
        }

        let datasource = datasource.ok_or_else(|| missing_attr("datasource"))?;


        //@TODO add any checks for query settings without queryable

        Ok(Schema {
            ident: value.ident.clone(),
            vis: value.vis.clone(),
            table: table.ok_or_else(|| missing_attr("table"))?,
            queryable,
            fields,
            datasource,
        })
    }
}
