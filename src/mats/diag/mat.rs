//! A diagonal matrix implementation where every element outside the diagonal is implicitly null.
//! It is often used when every entry in the matrix is 0 except for the diagonal.
//! This implementation is more efficient than the dense matrix implementation for this use case.
use super::DiagImplTraits;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::mem;

#[derive(Debug, PartialEq)]
pub struct Diag<T, const N: usize, const M: usize> {
    data: Vec<T>,
    // self.zero is used to return a reference to
    // the value 0 when the index is out of the diagonal.
    zero: T,
}

#[allow(private_bounds)]
impl<T: DiagImplTraits, const N: usize, const M: usize> Diag<T, N, M> {
    fn with_value(val: T) -> Self {
        let zero = T::from(0);
        let data = vec![val; N.min(M)];
        Self { data, zero }
    }

    /// Returns the `N x M` identity matrix.
    pub fn ident() -> Self {
        Self::with_value(T::from(1))
    }

    /// Initializes a new `N x M` matrix filled with zeros.
    pub fn zeros() -> Self {
        Self::with_value(T::from(0))
    }

    /// Initializes a new `N x M` where the diagonal is filled with random values.
    pub fn rand() -> Self
    where
        Standard: Distribution<T>,
    {
        let zero = T::from(0);
        let mut rand_gen = rand::thread_rng();
        let data = (0..N.min(M)).map(|_| rand_gen.gen()).collect();
        Self { data, zero }
    }

    /// Instanciates a new `Diag` type matrix with the given elements and shape.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mat = Diag::<f32, 3, 5>::from([1.0, 2.0, 3.0]).unwrap();
    /// assert_eq!(1.0, mat[(0, 0)]);
    /// assert_eq!(2.0, mat[(1, 1)]);
    /// assert_eq!(3.0, mat[(2, 2)]);
    /// ```
    /// # Errors
    /// This method will return the Err variant if len of `elems` is greater than MIN(n, m).
    /// If len of `elems` is less than MIN(n, m), the rest of the data will be filled with `0`.
    ///
    /// ## Example
    /// ```
    /// use mat_lib::diag::Diag;
    ///
    /// // Valid
    /// assert!(Diag::<f32, 4, 5>::from([1.0]).is_ok());
    /// assert!(Diag::<f32, 5, 5>::from([1.0, 2.0, 3.0, 4.0, 5.0]).is_ok());
    /// assert!(Diag::<f32, 2, 3>::from([4.0, 5.0]).is_ok());
    ///
    /// // Invalid
    /// assert!(Diag::<f32, 1, 2>::from([1.0, 2.0]).is_err());
    /// ```
    #[allow(unused_variables)]
    pub fn from<const S: usize>(elems: [T; S]) -> Result<Self, &'static str> {
        let min = N.min(M);

        if S > min {
            return Err("Invalid quantity of elements");
        }

        let mut data = elems.to_vec();
        let zero = T::from(0);

