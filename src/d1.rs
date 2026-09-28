use crate::CardOf;
use orx_col_dim::D1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CardD1(usize);

impl CardOf<D1> for CardD1 {}
