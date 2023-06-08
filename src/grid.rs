use crate::col_iters::{ColIter, MutColIter};
use crate::error::GridError;
use crate::index::{Coordinates, Index};
use crate::intogrid::IntoGrid;
use crate::row_iters::{MutRowIter, RowIter};
use crate::xyneightbor::XyNeighbor;
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub(crate) items: Vec<T>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) options: Option<GridOptions>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GridOptions {
    pub origin: Origin,
    pub inverted_y: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Origin {
    #[default]
    UpperLeft,
    UpperRight,
    Center,
    LowerLeft,
    LowerRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Neighbor<'a, T> {
    pub value: &'a T,
    pub location: Coordinates,
}

impl<T> Grid<T> {
    pub fn new<I: IntoGrid<T>>(items: I, options: Option<GridOptions>) -> Result<Self, GridError> {
        let grid = Grid {
            options,
            ..items.into_grid()?
        };

        // If origin is in the center, then the grid should have odd columns and rows
        if let Some(options) = &grid.options {
            if options.origin == Origin::Center {
                if grid.rows & 1 == 0 || grid.cols & 1 == 0 {
                    return Err(GridError::InvalidSize);
                }
            }
        }

        Ok(grid)
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub fn columns(&self) -> usize {
        self.cols
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&T> {
        if let Ok(index) = index.grid_index(&self) {
            Some(&self.items[index])
        } else {
            None
        }
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        if let Ok(index) = index.grid_index(&self) {
            Some(&mut self.items[index])
        } else {
            None
        }
    }

    pub fn get_up<I: Index>(&self, index: I) -> Option<&T> {
        let idx = self.up_idx(index).ok()?;
        Some(&self.items[idx])
    }

    pub fn get_down<I: Index>(&self, index: I) -> Option<&T> {
        let idx = self.down_idx(index).ok()?;
        Some(&self.items[idx])
    }

    pub fn get_left<I: Index>(&self, index: I) -> Option<&T> {
        let idx = self.left_idx(index).ok()?;
        Some(&self.items[idx])
    }

    pub fn get_right<I: Index>(&self, index: I) -> Option<&T> {
        let idx = self.right_idx(index).ok()?;
        Some(&self.items[idx])
    }

    pub fn get_up_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.up_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    pub fn get_down_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.down_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    pub fn get_left_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.left_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    pub fn get_right_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.right_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    fn down_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        let index = index.grid_index(self)?;
        let res = index + self.cols;
        if res < self.size() {
            Ok(res)
        } else {
            Err(GridError::IndexOutOfBounds)
        }
    }

    fn up_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        let index = index.grid_index(self)?;
        index
            .checked_sub(self.cols)
            .ok_or(GridError::IndexOutOfBounds)
    }

    fn left_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        let index = index.grid_index(self)?;
        if index == 0 || index % self.cols == 0 {
            Err(GridError::IndexOutOfBounds)
        } else {
            Ok(index - 1)
        }
    }

    fn right_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        let index = index.grid_index(self)? + 1;
        if index == self.size() || index % self.cols == 0 {
            Err(GridError::IndexOutOfBounds)
        } else {
            Ok(index)
        }
    }

    fn _get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn _get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    pub fn iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = &'a T> + 'b {
        self.items.iter()
    }

    pub fn iter_mut<'b, 'a: 'b>(&'a mut self) -> impl Iterator<Item = &'a mut T> + 'b {
        self.items.iter_mut()
    }

    /// Returns an iterator starting from the beginning of the row that the passed in index is on
    /// ```
    /// use neighborgrid::*;
    /// let vec = vec![
    ///             vec![0, 1, 2],
    ///             vec![3, 4, 5],
    ///             vec![6, 7, 8],
    ///             vec![9, 10, 11],
    ///             vec![12, 13, 14],
    /// ];
    /// let gridoptions = GridOptions {
    ///        origin: Origin::Center,
    ///         ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// let mut iter = grid.row_iter((0, 1));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), None)
    ///```

    pub fn row_iter<'b, 'a: 'b, I: Index>(&'a self, index: I) -> RowIter<'b, T> {
        let res = index.grid_index(&self);
        // Noop coverts invalid grid location Result into an iterator that returns None right way
        match res {
            Ok(i) => RowIter::new(self, i),
            Err(_) => RowIter::noop(),
        }
    }

    /// Returns an iterator starting from the beginning of the row that the passed in index is on
    /// ```
    /// use neighborgrid::*;
    /// let vec = vec![
    ///             vec![0, 1, 2],
    ///             vec![3, 4, 5],
    ///             vec![6, 7, 8],
    ///             vec![9, 10, 11],
    ///             vec![12, 13, 14],
    /// ];
    /// let gridoptions = GridOptions {
    ///        origin: Origin::Center,
    ///         ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// let mut iter = grid.col_iter((1, -2));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), Some(&8));
    /// assert_eq!(iter.next(), Some(&11));
    /// assert_eq!(iter.next(), Some(&14));
    /// assert_eq!(iter.next(), None)
    ///```
    pub fn col_iter<'b, 'a: 'b, I: Index>(&'a self, index: I) -> ColIter<'b, T> {
        let res = index.grid_index(&self);
        // Noop coverts invalid grid location Result into an iterator that returns None right way
        match res {
            Ok(i) => ColIter::new(self, i),
            Err(_) => ColIter::noop(),
        }
    }

    pub fn row_iter_mut<'b, 'a: 'b, I: Index>(&'a mut self, index: I) -> MutRowIter<'b, T> {
        let res = index.grid_index(&self);
        // Noop coverts invalid grid location Result into an iterator that returns None right way
        match res {
            Ok(i) => MutRowIter::new(self, i),
            Err(_) => MutRowIter::noop(),
        }
    }

    pub fn col_iter_mut<'b, 'a: 'b, I: Index>(&'a mut self, index: I) -> MutColIter<'b, T> {
        let res = index.grid_index(&self);
        // Noop coverts invalid grid location Result into an iterator that returns None right way
        match res {
            Ok(i) => MutColIter::new(self, i),
            Err(_) => MutColIter::noop(),
        }
    }

    /// Returns an vector of {Neighbor}s. Order is left, right, bottom, top of index called. The vec only
    /// contains neighbors that are in grid, therefore calling this function on a item along the edge will only return 3 neighbors.
    /// Order in which they are returned relative to the central location is not guarenteed, so if relative position is important, use the coordinate component.
    /// ```
    /// use neighborgrid::*;
    /// let vec = vec![
    ///             vec![0, 1, 2],
    ///             vec![3, 4, 5],
    ///             vec![6, 7, 8],
    ///             vec![9, 10, 11],
    ///             vec![12, 13, 14],
    /// ];
    /// let gridoptions = GridOptions {
    ///        origin: Origin::Center,
    ///         ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// let neighbors = grid.xy_neighbors((-1,-2)).expect("was not a valid coodinate"); // Neighbors of the item with 12 in it.
    /// assert_eq!(neighbors.up, Some(&9));
    /// assert_eq!(neighbors.down, None);
    /// assert_eq!(neighbors.left, None);
    /// assert_eq!(neighbors.right, Some(&13));
    ///```
    pub fn xy_neighbors<I: Index>(&self, index: I) -> Result<XyNeighbor<'_, T>, GridError> {
        let index = index.grid_index(&self)?;
        Ok(XyNeighbor {
            up: self.get_up(index),
            down: self.get_down(index),
            left: self.get_left(index),
            right: self.get_right(index),
        })
    }

    pub(crate) fn create(
        items: Vec<T>,
        rows: usize,
        cols: usize,
        options: Option<GridOptions>,
    ) -> Grid<T> {
        Grid {
            items,
            rows,
            cols,
            options,
        }
    }
    #[inline]
    pub(crate) fn origin(&self) -> Origin {
        if let Some(options) = &self.options {
            return options.origin.clone();
        }
        Origin::default()
    }
}