        data.append(&mut vec![zero; min - S]);
        Ok(Self { data, zero })
    }

    /// Returns a reference to the `Entry` at the given `idx: (i, j)`.
    /// If the given index is out of bounds returns the `None` variant.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mat = Diag::<f32, 2, 3>::from([1.0, 2.0]).unwrap();
    /// assert_eq!(Some(&0.0), mat.get((1, 0)));
    /// assert_eq!(Some(&2.0), mat.get((1, 1)));
    /// assert!(mat.get((2, 0)).is_none());
    /// ```
    #[allow(unused_variables)]
    pub fn get(&self, idx @ (i, j): (usize, usize)) -> Option<&T> {
        self.data
            .get(i)
            .map(|num| if i == j { num } else { &self.zero })
    }

    /// Returns a mutable reference to the `Entry` at the given `idx: :(i, j)`.
    /// If the given index is out of bounds returns the `None` variant.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mut mat = Diag::<f32, 3, 2>::zeros();
    ///
    /// if let Some(num) = mat.get_mut((0, 0)) {
    ///     *num = 1.0;
    /// }
    ///
    /// if mat.get_mut((3, 2)).is_some() {
    ///     panic!("Out of bounds");
    /// }
    ///
    /// assert_eq!(1.0, mat[(0, 0)]);
    /// ```
    #[allow(unused_variables)]
    pub fn get_mut(&mut self, idx @ (i, j): (usize, usize)) -> Option<&mut T> {
        (i == j).then(|| self.data.get_mut(i)).flatten()
    }

    /// Sets `val` to the given `idx: (i, j)`in the matrix
    /// and returns the previous value.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mut mat = Diag::<f32, 3, 3>::zeros();
    /// let prev = mat.set((0, 0), 7.0);
    /// assert_eq!(7.0, mat[(0, 0)]);
    /// assert_eq!(Some(0.0), prev);
    ///
    /// let prev = mat.set((3, 3), 8.0);
    /// assert_eq!(None, prev);
    /// ````
    pub fn set(&mut self, idx: (usize, usize), val: T) -> Option<T> {
        self.get_mut(idx).map(|num| mem::replace(num, val))
    }

    /// Returns the shape of the matrix in the format `(rows, cols)`.
    pub fn shape(&self) -> (usize, usize) {
        (N, M)
    }

    pub fn is_square(&self) -> bool {
        N == M
    }

    /// Computes the multiplication of the given matrix and `rhs` in-place.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mut mat = Diag::<f32, 2, 3>::from([1.0, 2.0]).unwrap();
    /// mat.scalar_mul(2.0);
    ///
    /// let res = Diag::<f32, 2, 3>::from([2.0, 4.0]).unwrap();
    /// assert_eq!(res, mat);
    /// ````
    pub fn scalar_mul(&mut self, rhs: T) -> &mut Self {
        self.apply(|e| e * rhs)
    }

    /// Applies the given function `f` to every entry in the matrix.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mut mat = Diag::<f32, 3, 3>::from([1.0, 2.0, 3.0]).unwrap();
    /// mat.apply(|e| e + 5.0);
    ///
    /// let res = Diag::<f32, 3, 3>::from([6.0, 7.0, 8.0]).unwrap();
    /// assert_eq!(res, mat);
    /// ```
    pub fn apply<F: Fn(T) -> T>(&mut self, f: F) -> &mut Self {
        self.data.iter_mut().for_each(|e| *e = f(*e));
        self
    }

    /// Computes the determinant of the given matrix.
    pub fn det(&self) -> Option<T> {
        if !self.is_square() {
            return None;
        }

        let one = T::from(1);
        Some(self.data.iter().fold(one, |acc, &e| acc * e))
    }

    /// Inverts the given matrix in-place.
    pub fn inv(&mut self) -> Option<&mut Self> {
        if !self.is_square() {
            return None;
        }

        let zero = T::from(0);
        let one = T::from(1);
        Some(self.apply(|e| if e != zero { one / e } else { zero }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ident() {
        let mat = Diag::<_, 20, 20>::ident();
        assert_eq!(vec![1.0; 20], mat.data);

        let (n, m) = mat.shape();
        assert_eq!(20, n);
        assert_eq!(20, m);
    }

    #[test]
    fn zeros() {
        let mat = Diag::<_, 5, 9>::zeros();
        assert_eq!(vec![0.0; 5], mat.data);

        let (n, m) = mat.shape();
        assert_eq!(5, n);
        assert_eq!(9, m);
    }

    #[test]
    fn from() {
        let mat = Diag::<_, 4, 5>::from([1.0, 2.0, 3.0]).unwrap();
        assert_eq!(1.0, mat[(0, 0)]);
        assert_eq!(2.0, mat[(1, 1)]);
        assert_eq!(3.0, mat[(2, 2)]);
        assert_eq!(0.0, mat[(3, 3)]);
    }

    #[test]
    fn det() {
        let mat = Diag::<_, 100, 100>::ident();
        assert_eq!(Some(1.0), mat.det());

        let mat = Diag::<_, 3, 4>::from([1.0, 2.0, 3.0]).unwrap();
        assert_eq!(None, mat.det());

        let mat = Diag::<_, 4, 4>::from([1.0, 2.0, 3.0]).unwrap();
        assert_eq!(Some(0.0), mat.det());

        let mat = Diag::<_, 4, 4>::from([1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(Some(24.0), mat.det());
    }

    #[test]
    fn inv() {
        let mut mat = Diag::<u8, 100, 100>::ident();
        mat.inv();
        assert_eq!(Diag::ident(), mat);

        let mut mat = Diag::<_, 2, 2>::from([2.0, 4.0]).unwrap();
        mat.inv();
        assert_eq!(Diag::<_, 2, 2>::from([0.5, 0.25]).unwrap(), mat);

        let mut mat = Diag::<_, 5, 5>::from([2.0, 3.0, 4.0]).unwrap();
        mat.inv();
        assert_eq!(Diag::<_, 5, 5>::from([0.5, 1.0 / 3.0, 0.25]).unwrap(), mat);
    }
}
