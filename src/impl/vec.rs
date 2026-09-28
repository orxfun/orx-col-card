use crate::Card;
use alloc::vec::Vec;
use orx_col_dim::{D1, D2, Dim};
use orx_col_dim::{DescendentIdxD1, DescendentIdxD2};

// d1

impl<T> Card<D1> for Vec<T> {
    fn card(&self, idx: <D1 as Dim>::DescendentIdx) -> usize {
        match idx {
            DescendentIdxD1::Child0([]) => self.len(),
        }
    }
}

// d2

impl<C1> Card<D2> for Vec<C1>
where
    C1: Card<D1>,
{
    fn card(&self, idx: <D2 as Dim>::DescendentIdx) -> usize {
        match idx {
            DescendentIdxD2::Child0([]) => self.len(),
            // DescendentIdxD2::Child1([i]) => self[i].card([]),
            _ => todo!(),
        }
    }
}
