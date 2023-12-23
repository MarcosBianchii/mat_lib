//! A dense matrix implementation where every element is explicitly stored in memeory.
//! It is often used when majority of entries are non-zero for computing operations
//! between matrices such as addition and multiplication.
use rand::Rng;
use std::mem;
pub use std::str::FromStr;

use crate::Entry;

#[derive(Debug, PartialEq)]
pub struct Dense {
    data: Vec<Entry>,
    n: usize,
    m: usize,
}

impl FromStr for Dense {
    type Err = &'static str;
    /// Create a `Dense` matrix from a string
    ///
    /// # Syntax
    ///
    /// Numbers are read as `Entry` type numbers and lines are separated by ';'.
    /// Matrices should have the same number of entries throughout every row to be valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use mat_lib::dense::*;
    ///
    /// // Valid
    /// assert!(Dense::from_str("1.2 2.3; 3.4 4.5").is_ok());
    /// assert!(Dense::from_str("1 2 3; 4 5 6").is_ok());
    /// assert!(Dense::from_str("1 ; 2; 3 ;4").is_ok());
    /// assert!(Dense::from_str("1 2 ; 3 4").is_ok());
    /// assert!(Dense::from_str("1").is_ok());
    ///
    /// // Invalid
    /// assert!(Dense::from_str("1; 2 3; 4").is_err());
    /// assert!(Dense::from_str("").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (data, n, m) = super::parse::mat_from_str(s, ";")?;
        Ok(Self { data, n, m })
    }
}

impl Dense {
    /// Initializes a new `N x M` matrix filled with zeros.
    pub fn zeros(n: usize, m: usize) -> Self {
        let data = vec![0.0; n * m];
        Self { data, n, m }
    }

    /// Initializes a new `N x M` matrix filled with random values.
    pub fn rand(n: usize, m: usize) -> Self {
        let mut rand_gen = rand::thread_rng();
        let data = (0..n * m).map(|_| rand_gen.gen()).collect();
        Self { data, n, m }
    }

    /// Returns a reference to the `Entry` at the given `idx: (i, j)`.
    /// If the given index is out of bounds returns the `None` variant.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::dense::*;
    /// let mat = Dense::from_str("1 2 3; 4 5 6").unwrap();
    /// assert_eq!(&4.0, mat.get((1, 0)).unwrap());
    /// assert!(mat.get((2, 0)).is_none());
    /// ```
    #[allow(unused_variables)]
    pub fn get(&self, idx @ (i, j): (usize, usize)) -> Option<&Entry> {
        self.data.get(i * self.m + j)
    }

    /// Returns a mutable reference to the `Entry` at the given `idx: :(i, j)`.
    /// If the given index is out of bounds returns the `None` variant.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::dense::*;
    /// let mut mat = Dense::zeros(3, 2);
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
        self.data.get_mut(i * self.m + j)
    }

    /// Sets `val` to the given `idx: (i, j)`in the matrix
    /// and returns the previous value.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::dense::*;
    /// let mut mat = Dense::zeros(3, 3);
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
        self.data.resize(n * m, 0.0);
        self.n = n;
        self.m = m;
        self
    }

    /// Computes the multiplication of the given matrix and `rhs` in-place.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::dense::*;
    /// let mut mat = Dense::from_str("1 2 3").unwrap();
    /// mat.scalar_mul(2.0);
    ///
    /// let res = Dense::from_str("2 4 6").unwrap();
    /// assert_eq!(res, mat);
    /// ````
    pub fn scalar_mul(&mut self, rhs: Entry) -> &mut Self {
        self.apply(|e| e * rhs)
    }

    /// Applies the given function `f` to every entry in the matrix.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::dense::*;
    /// let mut mat = Dense::from_str("1; 2; 3").unwrap();
    /// mat.apply(|e| e + 5.0);
    ///
    /// let res = Dense::from_str("6; 7; 8").unwrap();
    /// assert_eq!(res, mat);
    /// ```
    pub fn apply<F: Fn(Entry) -> Entry>(&mut self, f: F) -> &mut Self {
        self.data.iter_mut().for_each(|e| *e = f(*e));
        self
    }

    pub fn det(&self) -> Option<Entry> {
        todo!()
    }

    pub fn inv(&mut self) -> Option<&mut Self> {
        todo!();
    }
}
