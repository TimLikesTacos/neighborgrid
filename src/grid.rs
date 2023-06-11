use crate::col_iters::{ColIter, MutColIter};
use crate::error::GridError;
use crate::index::Index;
use crate::intogrid::IntoGrid;
pub use crate::origin::Origin;
use crate::quaditers::NrantIterator;
use crate::row_iters::{MutRowIter, RowIter};
use crate::xyneightbor::AllAroundNeighbor;
pub use crate::xyneightbor::XyNeighbor;

const NEIGHBOR_Y_BASED: bool = true;
const DEFAULT_WRAP: bool = false;

/// A collection that represents a 2-D grid with equal amount of cells in each row and equal number of cells in each column.  Supports different origin (location of 0,0) configurations,
/// and includes methods to get neighbors of cells, iterators, and more.  Behind the scenes, the data is stored in a 1-D `Vec` to improve performance, but interaction with grid is done through normal (x,y)
/// grid location methods.
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub(crate) items: Vec<T>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) options: GridOptions,
}

/// Custom configuration of the grid.  For most grids out there, with x and y values always positive, an `origin: Origin::UpperLeft` and `inverted_y: true` is the best fit, and therefore is the default setting.
#[derive(Debug, Clone, PartialEq)]
pub struct GridOptions {
    pub origin: Origin,
    pub inverted_y: bool,
    pub neighbor_ybased: bool,
    pub wrap_x: bool,
    pub wrap_y: bool,
}

impl Default for GridOptions {
    fn default() -> Self {
        GridOptions {
            origin: Origin::default(),
            inverted_y: true,
            neighbor_ybased: NEIGHBOR_Y_BASED,
            wrap_x: DEFAULT_WRAP,
            wrap_y: DEFAULT_WRAP,
        }
    }
}
impl<T> Grid<T> {
    /// Create a new grid. If `options` is `None`, then default `GridOptions` are used.  Takes as parameter `items`, which is anything that implements the `IntoGrid` trait.  
    /// These are things like a 2-D Vec, 1-D vec with row parameters, and others.
    pub fn new<I: IntoGrid<T>>(items: I, options: Option<GridOptions>) -> Result<Self, GridError> {
        let grid = Grid {
            options: options.unwrap_or_default(),
            ..items.into_grid()?
        };

        Ok(grid)
    }

    /// Already have a 1-D Vec for your grid?  Use this method to create a `Grid`, just specify how many rows and columns.  
    /// Returns Err if the vec size is not equal to the product of the rows and columns
    pub fn new_from_1d(
        vec: Vec<T>,
        columns: usize,
        rows: usize,
        options: Option<GridOptions>,
    ) -> Result<Self, GridError> {
        if vec.len() != rows.checked_mul(columns).ok_or(GridError::ExcessiveSize)? {
            return Err(GridError::InvalidSize);
        }
        Ok(Grid {
            items: vec,
            rows,
            cols: columns,
            options: options.unwrap_or_default(),
        })
    }

    /// The number of cells in the grid
    #[inline]
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// The number of rows
    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// The number of columns
    #[inline]
    pub fn columns(&self) -> usize {
        self.cols
    }

