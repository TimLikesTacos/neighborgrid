use crate::grid::Grid;

pub struct ColIter<'a, T> {
    pub(crate) slice: std::iter::StepBy<std::iter::Skip<std::slice::Iter<'a, T>>>,
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.slice.next()
    }
}

impl<'a, T> ColIter<'a, T> {
    pub(crate) fn new(grid: &'a Grid<T>, index: usize) -> ColIter<'a, T> {
        let col_start = crate::grid::col_start_index(grid, index);
        ColIter {
            slice: grid
                .items
                .iter()
                .skip(col_start)
                .step_by(grid.cols),
        }
    }

    pub(crate) fn noop() -> ColIter<'a, T> {
        ColIter {
            slice: [].iter().skip(0).step_by(1),
        }
    }
}

pub struct MutColIter<'a, T> {
    pub(crate) slice: std::iter::StepBy<std::iter::Skip<std::slice::IterMut<'a, T>>>,
}

impl<'a, T> Iterator for MutColIter<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.slice.next()
    }
}

impl<'a, T> MutColIter<'a, T> {
    pub(crate) fn new(grid: &'a mut Grid<T>, index: usize) -> MutColIter<'a, T> {
        let col_start = crate::grid::col_start_index(grid, index);
        MutColIter {
            slice: grid
                .items
                .iter_mut()
                .skip(col_start)
                .step_by(grid.cols),
        }
    }

    pub(crate) fn noop() -> MutColIter<'a, T> {
        MutColIter {
            slice: [].iter_mut().skip(0).step_by(1),
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

    mod col_iter_tests {
        use super::*;

        #[test]
        fn should_iter_over_col() {
            let grid = center_grid();
            let mut iter = ColIter::new(&grid, 3);
            assert_eq!(iter.next(), Some(&0));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&6));
            assert_eq!(iter.next(), Some(&9));
            assert_eq!(iter.next(), Some(&12));
            assert_eq!(iter.next(), None);

            let mut iter = ColIter::new(&grid, 1);
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), Some(&7));
            assert_eq!(iter.next(), Some(&10));
            assert_eq!(iter.next(), Some(&13));
            assert_eq!(iter.next(), None);

            let mut iter = ColIter::new(&grid, 2);
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), Some(&8));
            assert_eq!(iter.next(), Some(&11));
            assert_eq!(iter.next(), Some(&14));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn should_mut_iter_over_col() {
            let mut grid = center_grid();
            let mut iter = MutColIter::new(&mut grid, 3);
            assert_eq!(iter.next(), Some(&mut 0));
            assert_eq!(iter.next(), Some(&mut 3));
            assert_eq!(iter.next(), Some(&mut 6));
            assert_eq!(iter.next(), Some(&mut 9));
            assert_eq!(iter.next(), Some(&mut 12));
            assert_eq!(iter.next(), None);

            let iter = MutColIter::new(&mut grid, 3);
            for value in iter {
                *value += 1;
            }
            let mut iter = MutColIter::new(&mut grid, 3);
            assert_eq!(iter.next(), Some(&mut 1));
            assert_eq!(iter.next(), Some(&mut 4));
            assert_eq!(iter.next(), Some(&mut 7));
            assert_eq!(iter.next(), Some(&mut 10));
            assert_eq!(iter.next(), Some(&mut 13));
            assert_eq!(iter.next(), None);
        }
    }
}
