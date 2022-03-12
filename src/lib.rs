use std::cmp::Ordering;
use std::ops::Index;
use bisection::bisect_left_by;

trait Resort<T> where Self: Index<usize, Output = T> {
    /// Sort again already sorted sequence after the element at `index` changed.
    fn resort_element(&mut self, index: usize)
        where Self::Output: Ord + Sized
    {
        self.resort_element_by(index, |e, value| T::cmp(e, &value));
    }
    fn resort_element_by<F>(&mut self, index: usize, f: F)
        where F: FnMut(&T, &T) -> Ordering;
}

impl<T: Ord> Resort<T> for Vec<T> {
    // TODO: It can be made more efficient.
    fn resort_element_by<F>(&mut self, index: usize, mut f: F)
        where F: FnMut(&T, &T) -> Ordering
    {
        let value = self.remove(index);
        let new_index = bisect_left_by(self.as_slice(), |e| f(e, &value));
        self.insert(new_index, value);
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
