use crate::col_iters::{ColIter, MutColIter};
use crate::error::GridError;
use crate::index::{Coordinates, Index};
use crate::intogrid::IntoGrid;
use crate::row_iters::{MutRowIter, RowIter};
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
    /// let neighbors = grid.xy_neighbors((-1,-2)); // Neighbors of the item with 12 in it.
    /// assert_eq!(neighbors.len(), 2);
    /// let neighbors: Vec<_> = neighbors.iter().map(|x| x.value).collect();
    /// assert!(neighbors.contains(&&9));
    /// assert!(neighbors.contains(&&13));
    ///
    /// let neighbors = grid.xy_neighbors((0,0));
    /// assert_eq!(neighbors.len(), 4);
    /// let neighbors:Vec<_> = neighbors.iter().map(|x| x.value).collect();
    /// assert!(neighbors.contains(&&4));
    /// assert!(neighbors.contains(&&10));
    /// assert!(neighbors.contains(&&6));
    /// assert!(neighbors.contains(&&8));
    ///```
    pub fn xy_neighbors<I: Index>(&self, index: I) -> Vec<Neighbor<'_, T>> {
        let make_coordinate = |ind_op: Option<usize>| -> Option<Neighbor<'_, T>> {
            if let Some(index) = ind_op {
                if let Some(value) = self._get(index) {
                    return Some(Neighbor {
                        value,
                        location: Coordinates::output(index, self),
                    });
                }
            }
            None
        };

        if let Ok(index) = index.grid_index(self) {
            let indicies = self.xy_neighbor_locations(index);
            indicies
                .into_iter()
                .filter(|i| i.is_some())
                .filter_map(make_coordinate)
                .collect()
        } else {
            vec![]
        }
    }

    fn xy_neighbor_locations(&self, index: usize) -> [Option<usize>; 4] {
        let down = index.checked_add(self.cols as usize).and_then(|v| {
            if v < self.size() {
                Some(v)
            } else {
                None
            }
        });
        let up = index.checked_sub(self.cols as usize);

        let col = col_number(self, index);
        let left = if col > 0 { Some(index - 1) } else { None };
        let right = if col < self.cols as usize - 1 {
            Some(index + 1)
        } else {
            None
        };
        [up, down, left, right]
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