    /// Returns a immutable reference to the value stored in the specified cell.  None if outside the grid bounds
    pub fn get<I: Index>(&self, index: I) -> Option<&T> {
        if let Ok(index) = index.grid_index(&self) {
            Some(&self.items[index])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value stored in the specified cell.  None if outside the grid bounds
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
    ///        origin: Origin::UpperLeft,
    ///        inverted_y: true,
    ///        ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// let middle_cell = grid.get_mut((1, 2)).expect("invalid coodinate");
    /// assert_eq!(middle_cell, &mut 7);
    /// *middle_cell = 8;
    /// assert_eq!(middle_cell, &mut 8);
    /// ```
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        if let Ok(index) = index.grid_index(&self) {
            Some(&mut self.items[index])
        } else {
            None
        }
    }

    /// Return an immutable reference to the value stored in the cell with a 1 higher y-value. None if outside grid bounds
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
    ///        origin: Origin::LowerLeft,
    ///        inverted_y: false,
    ///        neighbor_ybased: true,
    ///        ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// // (2,1) is coordinate for 11, above that is 8
    /// let up = grid.get_up((2, 1));
    /// assert_eq!(up, Some(&8));
    /// // Asking for `up` at the top of the grid will return `None`
    /// assert_eq!(grid.get((2, 4)), Some(&2));
    /// assert_eq!(grid.get_up((2, 4)), None);
    /// ```
    /// The `GridOption` of `neighbor_ybased` does not have an impact when `inverted_y` is false.  Below is the same as above but with `neighbor_ybased` set to false.
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
    ///        origin: Origin::LowerLeft,
    ///        inverted_y: false,
    ///        neighbor_ybased: false,
    ///        ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// // (2,1) is coordinate for 11, above that is 8
    /// let up = grid.get_up((2, 1));
    /// assert_eq!(up, Some(&8));
    /// // Asking for `up` at the top of the grid will return `None`
    /// assert_eq!(grid.get((2, 4)), Some(&2));
    /// assert_eq!(grid.get_up((2, 4)), None);
    /// ```
    ///
    /// Note that this and `get_down` act differently with the `GridOption` of `inverted_y` and `neighbor_ybased`
    /// ```
    /// use neighborgrid::*;
    /// let vec = vec![
    ///             vec![0, 1, 2],
    ///             vec![3, 4, 5],
    ///             vec![6, 7, 8],
    ///             vec![9, 10, 11],
    ///             vec![12, 13, 14],
    /// ];
    /// let mut gridoptions = GridOptions {
    ///        origin: Origin::LowerLeft,
    ///        inverted_y: true,
    ///        neighbor_ybased: true,
    ///        ..GridOptions::default()
    /// };
    /// let grid = Grid::new(vec.clone(), Some(gridoptions.clone())).expect("failed to import 2d vec");
    ///
    /// // (2,1) is coordinate for 11, above (y+1)that is 14
    /// let up = grid.get_up((2, -1));
    /// assert_eq!(up, Some(&14));
    /// // Note how this is different from the previous example.
    /// assert_eq!(grid.get((2, -4)), Some(&2));
    /// assert_eq!(grid.get_up((2, -4)), Some(&5));
    ///
    /// gridoptions.neighbor_ybased = false;
    /// let grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// // (2,1) is coordinate for 11, above (relative) is 8
    /// let up = grid.get_up((2, -1));
    /// assert_eq!(up, Some(&8));
    /// // Note how this is different from the previous example.
    /// assert_eq!(grid.get((2, -4)), Some(&2));
    /// assert_eq!(grid.get_up((2, -4)), None);
    /// ```
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

    pub fn get_upleft<I: Index>(&self, index: I) -> Option<&T> {
        let idx = self.upleft_idx(index).ok()?;
        Some(&self.items[idx])
    }

    pub fn get_upright<I: Index>(&self, index: I) -> Option<&T> {
        let idx = self.upright_idx(index).ok()?;
        Some(&self.items[idx])
    }

    pub fn get_downleft<I: Index>(&self, index: I) -> Option<&T> {
        let idx = self.downleft_idx(index).ok()?;
        Some(&self.items[idx])
    }

    #[inline]
    pub fn get_downright<I: Index>(&self, index: I) -> Option<&T> {
        let idx = self.downright_idx(index).ok()?;
        Some(&self.items[idx])
    }

    #[inline]
    pub fn get_up_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.up_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    #[inline]
    pub fn get_down_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.down_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    #[inline]
    pub fn get_left_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.left_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    #[inline]
    pub fn get_right_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.right_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    #[inline]
    pub fn get_upleft_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.upleft_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    #[inline]
    pub fn get_upright_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.upright_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    #[inline]
    pub fn get_downleft_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.downleft_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    #[inline]
    pub fn get_downright_mut<I: Index>(&mut self, index: I) -> Option<&mut T> {
        let idx = self.downright_idx(index).ok()?;
        Some(&mut self.items[idx])
    }

    #[inline]
    fn down_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        let index = index.grid_index(self)?;
        if self.is_inverted_y() && self.neighbor_ybased_invert() {
            self.actual_up_ind(index)
        } else {
            self.actual_down_ind(index)
        }
    }

