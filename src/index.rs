use crate::error::GridError;
use crate::grid::{Grid, Origin};

pub trait Index {
    fn grid_index<T>(self, grid: &Grid<T>) -> Result<usize, GridError>;
    fn output<T>(index: usize, grid: &Grid<T>) -> Self;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl Index for Coordinates {
    fn grid_index<T>(self, grid: &Grid<T>) -> Result<usize, GridError> {
        let y = invert_y(grid, self.y);
        bounds_check(grid, self.x, y)?;
        Ok(xy_to_index(grid, self.x, y))
    }

    fn output<T>(index: usize, grid: &Grid<T>) -> Self {
        let (x, y) = (index % grid.cols, index / grid.cols);
        let (x, y) = adjust_to_origin(grid, x as isize, y as isize);
        let y = invert_y(grid, y);
        Coordinates { x, y }
    }
}

impl Index for usize {
    fn grid_index<T>(self, grid: &Grid<T>) -> Result<usize, GridError> {
        if self < grid.size() {
            Ok(self)
        } else {
            Err(GridError::IndexOutOfBounds)
        }
    }

    fn output<T>(index: usize, _grid: &Grid<T>) -> Self {
        index
    }
}

impl Index for (isize, isize) {
    fn grid_index<T>(self, grid: &Grid<T>) -> Result<usize, GridError> {
        let y = invert_y(grid, self.1);
        bounds_check(grid, self.0, y)?;
        Ok(xy_to_index(grid, self.0, y))
    }

    fn output<T>(index: usize, grid: &Grid<T>) -> Self {
        let (x, y) = (index % grid.cols, index / grid.cols);
        let (x, y) = adjust_to_origin(grid, x as isize, y as isize);
        let y = invert_y(grid, y);
        (x, y)
    }
}

impl<S: Index + Clone> Index for &S {
    fn grid_index<T>(self, grid: &Grid<T>) -> Result<usize, GridError> {
        S::grid_index(self.clone(), grid)
    }

    fn output<T>(_: usize, grid: &Grid<T>) -> Self {
        todo!()
    }
}

fn invert_y<T>(grid: &Grid<T>, y: isize) -> isize {
    if let Some(options) = &grid.options {
        if options.inverted_y {
            -y
        } else {
            y
        }
    } else {
        y
    }
}

fn bounds_check<T>(grid: &Grid<T>, x: isize, y: isize) -> Result<(), GridError> {
    let abs = |v: isize| v.abs() as usize;

    let maxlimit = abs(x) < grid.cols && abs(y) < grid.rows;

    let specific = match grid.origin() {
        Origin::UpperLeft => x >= 0 && y <= 0,
        Origin::UpperRight => x <= 0 && y <= 0,
        Origin::LowerLeft => x >= 0 && y >= 0,
        Origin::LowerRight => x <= 0 && y >= 0,
        Origin::Center => {
            let x_offset = grid.cols / 2 + 1;
            let y_offset = grid.rows / 2 + 1;
            abs(x) < x_offset && abs(y) < y_offset
        }
    };

    if specific && maxlimit {
        Ok(())
    } else {
        Err(GridError::IndexOutOfBounds)
    }
}

// Index is UpperLeft row dominate indexing.  This will take the x, y coordinate and convert to vec index
// No bounds checking
pub(crate) fn xy_to_index<T>(grid: &Grid<T>, x: isize, y: isize) -> usize {
    let (x, y) = adjust_from_origin(grid, x, y);
    debug_assert!(x >= 0);
    debug_assert!(y >= 0);
    y as usize * grid.cols + x as usize
}

/// Take a (x, y) and adjust it to be the internal vec perspective of 0,0 in the upper left with inverted y axis
#[inline]
fn adjust_from_origin<T>(grid: &Grid<T>, x: isize, y: isize) -> (isize, isize) {
    match grid.origin() {
        Origin::UpperLeft => convert_upper_left(&grid, x, y),
        Origin::UpperRight => convert_upper_right(&grid, x, y),
        Origin::Center => convert_center(&grid, x, y),
        Origin::LowerLeft => convert_lower_left(&grid, x, y),
        Origin::LowerRight => convert_lower_right(&grid, x, y),
    }
}

/// Take a (x, y) based on a upper left inverted y axis and adjust it based on the origin
#[inline]
fn adjust_to_origin<T>(grid: &Grid<T>, x: isize, y: isize) -> (isize, isize) {
    match grid.origin() {
        Origin::UpperLeft => convert_upper_left(&grid, x, y),
        Origin::UpperRight => {
            let (tx, ty) = convert_upper_right(&grid, -x, y);
            (-tx, ty)
        }
        Origin::Center => {
            let (tx, ty) = convert_center(&grid, -x, y);
            (-tx, ty)
        }
        Origin::LowerLeft => convert_lower_left(&grid, x, y),
        Origin::LowerRight => {
            let (x, y) = convert_lower_right(&grid, -x, y);
            (-x, y)
        }
    }
}

#[inline]
fn convert_center<T>(grid: &Grid<T>, x: isize, y: isize) -> (isize, isize) {
    let x_offset = grid.cols / 2;
    let y_offset = grid.rows / 2;
    (x + x_offset as isize, -y + y_offset as isize)
}

#[inline]
fn convert_upper_left<T>(_grid: &Grid<T>, x: isize, y: isize) -> (isize, isize) {
    (x, -y)
}

#[inline]
fn convert_upper_right<T>(grid: &Grid<T>, x: isize, y: isize) -> (isize, isize) {
    (x + ((grid.cols - 1) as isize), -y)
}

#[inline]
fn convert_lower_left<T>(grid: &Grid<T>, x: isize, y: isize) -> (isize, isize) {
    (x, (grid.rows - 1) as isize - y)
}

#[inline]
fn convert_lower_right<T>(grid: &Grid<T>, x: isize, y: isize) -> (isize, isize) {
    ((grid.cols - 1) as isize + x, (grid.rows - 1) as isize - y)
}

#[cfg(test)]
mod index_tests {
    use super::*;
    use crate::grid::GridOptions;

