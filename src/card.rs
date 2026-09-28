use orx_col_dim::Dim;

/// A trait representing the cardinality (length or size) of multi-dimensional collections.
///
/// `Card<D>` abstracts over collection structures of dimension `D` (`D1`, `D2`, `D3`, `D4`),
/// allowing querying the size of the root collection or any of its nested descendants.
pub trait Card<D: Dim> {
    /// Returns the cardinality (length or number of elements) at the specified descendant index.
    ///
    /// The descendant index can specify:
    /// - Root level: `[]`
    /// - 1st descendant: `i` or `[i]`
    /// - 2nd descendant: `[i, j]`
    /// - 3rd descendant: `[i, j, k]`
    ///
    /// # Panics
    ///
    /// Panics if any component of the provided descendant index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_col_card::*;
    ///
    /// // 1D Collection
    /// let vec = vec![10, 20, 30];
    /// assert_eq!(vec.card([]), 3);
    ///
    /// // 2D Collection
    /// let matrix: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4, 5]];
    /// assert_eq!(Card::<D2>::card(&matrix, []), 2);     // number of rows
    /// assert_eq!(Card::<D2>::card(&matrix, 0), 2);      // length of 1st row
    /// assert_eq!(Card::<D2>::card(&matrix, [1]), 3);    // length of 2nd row
    /// ```
    fn card(&self, idx: impl Into<D::DescendentIdx>) -> usize;

    /// Returns the cardinality (length or number of elements) at the specified descendant index,
    /// or `None` if any component of the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_col_card::*;
    ///
    /// let matrix: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4, 5]];
    /// assert_eq!(Card::<D2>::try_card(&matrix, []), Some(2));
    /// assert_eq!(Card::<D2>::try_card(&matrix, 0), Some(2));
    /// assert_eq!(Card::<D2>::try_card(&matrix, [1]), Some(3));
    /// assert_eq!(Card::<D2>::try_card(&matrix, 2), None);
    /// assert_eq!(Card::<D2>::try_card(&matrix, [5]), None);
    /// ```
    fn try_card(&self, idx: impl Into<D::DescendentIdx>) -> Option<usize>;
}

// forward ref

impl<D: Dim, C: Card<D>> Card<D> for &C {
    #[inline(always)]
    fn card(&self, idx: impl Into<D::DescendentIdx>) -> usize {
        <C as Card<D>>::card(self, idx)
    }

    #[inline(always)]
    fn try_card(&self, idx: impl Into<D::DescendentIdx>) -> Option<usize> {
        <C as Card<D>>::try_card(self, idx)
    }
}

// forward mut

impl<D: Dim, C: Card<D>> Card<D> for &mut C {
    #[inline(always)]
    fn card(&self, idx: impl Into<D::DescendentIdx>) -> usize {
        <C as Card<D>>::card(self, idx)
    }

    #[inline(always)]
    fn try_card(&self, idx: impl Into<D::DescendentIdx>) -> Option<usize> {
        <C as Card<D>>::try_card(self, idx)
    }
}