    #[inline]
    fn actual_down_ind(&self, index: usize) -> Result<usize, GridError> {
        let res = index + self.cols;
        if res < self.size() {
            Ok(res)
        } else {
            if self.options.wrap_y {
                Ok(res - self.size())
            } else {
                Err(GridError::IndexOutOfBounds)
            }
        }
    }

    #[inline]
    fn downleft_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        self.down_idx(index).and_then(|i| self.left_idx(i))
    }

    #[inline]
    fn downright_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        self.down_idx(index).and_then(|i| self.right_idx(i))
    }

    fn actual_up_ind(&self, index: usize) -> Result<usize, GridError> {
        match index.checked_sub(self.cols) {
            Some(v) => Ok(v),
            None => {
                if self.options.wrap_y {
                    Ok(index + self.size() - self.cols)
                } else {
                    Err(GridError::IndexOutOfBounds)
                }
            }
        }
    }

    #[inline]
    fn neighbor_ybased_invert(&self) -> bool {
        self.options.neighbor_ybased
    }

    fn up_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        let index = index.grid_index(self)?;
        if self.is_inverted_y() && self.neighbor_ybased_invert() {
            self.actual_down_ind(index)
        } else {
            self.actual_up_ind(index)
        }
    }

    #[inline]
    fn upleft_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        self.up_idx(index).and_then(|i| self.left_idx(i))
    }

    #[inline]
    fn upright_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        self.up_idx(index).and_then(|i| self.right_idx(i))
    }

    fn left_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        let index = index.grid_index(self)?;
        if index == 0 || index % self.cols == 0 {
            if self.options.wrap_x {
                Ok(index + self.columns() - 1)
            } else {
                Err(GridError::IndexOutOfBounds)
            }
        } else {
            Ok(index - 1)
        }
    }

    fn right_idx<I: Index>(&self, index: I) -> Result<usize, GridError> {
        let index = index.grid_index(self)? + 1;
        if index == self.size() || index % self.cols == 0 {
            if self.options.wrap_x {
                Ok(index - self.columns())
            } else {
                Err(GridError::IndexOutOfBounds)
            }
        } else {
            Ok(index)
        }
    }

    #[inline]
    fn _get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    #[inline]
    fn _get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    #[inline]
    fn is_inverted_y(&self) -> bool {
        self.options.inverted_y
    }

    /// Iterates over all elements
    #[inline]
    pub fn iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = &'a T> + 'b {
        self.items.iter()
    }

    /// Mutable iterator over all elements
    #[inline]
    pub fn iter_mut<'b, 'a: 'b>(&'a mut self) -> impl Iterator<Item = &'a mut T> + 'b {
        self.items.iter_mut()
    }

    /// Maximum x-value for grid coodinate. Depends on which `Origin` is used in `GridOptions`
    #[inline]
    pub fn max_x(&self) -> isize {
        self.origin().max_x(self)
    }

    /// Maximum y-value for grid coodinate. Depends on which `Origin` is used in `GridOptions`
    #[inline]
    pub fn max_y(&self) -> isize {
        self.origin().max_y(self)
    }

    /// Minimum x-value for grid coodinate. Depends on which `Origin` is used in `GridOptions`
    #[inline]
    pub fn min_x(&self) -> isize {
        self.origin().min_x(self)
    }

    /// Minimum y-value for grid coodinate. Depends on which `Origin` is used in `GridOptions`
    #[inline]
    pub fn min_y(&self) -> isize {
        self.origin().min_y(self)
    }

    /// Returns which Nth-rant (or whatever the actual mathy term is) the index is in. Quadrant size is done with ceiling math, so grids not evenly divisible by the `divisor` will have smaller amount of cells in the bottom and right quadrants.
    /// For example, if you have a 9X9 grid and want sections 3x3, like a Sudoku puzzle, you would use a divisor of 3 ( 9 / 3 == 3 );
    pub fn nrant<I: Index>(&self, index: I, divisor: usize) -> Result<usize, GridError> {
        if divisor < 1 || divisor > std::cmp::max(self.rows(), self.columns()) {
            return Err(GridError::InvalidDivisionSize);
        }
        let index = index.grid_index(self)?;
        let rheight = ceiling(self.rows(), divisor);
        let rwidth = ceiling(self.columns(), divisor);
        let steps = index / self.columns() / rheight * divisor + (index % self.columns()) / rwidth;
        Ok(steps)
    }

    /// Returns the index of the first cell of the Nrant
    pub(crate) fn nrant_start(&self, index: usize, divisor: usize) -> usize {
        let nrant = self
            .nrant(index, divisor)
            .expect("Index already validated. This is not a public facing method");
        let x_rants = nrant % divisor;
        let y_rants = nrant / divisor;
        let x_offset = x_rants * ceiling(self.columns(), divisor);
        let y_offset = self.rows() / divisor * y_rants;
        y_offset * self.columns() + x_offset
    }

    /// Returns which quadrant the index is in.  GridOptions configuration does not have an impact. This is a simplified call to `self.nrant(index, 2)`
    pub fn quadrant<I: Index>(&self, index: I) -> Result<usize, GridError> {
        self.nrant(index, 2)
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
    ///        inverted_y: false,
    ///        ..GridOptions::default()
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
    ///        inverted_y: false,
    ///        ..GridOptions::default()
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

    /// Swap two cells with each other.
    pub fn swap<I: Index>(&mut self, a: I, b: I) -> Result<(), GridError> {
        let a = a.grid_index(self)?;
        let b = b.grid_index(self)?;
        Ok(self.items.swap(a, b))
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

    /// Returns an `nrant_iter` with a divisor of 2.  Hence, the grid is split into 4 quadrants and iterates over the quadrant that the
    /// index belongs to, from the start of the quadrant to the end of the quadrant.
    pub fn quadrant_iter<'b, 'a: 'b, I: Index>(&'a self, index: I) -> NrantIterator<'b, T> {
        self.nrant_iter(2, index)
    }

    /// Divides the grid, as specified by `divisor`, and produces `divisor * divisor` amount of sections.  For example, if `divisor` is `2`, then
    /// the grid is divided into quadrants.  Making a Sudoku puzzle and want 9 sections (each 3x3) for your 9x9 grid?  Use a `divisor` of `3`.
    /// ```
    /// use neighborgrid::*;
    /// let mut vec = vec![];
    ///
    ///for i in 1..=81 {
    ///    vec.push(i);
    ///}
    ///
    ///let grid = Grid::new_from_1d(vec, 9, 9, None).unwrap();
    ///
    ///let mut iter = grid.nrant_iter(3, (1,1));
    ///assert_eq!(iter.next(), Some(Some(&1)));
    ///assert_eq!(iter.next(), Some(Some(&2)));
    ///assert_eq!(iter.next(), Some(Some(&3)));
    ///assert_eq!(iter.next(), Some(Some(&10)));
    ///assert_eq!(iter.next(), Some(Some(&11)));
    ///assert_eq!(iter.next(), Some(Some(&12)));
    ///assert_eq!(iter.next(), Some(Some(&19)));
    ///assert_eq!(iter.next(), Some(Some(&20)));
    ///assert_eq!(iter.next(), Some(Some(&21)));
    ///assert_eq!(iter.next(), None);
    ///```

    pub fn nrant_iter<'b, 'a: 'b, I: Index>(
        &'a self,
        divisor: usize,
        index: I,
    ) -> NrantIterator<'b, T> {
        let res = index.grid_index(&self);
        // Noop coverts invalid grid location Result into an iterator that returns None right way
        match res {
            Ok(i) => NrantIterator::new(self, divisor, i),
            Err(_) => NrantIterator::noop(self),
        }
    }
    /// Returns an `XyNeighbor` which are the four neighbors in cardinal directions from the called cell location
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
    ///        inverted_y: false,
    ///        ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// let neighbors = grid.xy_neighbors((-1,-2)).expect("was not a valid coodinate"); // Neighbors of the item with 12 in it.
    /// assert_eq!(neighbors.up, Some(&9));
    /// assert_eq!(neighbors.down, None);
    /// assert_eq!(neighbors.left, None);
    /// assert_eq!(neighbors.right, Some(&13));
    ///```
    /// If `GridOptions` `x_wrap` and / or `y_wrap` are true, then the wrapped neighbors will be returned
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
    ///        inverted_y: false,
    ///        wrap_x: true, // turn on wrapping
    ///        wrap_y: true,
    ///        ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    ///
    /// let neighbors = grid.xy_neighbors((-1,-2)).expect("was not a valid coodinate"); // Neighbors of the item with 12 in it.
    /// assert_eq!(neighbors.up, Some(&9));
    /// assert_eq!(neighbors.down, Some(&0));
    /// assert_eq!(neighbors.left, Some(&14));
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

    /// Returns an `AllAroundNeighbor` of the neighbors of the specified cell. Order is left, right, bottom, top of index called.
    /// ```
    /// use neighborgrid::*;
    /// let vec = vec![
    ///     vec![0, 1, 2, 3],
    ///     vec![4, 5, 6, 7],
    ///     vec![8, 9, 10, 11],
    ///     vec![12, 13, 14, 15],
    ///     vec![16, 17, 18, 19],
    /// ];
    /// let gridoptions = GridOptions {
    ///     origin: Origin::UpperLeft,
    ///     inverted_y: true,
    ///     neighbor_ybased: false,
    ///     ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    /// let neighbors = grid
    ///     .all_around_neighbors((0, 1))
    ///     .expect("was not a valid coodinate"); // Neighbors of the item with 4 in it.
    /// assert_eq!(neighbors.upleft, None);
    /// assert_eq!(neighbors.up, Some(&0));
    /// assert_eq!(neighbors.upright, Some(&1));
    /// assert_eq!(neighbors.left, None);
    /// assert_eq!(neighbors.right, Some(&5));
    /// assert_eq!(neighbors.downleft, None);
    /// assert_eq!(neighbors.down, Some(&8));
    /// assert_eq!(neighbors.downright, Some(&9));
    ///```
    /// Using `GridOptions` to enable wrap:
    ///
    /// ```
    /// use neighborgrid::*;
    /// let vec = vec![
    ///     vec![0, 1, 2, 3],
    ///     vec![4, 5, 6, 7],
    ///     vec![8, 9, 10, 11],
    ///     vec![12, 13, 14, 15],
    ///     vec![16, 17, 18, 19],
    /// ];
    /// let gridoptions = GridOptions {
    ///     origin: Origin::UpperLeft,
    ///     inverted_y: true,
    ///     neighbor_ybased: false,
    ///     wrap_x: true,
    ///     wrap_y:true,
    ///     ..GridOptions::default()
    /// };
    /// let mut grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
    /// let neighbors = grid
    ///     .all_around_neighbors((0, 1))
    ///     .expect("was not a valid coodinate"); // Neighbors of the item with 4 in it.
    /// assert_eq!(neighbors.upleft, Some(&3));
    /// assert_eq!(neighbors.up, Some(&0));
    /// assert_eq!(neighbors.upright, Some(&1));
    /// assert_eq!(neighbors.left, Some(&7));
    /// assert_eq!(neighbors.right, Some(&5));
    /// assert_eq!(neighbors.downleft, Some(&11));
    /// assert_eq!(neighbors.down, Some(&8));
    /// assert_eq!(neighbors.downright, Some(&9));
    ///```
    pub fn all_around_neighbors<I: Index>(
        &self,
        index: I,
    ) -> Result<AllAroundNeighbor<'_, T>, GridError> {
        let index = index.grid_index(&self)?;
        Ok(AllAroundNeighbor {
            upleft: self.get_upleft(index),
            up: self.get_up(index),
            upright: self.get_upright(index),
            left: self.get_left(index),
            right: self.get_right(index),
            downleft: self.get_downleft(index),
            down: self.get_down(index),
            downright: self.get_downright(index),
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
            options: options.unwrap_or_default(),
        }
    }
    #[inline]
    pub(crate) fn origin(&self) -> Origin {
        self.options.origin.clone()
    }
}

