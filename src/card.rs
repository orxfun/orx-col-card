use orx_col_dim::Dim;

pub trait Card<D: Dim> {
    fn card(&self, idx: impl Into<D::DescendentIdx>) -> usize;

    fn try_card(&self, idx: impl Into<D::DescendentIdx>) -> Option<usize>;
}

// forward ref

impl<D: Dim, C: Card<D>> Card<D> for &C {
    fn card(&self, idx: impl Into<D::DescendentIdx>) -> usize {
        <C as Card<D>>::card(self, idx)
    }

    fn try_card(&self, idx: impl Into<D::DescendentIdx>) -> Option<usize> {
        <C as Card<D>>::try_card(self, idx)
    }
}

// forward mut

impl<D: Dim, C: Card<D>> Card<D> for &mut C {
    fn card(&self, idx: impl Into<D::DescendentIdx>) -> usize {
        <C as Card<D>>::card(self, idx)
    }

    fn try_card(&self, idx: impl Into<D::DescendentIdx>) -> Option<usize> {
        <C as Card<D>>::try_card(self, idx)
    }
}
