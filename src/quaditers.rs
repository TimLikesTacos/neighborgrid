use crate::grid::ceiling;
use crate::Grid;
pub struct NrantIterator<'a, T> {
    grid: &'a Grid<T>,
    current: usize,
    start: usize,
    rwidth: usize,
    rheight: usize,
}

impl<'a, T> NrantIterator<'a, T> {
    pub(crate) fn new(grid: &'a Grid<T>, divisor: usize, index: usize) -> Self {
        let rwidth = ceiling(grid.columns(), divisor);
        let rheight = ceiling(grid.rows(), divisor);
        let start = grid.nrant_start(index, divisor);
        Self {
            grid,
            current: 0,
            start,
            rwidth,
            rheight,
        }
    }

    /// Creates a condition that will appear that the iterator has ended.
    pub(crate) fn noop(grid: &'a Grid<T>) -> Self {
        NrantIterator {
            grid,
            current: 100,
            start: 0,
            rwidth: 1,
            rheight: 1,
        }
    }
}

impl<'a, T> Iterator for NrantIterator<'a, T> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.rwidth * self.rheight {
            return None;
        }
        let row_offset = self.current / self.rwidth;
        let col_offset = self.current % self.rwidth;
        // Check for overrunning the grid
        if col_offset + (self.start % self.grid.columns()) >= self.grid.columns() {
            self.current += 1;
            return Some(None);
        }
        let index = self.start + row_offset * self.grid.columns() + col_offset;
        self.current += 1;
        return Some(self.grid.get(index));
    }
}

#[cfg(test)]
mod nrant_iterator_tests {
    use super::*;

    #[test]
    fn should_iterate_over_quad() {
        let vec = vec![vec![0, 1, 2], vec![3, 4, 5]];

        let grid = Grid::new(vec, None).unwrap();

        let mut iter = NrantIterator::new(&grid, 2, 0);
        assert_eq!(iter.next(), Some(Some(&0)));
        assert_eq!(iter.next(), Some(Some(&1)));
        assert_eq!(iter.next(), None);

        let mut iter = NrantIterator::new(&grid, 2, 1);
        assert_eq!(iter.next(), Some(Some(&0)));
        assert_eq!(iter.next(), Some(Some(&1)));
        assert_eq!(iter.next(), None);

        let mut iter = NrantIterator::new(&grid, 2, 2);
        assert_eq!(iter.next(), Some(Some(&2)));
        assert_eq!(iter.next(), Some(None));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ssudoku_test() {
        let mut vec = vec![];

        for i in 1..=81 {
            vec.push(i);
        }

        let grid = Grid::new_from_1d(vec, 9, 9, None).unwrap();

        let mut iter = NrantIterator::new(&grid, 3, 10);
        assert_eq!(iter.next(), Some(Some(&1)));
        assert_eq!(iter.next(), Some(Some(&2)));
        assert_eq!(iter.next(), Some(Some(&3)));
        assert_eq!(iter.next(), Some(Some(&10)));
        assert_eq!(iter.next(), Some(Some(&11)));
        assert_eq!(iter.next(), Some(Some(&12)));
        assert_eq!(iter.next(), Some(Some(&19)));
        assert_eq!(iter.next(), Some(Some(&20)));
        assert_eq!(iter.next(), Some(Some(&21)));
        assert_eq!(iter.next(), None);

        let mut iter = NrantIterator::new(&grid, 3, 80);
        assert_eq!(iter.next(), Some(Some(&61)));
        assert_eq!(iter.next(), Some(Some(&62)));
        assert_eq!(iter.next(), Some(Some(&63)));
        assert_eq!(iter.next(), Some(Some(&70)));
        assert_eq!(iter.next(), Some(Some(&71)));
        assert_eq!(iter.next(), Some(Some(&72)));
        assert_eq!(iter.next(), Some(Some(&79)));
        assert_eq!(iter.next(), Some(Some(&80)));
        assert_eq!(iter.next(), Some(Some(&81)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_noop() {
        let mut vec = vec![];

        for i in 1..=81 {
            vec.push(i);
        }

        let grid = Grid::new_from_1d(vec, 9, 9, None).unwrap();
        let mut iter = grid.quadrant_iter((10, 10));
        assert_eq!(iter.next(), None);
    }
}
