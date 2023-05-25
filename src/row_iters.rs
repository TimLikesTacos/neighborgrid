use crate::grid::Grid;
use crate::index::Index;

pub struct RowIter<'a, T> {
    pub(crate) slice: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.slice.next()
    }
}

impl<'a, T> RowIter<'a, T> {
    pub(crate) fn new(grid: &'a Grid<T>, index: usize) -> RowIter<'a, T> {
        let row_start = crate::grid::row_start_index(&grid, index);
        let slice = &grid.items[row_start..row_start + grid.cols as usize];
        RowIter {
            slice: slice.iter(),
        }
    }

    pub(crate) fn noop() -> RowIter<'a, T> {
        RowIter { slice: [].iter() }
    }
}

pub struct MutRowIter<'a, T> {
    pub(crate) slice: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for MutRowIter<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.slice.next()
    }
}

impl<'a, T> MutRowIter<'a, T> {
    pub(crate) fn new(grid: &'a mut Grid<T>, index: usize) -> MutRowIter<'a, T> {
        let row_start = crate::grid::row_start_index(&grid, index);
        let slice = &mut grid.items[row_start..row_start + grid.cols as usize];
        MutRowIter {
            slice: slice.iter_mut(),
        }
    }

    pub(crate) fn noop() -> MutRowIter<'a, T> {
        MutRowIter {
            slice: [].iter_mut(),
        }
    }
}

#[cfg(test)]
mod iter_tests {
    use super::*;
    use crate::grid::{GridOptions, Origin};

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

    mod row_iter_tests {
        use super::*;

        #[test]
        fn should_iter_over_row() {
            let grid = center_grid();
            let mut iter = RowIter::new(&grid, 3);
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), None);

            let mut iter = RowIter::new(&grid, 4);
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), None);

            let mut iter = RowIter::new(&grid, 5);
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), None);

            let mut iter = RowIter::new(&grid, 12);
            assert_eq!(iter.next(), Some(&12));
            assert_eq!(iter.next(), Some(&13));
            assert_eq!(iter.next(), Some(&14));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn should_mut_iter_over_row() {
            let mut grid = center_grid();
            let mut iter = MutRowIter::new(&mut grid, 3);
            assert_eq!(iter.next(), Some(&mut 3));
            assert_eq!(iter.next(), Some(&mut 4));
            assert_eq!(iter.next(), Some(&mut 5));
            assert_eq!(iter.next(), None);

            let mut iter = MutRowIter::new(&mut grid, 3);
            for value in iter {
                *value += 1;
            }
            let mut iter = MutRowIter::new(&mut grid, 3);
            assert_eq!(iter.next(), Some(&mut 4));
            assert_eq!(iter.next(), Some(&mut 5));
            assert_eq!(iter.next(), Some(&mut 6));
            assert_eq!(iter.next(), None);
        }
    }
}
// pub(crate) struct GridIter<'a, T, C>
// where
//     C: HouseCoord,
// {
//     pub(crate) grid: &'a Grid<T>,
//     pub(crate) location: Option<C>,
// }
//
// pub(crate) struct MutGridIter<'a, T, C>
// where
//     C: HouseCoord,
// {
//     pub(crate) grid: &'a mut Grid<T>,
//     pub(crate) location: Option<C>,
// }
//
// impl<'a, T, C: HouseCoord> Iterator for GridIter<'a, T, C> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<&'a T> {
//         if let Some(current_pos) = self.location {
//             self.location = current_pos.inc();
//             Some(&self.grid.items[current_pos.to_usize()])
//         } else {
//             None
//         }
//     }
// }
//
// impl<'a, T, C: HouseCoord> Iterator for MutGridIter<'a, T, C> {
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<&'a mut T> {
//         if let Some(current_pos) = self.location {
//             self.location = current_pos.inc();
//             let ptr: *mut T = &mut self.grid.items[current_pos.to_usize()];
//
//             // self.grid.items should not reallocate when using this iterator and therefore safe
//             unsafe { Some(&mut *ptr) }
//         } else {
//             None
//         }
//     }
// }
