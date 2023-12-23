//! A diagonal matrix implementation where every element outside the diagonal is implicitly null.
//! It is often used when every entry in the matrix is 0 except for the diagonal.
use std::mem;

use crate::Entry;

#[derive(Debug, PartialEq, Clone)]
pub struct Diag {
    data: Vec<Entry>,
    n: usize,
    m: usize,
}

impl Diag {
    /// Returns the `N x N` identity matrix.
    pub fn ident(n: usize) -> Self {
        let data = vec![1.0; n];
        Self { data, n, m: n }
    }

    /// Initializes a new `N x M` matrix filled with zeros.
    pub fn zeros(n: usize, m: usize) -> Self {
        let data = vec![0.0; n.min(m)];
        Self { data, n, m }
    }

    /// Instanciates a new `Diag` type matrix with the given elements and shape.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mat = Diag::from((3, 5), [1.0, 2.0, 3.0]).unwrap();
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
    /// assert!(Diag::from((4, 5), [1.0]).is_ok());
    /// assert!(Diag::from((5, 5), [1.0, 2.0, 3.0, 4.0, 5.0]).is_ok());
    /// assert!(Diag::from((2, 3), [4.0, 5.0]).is_ok());
    ///
    /// // Invalid
    /// assert!(Diag::from((1, 2), [1.0, 2.0]).is_err());
    /// ```
    #[allow(unused_variables)]
    pub fn from<const S: usize>(
        shape @ (n, m): (usize, usize),
        elems: [Entry; S],
    ) -> Result<Self, &'static str> {
        let len = elems.len();
        let min = n.min(m);

        if len > min {
            return Err("Invalid quantity of elements");
        }

        let mut data = elems.to_vec();
        data.append(&mut vec![Entry::default(); min - len]);

