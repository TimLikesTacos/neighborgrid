use crate::Grid;

/// Determines where (0,0) is on the grid.  Care should be taken with `Center`, especially with even number of rows or columns, for example:
/// ```
/// use neighborgrid::*;
///
/// let input = vec![
///                 vec![1, 2, 3, 4],
///                 vec![5, 6, 7, 8],
///                 vec![9, 10, 11 ,12],
///                 vec![13, 14, 15, 16]
///                 ];
/// let options = GridOptions{
///        origin: Origin::Center,
///        inverted_y: false,
///        ..GridOptions::default()
/// };
/// let grid = Grid::new(input.clone(), Some(options)).unwrap();
/// assert_eq!(grid.get((0,0)), Some(&11));
///
/// let options = GridOptions{
///        origin: Origin::Center,
///        inverted_y: true,
///        ..GridOptions::default()
/// };
/// let grid = Grid::new(input, Some(options)).unwrap();
/// assert_eq!(grid.get((0,0)), Some(&11));
/// ```
///
/// In the above example, for `Origin::UpperLeft`, `(0,0)` would be the cell with a `1`, or a `13` for `Origin::LowerLeft`  
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
            Origin::Center => -(grid.cols as isize / 2),
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
            Origin::Center => -(grid.rows as isize / 2),
            Origin::LowerLeft => 0,
            Origin::UpperLeft => -(grid.rows as isize),
        }
    }
}