pub(crate) fn row_number<T>(grid: &Grid<T>, index: usize) -> usize {
    index / grid.cols as usize
}

pub(crate) fn col_number<T>(grid: &Grid<T>, index: usize) -> usize {
    index % grid.cols as usize
}

pub(crate) fn row_start_index<T>(grid: &Grid<T>, index: usize) -> usize {
    row_number(grid, index) * grid.cols as usize
}

pub(crate) fn col_start_index<T>(grid: &Grid<T>, index: usize) -> usize {
    col_number(grid, index)
}

pub(crate) fn ceiling(a: usize, b: usize) -> usize {
    (a + b - 1) / b
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
            inverted_y: false,
            ..GridOptions::default()
        };
        let grid = Grid::new(vec, Some(gridoptions));
        grid.unwrap()
    }

    fn wrap_grid(wrap_x: bool, wrap_y: bool) -> Grid<i32> {
        let vec = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![9, 10, 11],
            vec![12, 13, 14],
        ];
        let gridoptions = GridOptions {
            wrap_x,
            wrap_y,
            neighbor_ybased: false,
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

        #[test]
        fn should_get_up_wrap() {
            let grid = wrap_grid(false, true);
            assert_eq!(grid.get_up((0, 1)), Some(&0i32));
            assert_eq!(grid.get_up((0, 0)), Some(&12i32));
            assert_eq!(grid.get_up((0, 2)), Some(&3i32));
        }

        #[test]
        fn should_get_down_wrap() {
            let grid = wrap_grid(false, true);
            assert_eq!(grid.get_down((0, 3)), Some(&12i32));
            assert_eq!(grid.get_down((0, 4)), Some(&0i32));
            assert_eq!(grid.get_down((0, 0)), Some(&3i32));
        }

        #[test]
        fn should_get_left_wrap() {
            let grid = wrap_grid(true, false);
            assert_eq!(grid.get_left((1, 0)), Some(&0i32));
            assert_eq!(grid.get_left((0, 0)), Some(&2i32));
            assert_eq!(grid.get_left((2, 0)), Some(&1i32));
        }

        #[test]
        fn should_get_right_wrap() {
            let grid = wrap_grid(true, false);
            assert_eq!(grid.get_right((1, 0)), Some(&2i32));
            assert_eq!(grid.get_right((2, 0)), Some(&0i32));
            assert_eq!(grid.get_right((0, 0)), Some(&1i32));
        }
        #[test]
        fn basic_quadrant() {
            let vec = vec![vec![0, 1], vec![2, 3]];

            let grid = Grid::new(vec, None).unwrap();
            assert_eq!(grid.nrant((0, 0), 1).unwrap(), 0);
            assert_eq!(grid.nrant((1, 0), 1).unwrap(), 0);
            assert_eq!(grid.nrant((0, 1), 1).unwrap(), 0);
            assert_eq!(grid.nrant((1, 1), 1).unwrap(), 0);

            assert_eq!(grid.nrant((0, 0), 2).unwrap(), 0);
            assert_eq!(grid.nrant((1, 0), 2).unwrap(), 1);
            assert_eq!(grid.nrant((0, 1), 2).unwrap(), 2);
            assert_eq!(grid.nrant((1, 1), 2).unwrap(), 3);
        }

        #[test]
        fn uneven_quadrant() {
            let vec = vec![vec![0, 1, 2], vec![3, 4, 5]];

            let grid = Grid::new(vec, None).unwrap();

            assert_eq!(grid.nrant((0, 0), 2).unwrap(), 0);
            assert_eq!(grid.nrant((1, 0), 2).unwrap(), 0);
            assert_eq!(grid.nrant((2, 0), 2).unwrap(), 1);
            assert_eq!(grid.nrant((0, 1), 2).unwrap(), 2);
            assert_eq!(grid.nrant((1, 1), 2).unwrap(), 2);
            assert_eq!(grid.nrant((2, 1), 2).unwrap(), 3);
        }

        #[test]
        fn nrant_start() {
            let vec = vec![vec![0, 1], vec![2, 3]];

            let grid = Grid::new(vec, None).unwrap();
            assert_eq!(grid.nrant_start(0, 1), 0);
            assert_eq!(grid.nrant_start(1, 1), 0);
            assert_eq!(grid.nrant_start(2, 1), 0);
            assert_eq!(grid.nrant_start(3, 1), 0);

            assert_eq!(grid.nrant_start(0, 2), 0);
            assert_eq!(grid.nrant_start(1, 2), 1);
            assert_eq!(grid.nrant_start(2, 2), 2);
            assert_eq!(grid.nrant_start(3, 2), 3);
        }

        #[test]
        fn uneven_quadrant_start() {
            let vec = vec![vec![0, 1, 2], vec![3, 4, 5]];

            let grid = Grid::new(vec, None).unwrap();

            assert_eq!(grid.nrant_start(0, 2), 0);
            assert_eq!(grid.nrant_start(1, 2), 0);
            assert_eq!(grid.nrant_start(2, 2), 2);
            assert_eq!(grid.nrant_start(3, 2), 3);
            assert_eq!(grid.nrant_start(4, 2), 3);
            assert_eq!(grid.nrant_start(5, 2), 5);
        }
    }

    mod row_iters {
        use super::*;

        #[test]
        fn should_return_none_outside_bounds() {
            let grid = center_grid();
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
            for cell in grid.row_iter_mut((0, 1)) {
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
            let grid = center_grid();
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
            for cell in grid.col_iter_mut((0, 1)) {
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
    mod all_around_neighbors {
        use super::*;

        #[test]
        fn test_all_around() {
            let vec = vec![
                vec![0, 1, 2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11],
                vec![12, 13, 14, 15],
                vec![16, 17, 18, 19],
            ];

            let gridoptions = GridOptions {
                origin: Origin::Center,
                inverted_y: false,
                ..GridOptions::default()
            };
            let grid = Grid::new(vec, Some(gridoptions)).expect("failed to import 2d vec");
            let neighbors = grid
                .all_around_neighbors((-2, 1))
                .expect("was not a valid coodinate"); // Neighbors of the item with 4 in it.
            assert_eq!(neighbors.upleft, None);
            assert_eq!(neighbors.up, Some(&0));
            assert_eq!(neighbors.upright, Some(&1));
            assert_eq!(neighbors.left, None);
            assert_eq!(neighbors.right, Some(&5));
            assert_eq!(neighbors.downleft, None);
            assert_eq!(neighbors.down, Some(&8));
            assert_eq!(neighbors.downright, Some(&9));
        }
    }
}