        Ok(Self { data, n, m })
    }

    /// Returns a reference to the `Entry` at the given `idx: (i, j)`.
    /// If the given index is out of bounds returns the `None` variant.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mat = Diag::from((2, 3), [1.0, 2.0]).unwrap();
    /// assert_eq!(&0.0, mat.get((1, 0)).unwrap());
    /// assert_eq!(&2.0, mat.get((1, 1)).unwrap());
    /// assert!(mat.get((2, 0)).is_none());
    /// ```
    #[allow(unused_variables)]
    pub fn get(&self, idx @ (i, j): (usize, usize)) -> Option<&Entry> {
        self.data.get(i).map(|num| if i == j { num } else { &0.0 })
    }

    /// Returns a mutable reference to the `Entry` at the given `idx: :(i, j)`.
    /// If the given index is out of bounds returns the `None` variant.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mut mat = Diag::zeros(3, 2);
    ///
    /// if let Some(num) = mat.get_mut((0, 0)) {
    ///     *num = 1.0;
    /// }
    ///
    /// if let Some(_) = mat.get_mut((3, 2)) {
    ///     panic!("Out of bounds");
    /// }
    ///
    /// assert_eq!(1.0, mat[(0, 0)]);
    /// ```
    #[allow(unused_variables)]
    pub fn get_mut(&mut self, idx @ (i, j): (usize, usize)) -> Option<&mut Entry> {
        if i == j {
            self.data.get_mut(i)
        } else {
            None
        }
    }

    /// Sets `val` to the given `idx: (i, j)`in the matrix
    /// and returns the previous value.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mut mat = Diag::zeros(3, 3);
    /// let prev = mat.set((0, 0), 7.0);
    /// assert_eq!(7.0, mat[(0, 0)]);
    /// assert_eq!(Some(0.0), prev);
    ///
    /// let prev = mat.set((3, 3), 8.0);
    /// assert_eq!(None, prev);
    /// ````
    pub fn set(&mut self, idx: (usize, usize), val: Entry) -> Option<Entry> {
        self.get_mut(idx).map(|num| mem::replace(num, val))
    }

    /// Returns the shape of the matrix in the format `(rows, cols)`.
    pub fn shape(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn is_square(&self) -> bool {
        let (n, m) = self.shape();
        n == m
    }

    /// Resizes the matrix with a new shape. Fills every new entry with the value `0`.
    #[allow(unused_variables)]
    pub fn reshape(&mut self, shape @ (n, m): (usize, usize)) -> &mut Self {
        self.data.resize(n.min(m), 0.0);
        self.n = n;
        self.m = m;
        self
    }

    /// Computes the multiplication of the given matrix and `rhs` in-place.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mut mat = Diag::from((2, 3), [1.0, 2.0]).unwrap();
    /// mat.scalar_mul(2.0);
    ///
    /// let res = Diag::from((2, 3), [2.0, 4.0]).unwrap();
    /// assert_eq!(res, mat);
    /// ````
    pub fn scalar_mul(&mut self, rhs: Entry) -> &mut Self {
        self.apply(|e| e * rhs)
    }

    /// Applies the given function `f` to every entry in the matrix.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::diag::Diag;
    /// let mut mat = Diag::from((3, 3), [1.0, 2.0, 3.0]).unwrap();
    /// mat.apply(|e| e + 5.0);
    ///
    /// let res = Diag::from((3, 3), [6.0, 7.0, 8.0]).unwrap();
    /// assert_eq!(res, mat);
    /// ```
    pub fn apply<F: Fn(Entry) -> Entry>(&mut self, f: F) -> &mut Self {
        self.data.iter_mut().for_each(|e| *e = f(*e));
        self
    }

    /// Computes the determinant of the given matrix.
    pub fn det(&self) -> Option<Entry> {
        if !self.is_square() {
            return None;
        }

        Some(self.data.iter().fold(1.0, |a, b| a * b))
    }

    /// Inverts the given matrix in-place.
    pub fn inv(&mut self) -> Option<&mut Self> {
        if !self.is_square() {
            return None;
        }

        Some(self.apply(|e| if e != 0.0 { 1.0 / e } else { 0.0 }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ident() {
        let mat = Diag::ident(20);
        assert_eq!(vec![1.0; 20], mat.data);
        assert_eq!(20, mat.n);
        assert_eq!(20, mat.m);
    }

    #[test]
    fn zeros() {
        let mat = Diag::zeros(5, 9);
        assert_eq!(vec![0.0; 5], mat.data);
        assert_eq!(5, mat.n);
        assert_eq!(9, mat.m);
    }

    #[test]
    fn from() {
        let mat = Diag::from((4, 5), [1.0, 2.0, 3.0]).unwrap();
        assert_eq!(1.0, mat[(0, 0)]);
        assert_eq!(2.0, mat[(1, 1)]);
        assert_eq!(3.0, mat[(2, 2)]);
        assert_eq!(0.0, mat[(3, 3)]);
    }

    #[test]
    fn reshape() {
        let mut mat = Diag::from((4, 5), [1.0, 2.0, 3.0, 4.0]).unwrap();
        let res = mat.clone();
        mat.reshape(res.shape());
        assert_eq!(res, mat);

        let mut mat = Diag::from((3, 3), [1.0, 2.0, 3.0]).unwrap();
        let res = Diag::from((4, 4), [1.0, 2.0, 3.0]).unwrap();
        mat.reshape(res.shape());
        assert_eq!(res, mat);

        let mut mat = Diag::from((4, 4), [1.0, 2.0, 3.0, 4.0]).unwrap();
        let res = Diag::from((2, 2), [1.0, 2.0]).unwrap();
        mat.reshape(res.shape());
        assert_eq!(res, mat);
    }

    #[test]
    fn det() {
        let mat = Diag::ident(100);
        assert_eq!(Some(1.0), mat.det());

        let mat = Diag::from((3, 4), [1.0, 2.0, 3.0]).unwrap();
        assert_eq!(None, mat.det());

        let mat = Diag::from((4, 4), [1.0, 2.0, 3.0]).unwrap();
        assert_eq!(Some(0.0), mat.det());

        let mat = Diag::from((4, 4), [1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(Some(24.0), mat.det());
    }

    #[test]
    fn inv() {
        let mut mat = Diag::ident(100);
        mat.inv();
        assert_eq!(Diag::ident(100), mat);

        let mut mat = Diag::from((2, 2), [2.0, 4.0]).unwrap();
        mat.inv();
        assert_eq!(Diag::from((2, 2), [0.5, 0.25]).unwrap(), mat);

        let mut mat = Diag::from((5, 5), [2.0, 3.0, 4.0]).unwrap();
        mat.inv();
        assert_eq!(Diag::from((5, 5), [0.5, 1.0 / 3.0, 0.25]).unwrap(), mat);
    }
}
