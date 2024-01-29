use super::SparseImplTraits;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Sparse<T, const N: usize, const M: usize> {
    data: BTreeMap<(usize, usize), T>,
    // self.zero is used to return a reference to
    // the value 0 when the index is out of the diagonal.
    zero: T,
}

#[allow(private_bounds)]
impl<T: SparseImplTraits, const N: usize, const M: usize> Sparse<T, N, M> {
    /// Initializes a new `N x M` matrix filled with zeros.
    pub fn zeros() -> Self {
        let data = BTreeMap::new();
        let zero = T::from(0);
        Self { data, zero }
    }

    fn is_in_range(&self, (i, j): (usize, usize)) -> bool {
        i < N && j < M
    }

    /// Returns a reference to the entry at the given `idx: (i, j)`.
    /// If the given index is out of bounds returns the `None` variant.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::sparse::Sparse;
    /// let mut mat = Sparse::<f32, 2, 3>::zeros();
    /// mat[(0, 0)] = 1.0;
    /// mat[(1, 1)] = 2.0;
    ///
    /// assert_eq!(Some(&1.0), mat.get((0, 0)));
    /// assert_eq!(Some(&2.0), mat.get((1, 1)));
    /// assert_eq!(Some(&0.0), mat.get((0, 2)));
    /// assert_eq!(None, mat.get((2, 0)));
    /// ```
    pub fn get(&self, idx: (usize, usize)) -> Option<&T> {
        if !self.is_in_range(idx) {
            return None;
        }

        self.data.get(&idx).or_else(|| Some(&self.zero))
    }

    /// Returns a mutable reference to the entry at the given `idx: (i, j)`.
    /// If the given index is out of bounds returns the `None` variant.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::sparse::Sparse;
    /// let mut mat = Sparse::<f32, 3, 2>::zeros();
    ///
    /// if let Some(num) = mat.get_mut((0, 0)) {
    ///     *num = 1.0;
    /// }
    ///
    /// assert_eq!(1.0, mat[(0, 0)]);
    /// assert_eq!(None, mat.get_mut((3, 2)));
    /// ```
    pub fn get_mut(&mut self, idx: (usize, usize)) -> Option<&mut T> {
        if !self.is_in_range(idx) {
            return None;
        }

        Some(self.data.entry(idx).or_insert(T::from(0)))
    }

    /// Sets `val` to the given `idx: (i, j)` in the matrix
    /// and returns the previous value.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::sparse::Sparse;
    ///
    /// let mut mat = Sparse::<f32, 3, 3>::zeros();
    ///
    /// let prev = mat.set((0, 0), 7.0);
    /// assert_eq!(7.0, mat[(0, 0)]);
    /// assert_eq!(Some(0.0), prev);
    ///
    /// let prev = mat.set((3, 3), 8.0);
    /// assert_eq!(None, prev);
    /// ````
    pub fn set(&mut self, idx: (usize, usize), val: T) -> Option<T> {
        if !self.is_in_range(idx) {
            return None;
        }

        self.data.insert(idx, val).or_else(|| Some(T::from(0)))
    }

    /// Returns the shape of the matrix in the format `(rows, cols)`.
    pub fn shape(&self) -> (usize, usize) {
        (N, M)
    }

    pub fn is_square(&self) -> bool {
        N == M
    }

    /// Computes the multiplication of the given matrix and a scalar `rhs` in-place.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::sparse::Sparse;
    /// let mut mat: Sparse<f32, 5, 5> = (0..3).map(|i| ((i, i), 1.0)).collect();
    /// mat.scalar_mul(2.0);
    ///
    /// let res: Sparse<f32, 5, 5> = (0..3).map(|i| ((i, i), 2.0)).collect();
    /// assert_eq!(res, mat);
    /// ````
    pub fn scalar_mul(&mut self, rhs: T) -> &mut Self {
        self.apply(|e| e * rhs)
    }

    /// Applies the given function `f` to every entry in the matrix.
    ///
    /// # Precaution
    ///
    /// This method will only affect entries that are already initialized.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::sparse::Sparse;
    ///
    /// let mut mat: Sparse<f32, 2, 2> = (0..2).map(|i| ((i, i), 1.0)).collect();
    /// mat.apply(|e| e + 5.0);
    ///
    /// let res: Sparse<f32, 2, 2> = (0..2).map(|i| ((i, i), 6.0)).collect();
    /// assert_eq!(res, mat);
    /// ```
    pub fn apply<F: Fn(T) -> T>(&mut self, f: F) -> &mut Self {
        self.data.values_mut().for_each(|e| *e = f(*e));
        self
    }

    /// Computes the determinant of the given matrix.
    ///
    /// # Example
    /// ```
    /// use mat_lib::sparse::Sparse;
    /// let mat: Sparse<i32, 4, 4> = (0..4).map(|i| ((3 - i, i), i + 1)).collect();
    /// assert_eq!(Some(24), mat.det());
    /// ```
    pub fn det(&self) -> Option<T> {
        todo!()
    }

    /// Inverts the given matrix in-place.
    pub fn inv(&mut self) -> Option<&mut Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_iter() {
        let iter = vec![((0, 0), 1.1), ((1, 1), 2.2), ((2, 2), 3.3)];
        let mat: Sparse<f32, 3, 3> = iter.into_iter().collect();
        println!("mat:\n{mat}");
    }
}