    type Result<T> = std::result::Result<T, GridError>;

    fn basic_grid() -> Grid<i32> {
        Grid {
            items: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            rows: 4,
            cols: 3,
            options: None,
        }
    }

    fn center_origin() -> Grid<i32> {
        let mut grid = basic_grid();
        grid.items.append(&mut vec![12, 13, 14]);
        grid.rows += 1;
        grid.options = Some(GridOptions {
            origin: Origin::Center,
            ..GridOptions::default()
        });
        grid
    }

    fn origin_grid(origin: Origin) -> Grid<i32> {
        let mut grid = basic_grid();
        grid.options = Some(GridOptions {
            origin,
            ..GridOptions::default()
        });
        grid
    }

    #[test]
    fn default_origin() {
        let grid = basic_grid();
        assert_eq!(grid.origin(), Origin::UpperLeft);
    }

    #[test]
    fn center_origin_xy() {
        let grid = center_origin();
        let (x, y) = adjust_from_origin(&grid, 0, 0);
        assert_eq!(x, 1);
        assert_eq!(y, 2);

        let (x, y) = adjust_from_origin(&grid, -1, 2);
        assert_eq!(x, 0);
        assert_eq!(y, 0);

        let (x, y) = adjust_from_origin(&grid, 1, -2);
        assert_eq!(x, 2);
        assert_eq!(y, 4);
    }

    #[test]
    fn upperleft_xy() {
        let grid = origin_grid(Origin::UpperLeft);
        let (x, y) = adjust_from_origin(&grid, 0, 0);
        assert_eq!(x, 0);
        assert_eq!(y, 0);

        let (x, y) = adjust_from_origin(&grid, 1, -2);
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    }

    #[test]
    fn upperright_xy() {
        let grid = origin_grid(Origin::UpperRight);
        let (x, y) = adjust_from_origin(&grid, 0, 0);
        assert_eq!(x, 2);
        assert_eq!(y, 0);

        let (x, y) = adjust_from_origin(&grid, -1, -2);
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    }

    #[test]
    fn lowerleft_xy() {
        let grid = origin_grid(Origin::LowerLeft);
        let (x, y) = adjust_from_origin(&grid, 0, 0);
        assert_eq!(x, 0);
        assert_eq!(y, 3);

        let (x, y) = adjust_from_origin(&grid, 1, 2);
        assert_eq!(x, 1);
        assert_eq!(y, 1);
    }

    #[test]
    fn lowerright_xy() {
        let grid = origin_grid(Origin::LowerRight);
        let (x, y) = adjust_from_origin(&grid, 0, 0);
        assert_eq!(x, 2);
        assert_eq!(y, 3);

        let (x, y) = adjust_from_origin(&grid, -1, 2);
        assert_eq!(x, 1);
        assert_eq!(y, 1);
    }

    #[test]
    fn xy_to_index_test() {
        let grid = basic_grid();
        let index = xy_to_index(&grid, 1, -2);
        assert_eq!(index, 7);

        let index = xy_to_index(&grid, 3, -2);
        assert_eq!(index, 9);
    }

    #[test]
    fn xy_to_index_center() {
        let grid = center_origin();
        let index = xy_to_index(&grid, 1, 2);
        assert_eq!(index, 2);

        let index = xy_to_index(&grid, -1, -2);
        assert_eq!(index, 12);
    }

    #[test]
    fn should_err_on_outofbounds() {
        let grid = center_origin();
        let index = (2, 0).grid_index(&grid);
        assert!(matches!(index, Err(GridError::IndexOutOfBounds)));

        let index = Coordinates { x: -3, y: 0 }.grid_index(&grid);
        assert!(matches!(index, Err(GridError::IndexOutOfBounds)));

        let index = (1, 0).grid_index(&grid);
        assert!(matches!(index, Ok(x) if x == 8));
    }

