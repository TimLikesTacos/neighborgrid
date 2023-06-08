use crate::Grid;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Origin {
    #[default]
    UpperLeft,
    Center,
    LowerLeft,
}

/// Assumptions is that the grid cannot be larger than isize::MAX, which is a fair assumption since the largest Vec in stdlib is isize::MAX
impl Origin {
    #[inline]
    pub(crate) fn max_x<T>(&self, grid: &Grid<T>) -> isize {
        match self {
            Origin::Center => (grid.cols as isize - 1) / 2,
            Origin::LowerLeft | Origin::UpperLeft => grid.cols as isize,
        }
    }

    #[inline]
    pub(crate) fn min_x<T>(&self, grid: &Grid<T>) -> isize {
        match self {
            Origin::Center => (grid.cols as isize / 2) * -1,
            Origin::LowerLeft | Origin::UpperLeft => 0,
        }
    }

    #[inline]
    pub(crate) fn max_y<T>(&self, grid: &Grid<T>) -> isize {
        match self {
            Origin::Center => (grid.rows + 1) as isize / 2,
            Origin::LowerLeft => grid.rows as isize,
            Origin::UpperLeft => 0,
        }
    }

    #[inline]
    pub(crate) fn min_y<T>(&self, grid: &Grid<T>) -> isize {
        match self {
            Origin::Center => (grid.rows as isize / 2) * -1,
            Origin::LowerLeft => 0,
            Origin::UpperLeft => -1 * grid.rows as isize,
        }
    }
}
