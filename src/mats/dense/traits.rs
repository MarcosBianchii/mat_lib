pub use std::str::FromStr;
use std::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
};

use super::mat::Dense;
use crate::Entry;

impl Index<(usize, usize)> for Dense {
    type Output = Entry;
    /// Returns the entry at `idx: (i, j)`.
    ///
    /// # Usage
    ///
    /// ```
    /// use mat_lib::dense::*;
    /// let mat = Dense::from_str("1 2 3; 4 5 6").unwrap();
    /// assert_eq!(2.0, mat[(0, 1)]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `idx` is out of bounds.
    ///
    /// ```should_panic
    /// use mat_lib::dense::*;
    /// let mat = Dense::from_str("1 2; 3 4").unwrap();
    /// let panic = mat[(2, 2)];
    /// ```
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        self.get(idx).expect("Index out of range")
    }
}

impl IndexMut<(usize, usize)> for Dense {
    /// Returns a mutable reference to the entry at `idx: (i, j)`.
    ///
    /// # Usage
    /// ```
    /// use mat_lib::dense::*;
    ///
    /// let mut mat = Dense::from_str("1 1; 1 1").unwrap();
    /// mat[(0, 0)] = 2.0;
    /// mat[(0, 1)] = 3.0;
    ///
    /// let res = Dense::from_str("2 3; 1 1").unwrap();
    /// assert_eq!(res, mat);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `idx` is out of bounds.
    ///
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        self.get_mut(idx).expect("IndexMut out of range")
    }
}

impl Display for Dense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (n, m) = self.shape();
        let mut rows = Vec::with_capacity(n);

        let len = 7;
        for i in 0..n {
            let mut row = String::with_capacity(len * m + 3);
            row.push('[');
            for j in 0..m {
                let num = self[(i, j)];
                let fmt = format!(" {num:.4}");
                row.push_str(&fmt[..7]);
            }

            row.push_str(" ]");
            rows.push(row);
        }

        write!(f, "{}", rows.join("\n"))
    }
}
