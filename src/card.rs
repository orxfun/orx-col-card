use orx_col_dim::Dim;

pub trait Card<D: Dim> {
    fn card(&self, idx: impl Into<D::DescendentIdx>) -> usize;

    fn try_card(&self, idx: impl Into<D::DescendentIdx>) -> Option<usize>;
}
