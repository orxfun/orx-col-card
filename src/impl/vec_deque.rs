use crate::Card;
use alloc::collections::VecDeque;
use orx_col_dim::{D1, D2, D3, D4, Dim};
use orx_col_dim::{DescendentIdxD1, DescendentIdxD2, DescendentIdxD3, DescendentIdxD4};

// d1

impl<T> Card<D1> for VecDeque<T> {
    fn card(&self, idx: impl Into<<D1 as Dim>::DescendentIdx>) -> usize {
        match idx.into() {
            DescendentIdxD1::Child0([]) => self.len(),
        }
    }

    fn try_card(&self, idx: impl Into<<D1 as Dim>::DescendentIdx>) -> Option<usize> {
        match idx.into() {
            DescendentIdxD1::Child0([]) => Some(self.len()),
        }
    }
}

// d2

impl<C1> Card<D2> for VecDeque<C1>
where
    C1: Card<D1>,
{
    fn card(&self, idx: impl Into<<D2 as Dim>::DescendentIdx>) -> usize {
        match idx.into() {
            DescendentIdxD2::Child0([]) => self.len(),
            DescendentIdxD2::Child1([i]) => self[i].card([]),
        }
    }

    fn try_card(&self, idx: impl Into<<D2 as Dim>::DescendentIdx>) -> Option<usize> {
        match idx.into() {
            DescendentIdxD2::Child0([]) => Some(self.len()),
            DescendentIdxD2::Child1([i]) => self.get(i).and_then(|x| x.try_card([])),
        }
    }
}

// d3

impl<C2> Card<D3> for VecDeque<C2>
where
    C2: Card<D2>,
{
    fn card(&self, idx: impl Into<<D3 as Dim>::DescendentIdx>) -> usize {
        match idx.into() {
            DescendentIdxD3::Child0([]) => self.len(),
            DescendentIdxD3::Child1([i]) => self[i].card([]),
            DescendentIdxD3::Child2([i, j]) => self[i].card([j]),
        }
    }

    fn try_card(&self, idx: impl Into<<D3 as Dim>::DescendentIdx>) -> Option<usize> {
        match idx.into() {
            DescendentIdxD3::Child0([]) => Some(self.len()),
            DescendentIdxD3::Child1([i]) => self.get(i).and_then(|x| x.try_card([])),
            DescendentIdxD3::Child2([i, j]) => self.get(i).and_then(|x| x.try_card([j])),
        }
    }
}

// d4

impl<C3> Card<D4> for VecDeque<C3>
where
    C3: Card<D3>,
{
    fn card(&self, idx: impl Into<<D4 as Dim>::DescendentIdx>) -> usize {
        match idx.into() {
            DescendentIdxD4::Child0([]) => self.len(),
            DescendentIdxD4::Child1([i]) => self[i].card([]),
            DescendentIdxD4::Child2([i, j]) => self[i].card([j]),
            DescendentIdxD4::Child3([i, j, k]) => self[i].card([j, k]),
        }
    }

    fn try_card(&self, idx: impl Into<<D4 as Dim>::DescendentIdx>) -> Option<usize> {
        match idx.into() {
            DescendentIdxD4::Child0([]) => Some(self.len()),
            DescendentIdxD4::Child1([i]) => self.get(i).and_then(|x| x.try_card([])),
            DescendentIdxD4::Child2([i, j]) => self.get(i).and_then(|x| x.try_card([j])),
            DescendentIdxD4::Child3([i, j, k]) => self.get(i).and_then(|x| x.try_card([j, k])),
        }
    }
}