pub(crate) fn row_number<T>(grid: &Grid<T>, index: usize) -> usize {
    index / grid.cols as usize
}

pub(crate) fn col_number<T>(grid: &Grid<T>, index: usize) -> usize {
    index % grid.cols as usize
}

pub(crate) fn row_col_number<T>(grid: &Grid<T>, index: usize) -> (usize, usize) {
    (row_number(grid, index), col_number(grid, index))
}

pub(crate) fn row_start_index<T>(grid: &Grid<T>, index: usize) -> usize {
    row_number(grid, index) * grid.cols as usize
}

pub(crate) fn col_start_index<T>(grid: &Grid<T>, index: usize) -> usize {
    col_number(grid, index)
}

#[cfg(test)]
mod grid_tests {

    fn center_grid() -> Grid<i32> {
        let vec = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![9, 10, 11],
            vec![12, 13, 14],
        ];
        let gridoptions = GridOptions {
            origin: Origin::Center,
            ..GridOptions::default()
        };
        let grid = Grid::new(vec, Some(gridoptions));
        grid.unwrap()
    }
    use super::*;
    #[test]
    fn should_contain_large_size() -> Result<(), GridError> {
        let vec = vec![vec![1; u16::MAX as usize]; 1000];
        let grid = vec.into_grid()?;
        assert_eq!(grid.rows, 1000);
        assert_eq!(grid.cols, usize::from(u16::MAX));

        let vec = vec![vec![1; 1000]; u16::MAX as usize];
        let grid = vec.into_grid()?;
        assert_eq!(grid.rows, u16::MAX as usize);
        assert_eq!(grid.cols, 1000);

        Ok(())
    }

    mod getters {
        use super::*;

        #[test]
        fn should_get_item() {
            let grid = center_grid();
            assert_eq!(grid.get((0, 0)).unwrap(), &7i32);
            assert_eq!(grid.get((-1, 1)).unwrap(), &3i32);
            assert_eq!(grid.get(1).unwrap(), &1i32);
            assert_eq!(grid.get((-2, 0)), None);
        }

        #[test]
        fn should_get_mut_item() {
            let mut grid = center_grid();
            let v = grid.get_mut((0, 0)).unwrap();
            assert_eq!(*v, 7i32);
            *v = 12i32;
            assert_eq!(*v, 12i32);
            let v = grid.get((0, 0)).unwrap();
            assert_eq!(*v, 12i32);
        }

        #[test]
        fn should_get_up() {
            let grid = center_grid();
            assert_eq!(grid.get_up((0, 0)), Some(&4i32));
            assert_eq!(grid.get_up((-1, 1)), Some(&0i32));
            assert_eq!(grid.get_up(1), None);
            assert_eq!(grid.get_up((-2, 0)), None);
        }

        #[test]
        fn should_get_down() {
            let grid = center_grid();
            assert_eq!(grid.get_down((0, 0)), Some(&10i32));
            assert_eq!(grid.get_down((-1, 1)), Some(&6i32));
            assert_eq!(grid.get_down(12), None);
            assert_eq!(grid.get_down((-2, 0)), None);
        }

        #[test]
        fn should_get_left() {
            let grid = center_grid();
            assert_eq!(grid.get_left((0, 0)), Some(&6i32));
            assert_eq!(grid.get_left((1, 1)), Some(&4i32));
            assert_eq!(grid.get_left(12), None);
            assert_eq!(grid.get_left((-2, 0)), None);
        }

        #[test]
        fn should_get_right() {
            let grid = center_grid();
            assert_eq!(grid.get_right((0, 0)), Some(&8i32));
            assert_eq!(grid.get_right((-1, -1)), Some(&10i32));
            assert_eq!(grid.get_right(11), None);
            assert_eq!(grid.get_right((-2, 0)), None);
        }
    }

    mod row_iters {
        use super::*;

        #[test]
        fn should_return_none_outside_bounds() {
            let mut grid = center_grid();
            let mut iter = grid.row_iter((2, 0));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn should_return_none_outside_bounds_mut() {
            let mut grid = center_grid();
            let mut iter = grid.row_iter_mut((2, 0));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn should_iter_mutably() {
            let mut grid = center_grid();
            for mut cell in grid.row_iter_mut((0, 1)) {
                *cell += 1;
            }
            let mut iter = grid.row_iter((0, 1));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), Some(&6));
            assert_eq!(iter.next(), None);
        }
    }

    mod col_iters {
        use super::*;

        #[test]
        fn should_return_none_outside_bounds() {
            let mut grid = center_grid();
            let mut iter = grid.col_iter((-4, 0));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn should_return_none_outside_bounds_mut() {
            let mut grid = center_grid();
            let mut iter = grid.col_iter_mut((-4, 0));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn should_iter_mutably() {
            let mut grid = center_grid();
            for mut cell in grid.col_iter_mut((0, 1)) {
                *cell += 1;
            }
            let mut iter = grid.col_iter((0, 1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), Some(&8));
            assert_eq!(iter.next(), Some(&11));
            assert_eq!(iter.next(), Some(&14));
            assert_eq!(iter.next(), None);
        }
    }
}
