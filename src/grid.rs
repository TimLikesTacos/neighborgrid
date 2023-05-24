use crate::error::GridError;
use crate::index::Index;
use crate::intogrid::IntoGrid;
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub(crate) items: Vec<T>,
    pub(crate) rows: i32,
    pub(crate) cols: i32,
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

    pub fn iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = &'a T> + 'b {
        self.items.iter()
    }

    pub fn iter_mut<'b, 'a: 'b>(&'a mut self) -> impl Iterator<Item = &'a mut T> + 'b {
        self.items.iter_mut()
    }

    pub(crate) fn create(
        items: Vec<T>,
        rows: i32,
        cols: i32,
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

pub(crate) fn row_start_index<T>(grid: &Grid<T>, index: usize) -> usize {
    row_number(grid, index) * grid.cols as usize
}

pub(crate) fn col_start_index<T>(grid: &Grid<T>, index: usize) -> usize {
    col_number(grid, index)
}

// impl<S: Clone + PartialEq + Send + Sync + 'static> Rotate for &Grid<S> {
//     type PuzzleInp = Grid<S>;
//
//     fn cwrotate(self, quarter_rotations: usize) -> Res<Self::PuzzleInp> {
//         let max = self.size.house_size();
//         let mut ret: Vec<_> = self.iter().cloned().collect();
//         for _ in 0..quarter_rotations {
//             for x in 0..max / 2 {
//                 for y in x..max - 1 - x {
//                     let ul = Coord::new(x, y, self.size).to_usize();
//                     let ur = Coord::new(y, max - 1 - x, self.size).to_usize();
//                     let ll = Coord::new(max - 1 - y, x, self.size).to_usize();
//                     let lr = Coord::new(max - 1 - x, max - 1 - y, self.size).to_usize();
//                     ret.swap(ul, ll);
//                     ret.swap(ll, lr);
//                     ret.swap(lr, ur);
//                 }
//             }
//         }
//         Ok(Grid::new(ret))
//     }
// }

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
        assert_eq!(grid.cols, i32::from(u16::MAX));

        let vec = vec![vec![1; 1000]; u16::MAX as usize];
        let grid = vec.into_grid()?;
        assert_eq!(grid.rows, u16::MAX as i32);
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
}
