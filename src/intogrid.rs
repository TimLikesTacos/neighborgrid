use crate::error::GridError;
use crate::grid::Grid;

pub trait IntoGrid<T> {
    fn into_grid(self) -> Result<Grid<T>, GridError>;
}

impl<T> IntoGrid<T> for Vec<Vec<T>> {
    fn into_grid(self) -> Result<Grid<T>, GridError> {
        let rows = self.len();
        let mut cols = 0;
        let mut total = 0;
        if let Some(first) = self.get(0) {
            cols = first.len();

            total = row_col_length_check(rows, cols)?;
            if rows == 0 || cols == 0 {
                return Err(GridError::InvalidSize);
            }
        } else {
            return Err(GridError::InvalidSize);
        }

        let mut grid = Vec::with_capacity(total);
        for mut row in self.into_iter() {
            if row.len() != cols {
                return Err(GridError::RowSizeMismatch);
            }
            grid.append(&mut row);
        }
        Ok(Grid::create(grid, rows, cols, None))
    }
}

impl<T: Clone> IntoGrid<T> for (&Vec<T>, usize) {
    fn into_grid(self) -> Result<Grid<T>, GridError> {
        _convert1d(self)
    }
}

impl<T: Clone> IntoGrid<T> for (Vec<T>, usize) {
    fn into_grid(self) -> Result<Grid<T>, GridError> {
        _convert1d((&self.0, self.1))
    }
}

/// isize::MAX is the max size for a vec.  Checks that excessive amount will not be allocated and panic.
fn row_col_length_check(rows: usize, cols: usize) -> Result<usize, GridError> {
    if rows >= i32::MAX as usize || cols >= i32::MAX as usize {
        return Err(GridError::ExcessiveSize);
    }
    let size = rows.checked_mul(cols).ok_or(GridError::ExcessiveSize)?;
    if size >= i32::MAX as usize {
        Err(GridError::ExcessiveSize)
    } else {
        Ok(size)
    }
}

fn _convert1d<T: Clone>(items: (&Vec<T>, usize)) -> Result<Grid<T>, GridError> {
    let total = row_col_length_check(items.1, items.0.len())?;
    let cols = items.0.len();
    if cols == 0 || items.1 <= 0 {
        return Err(GridError::InvalidSize);
    }
    let mut vec = Vec::with_capacity(total);
    for _ in 0..items.1 {
        vec.append(&mut items.0.clone())
    }
    Ok(Grid::create(vec, cols, items.1, None))
}

#[cfg(test)]
mod grid_tests {
    use super::*;
    type Result<T> = std::result::Result<T, GridError>;

    mod two_d_vec {
        use super::*;

        fn simple2d() -> Vec<Vec<i32>> {
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        }

        #[test]
        fn should_create_new_from_2d_vec() -> Result<()> {
            let vec = simple2d();
            let grid = vec.into_grid()?;
            assert_eq!(grid.size(), 9);
            let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            assert_eq!(expected, grid.items);
            Ok(())
        }

        #[test]
        fn should_error_on_uneven_rows() -> () {
            let mut vec = simple2d();
            vec[2].push(10);
            let grid = vec.into_grid();
            assert!(matches!(grid, Err(GridError::RowSizeMismatch)));
        }

        #[test]
        fn should_error_on_empty_vec() {
            let vec: Vec<Vec<i32>> = vec![];
            let grid = vec.into_grid();
            assert!(matches!(grid, Err(GridError::InvalidSize)), "{:?}", grid);
        }

        #[test]
        fn should_error_on_empty_row_vec() {
            let vec: Vec<Vec<i32>> = vec![vec![], vec![]];
            let grid = vec.into_grid();
            assert!(matches!(grid, Err(GridError::InvalidSize)));
        }
    }

    mod one_d_vec {
        use super::*;

        fn basic_input() -> (Vec<i32>, usize) {
            (vec![1, 2, 3], 4usize)
        }

        #[test]
        fn should_create_grid() -> Result<()> {
            let grid = basic_input().into_grid()?;
            assert_eq!(grid.size(), 12);
            let expected = vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3];
            assert_eq!(expected, grid.items);
            Ok(())
        }

        #[test]
        fn should_create_grid_from_vec_reference() -> Result<()> {
            let grid = (&vec![1, 2, 3], 4usize).into_grid()?;
            assert_eq!(grid.size(), 12);
            let expected = vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3];
            assert_eq!(expected, grid.items);
            Ok(())
        }

        #[test]
        fn should_error_on_empty_vec() {
            let grid: Result<Grid<i32>> = (vec![], 5).into_grid();
            assert!(matches!(grid, Err(GridError::InvalidSize)));
        }

        #[test]
        fn should_error_on_no_rows() {
            let grid = (vec![1, 2, 3], 0).into_grid();
            assert!(matches!(grid, Err(GridError::InvalidSize)));
        }
    }
}
