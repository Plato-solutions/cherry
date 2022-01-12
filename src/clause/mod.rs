pub(crate) mod wher;
pub(crate) mod like;
pub(crate) mod order;
pub(crate) mod group;
pub(crate) mod having;
pub(crate) mod distinct;

pub(crate) use wher::Where;
pub(crate) use like::Like;
pub(crate) use order::Order;
pub(crate) trait Clause<'a>: wher::Where<'a> + like::Like<'a> + order::Order<'a> {}