use std::cmp::Ordering;
use std::ops::Index;
use bisection::bisect_left_by;

trait Resort<T> where Self: Index<usize, Output = T> {
    fn resort_element_by<F>(&mut self, index: usize, f: F) -> usize
        where F: FnMut(&T, &T) -> Ordering;
    /// Sort again already sorted sequence after the element at `index` changed.
    /// Returns new index.
    fn resort_element(&mut self, index: usize) -> usize
        where Self::Output: Ord
    {
        self.resort_element_by(index, |e, value| T::cmp(e, &value))
    }
}

trait InsertSorted<T> where Self: Index<usize, Output = T> {
    fn insert_sorted_by<F>(&mut self, value: T, f: F) -> usize
        where F: FnMut(&T, &T) -> Ordering;
    fn insert_sorted(&mut self, value: T) -> usize
        where Self::Output: Ord
    {
        self.insert_sorted_by(value, |e, value| T::cmp(e, &value))
    }
}

impl<T: Ord> InsertSorted<T> for Vec<T> {
    fn insert_sorted_by<F>(&mut self, value: T, mut f: F) -> usize
        where F: FnMut(&T, &T) -> Ordering
    {
        let index = bisect_left_by(self.as_slice(), |e| f(e,&value));
        self.insert(index, value);
        index
    }
}

impl<T: Ord> Resort<T> for Vec<T> {
    // TODO: It can be made more efficient.
    fn resort_element_by<F>(&mut self, index: usize, f: F) -> usize
        where F: FnMut(&T, &T) -> Ordering
    {
        let value = self.remove(index);
        self.insert_sorted_by(value, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::Resort;

    #[test]
    fn decrease() {
        let mut v1 = vec![0, 1, 2];
        v1[1] = -1;
        v1.resort_element(1);
        assert_eq!(v1, [-1, 0, 2]);
    }

    #[test]
    fn increase() {
        let mut v1 = vec![0, 1, 2];
        v1[1] = 3;
        v1.resort_element(1);
        assert_eq!(v1, [0, 2, 3]);
    }

    #[test]
    fn no_change() {
        let mut v1 = vec![0, 1, 2];
        v1.resort_element(1);
        assert_eq!(v1, [0, 1, 2]);
    }
}