    #[test]
    fn should_convert_index_upperleft() -> Result<()> {
        let mut grid = origin_grid(Origin::UpperLeft);
        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 0);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 1);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (1, 0));

        let index = (0, -1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 3);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, -1));

        let index = (2, -3).grid_index(&grid)?;
        assert_eq!(grid.items[index], 11);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (2, -3));

        let mut options = grid.options.unwrap().clone();
        options.inverted_y = true;
        grid.options = Some(options);

        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 0);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 1);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (1, 0));

        let index = (0, 1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 3);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 1));

        let index = (2, 3).grid_index(&grid)?;
        assert_eq!(grid.items[index], 11);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (2, 3));

        Ok(())
    }

    #[test]
    fn should_convert_index_upperright() -> Result<()> {
        let mut grid = origin_grid(Origin::UpperRight);
        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 2);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (-1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 1);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-1, 0));

        let index = (0, -1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 5);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, -1));

        let index = (-2, -3).grid_index(&grid)?;
        assert_eq!(grid.items[index], 9);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-2, -3));

        let mut options = grid.options.unwrap().clone();
        options.inverted_y = true;
        grid.options = Some(options);

        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 2);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (-1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 1);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-1, 0));

        let index = (0, 1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 5);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 1));

        let index = (-2, 3).grid_index(&grid)?;
        assert_eq!(grid.items[index], 9);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-2, 3));

        Ok(())
    }

    #[test]
    fn should_convert_index_lowerleft() -> Result<()> {
        let mut grid = origin_grid(Origin::LowerLeft);
        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 9);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 10);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (1, 0));

        let index = (0, 1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 6);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 1));

        let index = (2, 3).grid_index(&grid)?;
        assert_eq!(grid.items[index], 2);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (2, 3));

        let mut options = grid.options.unwrap().clone();
        options.inverted_y = true;
        grid.options = Some(options);

        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 9);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 10);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (1, 0));

        let index = (0, -1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 6);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, -1));

        let index = (2, -3).grid_index(&grid)?;
        assert_eq!(grid.items[index], 2);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (2, -3));

        Ok(())
    }

    #[test]
    fn should_convert_index_lowerright() -> Result<()> {
        let mut grid = origin_grid(Origin::LowerRight);
        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 11);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (-1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 10);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-1, 0));

        let index = (0, 1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 8);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 1));

        let index = (-2, 3).grid_index(&grid)?;
        assert_eq!(grid.items[index], 0);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-2, 3));

        let mut options = grid.options.unwrap().clone();
        options.inverted_y = true;
        grid.options = Some(options);

        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 11);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (-1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 10);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-1, 0));

        let index = (0, -1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 8);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, -1));

        let index = (-2, -3).grid_index(&grid)?;
        assert_eq!(grid.items[index], 0);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-2, -3));

        Ok(())
    }

    #[test]
    fn should_convert_index_center() -> Result<()> {
        let mut grid = center_origin();
        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 7);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (-1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 6);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-1, 0));

        let index = (0, 1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 4);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 1));

        let index = (-1, 2).grid_index(&grid)?;
        assert_eq!(grid.items[index], 0);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-1, 2));

        let mut options = grid.options.unwrap().clone();
        options.inverted_y = true;
        grid.options = Some(options);

        let index = (0, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 7);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, 0));

        let index = (-1, 0).grid_index(&grid)?;
        assert_eq!(grid.items[index], 6);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-1, 0));

        let index = (0, -1).grid_index(&grid)?;
        assert_eq!(grid.items[index], 4);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (0, -1));

        let index = (-1, -2).grid_index(&grid)?;
        assert_eq!(grid.items[index], 0);
        let output: (isize, isize) = Index::output(index, &grid);
        assert_eq!(output, (-1, -2));

        Ok(())
    }

    #[test]
    fn coodinate_index() -> Result<()> {
        let mut grid = center_origin();
        let index = (0, 0).grid_index(&grid)?;
        let cord_index = Coordinates { x: 0, y: 0 }.grid_index(&grid)?;
        assert_eq!(index, cord_index);

        let index = (-1, 2).grid_index(&grid)?;
        let cord_index = Coordinates { x: -1, y: 2 }.grid_index(&grid)?;

        let cord_index = Coordinates { x: -2, y: 2 }.grid_index(&grid);
        assert!(matches!(cord_index, Err(GridError::IndexOutOfBounds)));
        Ok(())
    }

    #[test]
    fn usize_index() -> Result<()> {
        let mut grid = basic_grid();
        let index = 5usize.grid_index(&grid)?;
        assert_eq!(index, 5);

        let index = 11usize.grid_index(&grid)?;
        assert_eq!(index, 11);

        let cord_index = 12usize.grid_index(&grid);
        assert!(matches!(cord_index, Err(GridError::IndexOutOfBounds)));
        Ok(())
    }
}
