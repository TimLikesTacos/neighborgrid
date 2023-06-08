/// Represents (0,1), (0,1), (-1,0), (1,0) relative values to the specific point
#[derive(Debug, Clone, PartialEq)]
pub struct XyNeighbor<'a, T> {
    pub up: Option<&'a T>,
    pub left: Option<&'a T>,
    pub right: Option<&'a T>,
    pub down: Option<&'a T>,
}

impl<'a, T> XyNeighbor<'a, T> {
    /// Returns an iterator that returns an `Option<Option<T>>`.  The outer option is for the use with the iterator, so any loop knows when to stop.  The inner
    /// `Option` is if there is a neighbor in that position.  Using this around a cell on the edge of the grid will return some inner `None`s.
    ///
    /// Follows top to bottom, left to right.  So up (positive y value), left, right, down.
    pub fn iter(&self) -> XyNeighIterator<Option<&T>> {
        XyNeighIterator {
            refs: [&self.up, &self.left, &self.right, &self.down],
            current: 0,
        }
    }
}

pub struct XyNeighIterator<'a, V> {
    refs: [&'a V; 4],
    current: usize,
}

impl<'a, V> Iterator for XyNeighIterator<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= 4 {
            None
        } else {
            let ret = self.refs[self.current];
            self.current += 1;
            Some(ret)
        }
    }
}

pub struct AllAroundNeighbor<'a, T> {
    pub upleft: Option<&'a T>,
    pub up: Option<&'a T>,
    pub upright: Option<&'a T>,
    pub left: Option<&'a T>,
    pub right: Option<&'a T>,
    pub downleft: Option<&'a T>,
    pub down: Option<&'a T>,
    pub downright: Option<&'a T>,
}

impl<'a, T> AllAroundNeighbor<'a, T> {
    /// Returns an iterator that returns an `Option<Option<T>>`.  The outer option is for the use with the iterator, so any loop knows when to stop.  The inner
    /// `Option` is if there is a neighbor in that position.  Using this around a cell on the edge of the grid will return some inner `None`s.
    ///
    /// Follows top to bottom, left to right.  So upleft (positive y value), up, upright, left, right, downleft, down, downright.
    pub fn iter(&self) -> AllAroundNeighIterator<Option<&T>> {
        AllAroundNeighIterator {
            refs: [
                &self.upleft,
                &self.up,
                &self.upright,
                &self.left,
                &self.right,
                &self.downleft,
                &self.down,
                &self.downright,
            ],
            current: 0,
        }
    }
}

pub struct AllAroundNeighIterator<'a, V> {
    refs: [&'a V; 8],
    current: usize,
}

impl<'a, V> Iterator for AllAroundNeighIterator<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= 8 {
            None
        } else {
            let ret = self.refs[self.current];
            self.current += 1;
            Some(ret)
        }
    }
}

#[cfg(test)]
mod xyneightbor_tests {
    use super::*;

    #[test]
    fn xyneightbor_test() {
        let neigh = XyNeighbor {
            up: Some(&1),
            left: None,
            right: Some(&3),
            down: Some(&4),
        };

        let mut iter = neigh.iter();
        assert_eq!(iter.next(), Some(&Some(&1)));
        assert_eq!(iter.next(), Some(&None));
        assert_eq!(iter.next(), Some(&Some(&3)));
        assert_eq!(iter.next(), Some(&Some(&4)));
        assert_eq!(iter.next(), None);
    }

    fn xyneightbor_intoiter_test() {
        let neigh = XyNeighbor {
            up: Some(&1),
            left: None,
            right: Some(&3),
            down: Some(&4),
        };

        let mut iter = neigh.iter();
        assert_eq!(iter.next(), Some(&Some(&1)));
        assert_eq!(iter.next(), Some(&None));
        assert_eq!(iter.next(), Some(&Some(&3)));
        assert_eq!(iter.next(), Some(&Some(&4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn all_around_neightbor_test() {
        let neigh = AllAroundNeighbor {
            upleft: Some(&1),
            up: Some(&2),
            upright: None,
            left: Some(&3),
            right: Some(&4),
            downleft: None,
            down: Some(&5),
            downright: Some(&6),
        };

        let mut iter = neigh.iter();
        assert_eq!(iter.next(), Some(&Some(&1)));
        assert_eq!(iter.next(), Some(&Some(&2)));
        assert_eq!(iter.next(), Some(&None));
        assert_eq!(iter.next(), Some(&Some(&3)));
        assert_eq!(iter.next(), Some(&Some(&4)));
        assert_eq!(iter.next(), Some(&None));
        assert_eq!(iter.next(), Some(&Some(&5)));
        assert_eq!(iter.next(), Some(&Some(&6)));
        assert_eq!(iter.next(), None);
    }
}
