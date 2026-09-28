use crate::Card;
use alloc::vec::Vec;
use orx_col_dim::{D1, DescendentIdxD1, Dim};

impl<T> Card<D1> for Vec<T> {
    fn card(&self, idx: <D1 as Dim>::DescendentIdx) -> usize {
        match idx {
            DescendentIdxD1::Child0([]) => self.len(),
        }
    }
}
