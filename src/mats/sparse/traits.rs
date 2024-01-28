use std::fmt::{self, Display};
use std::ops::{Index, IndexMut};

use super::mat::Sparse;
use super::SparseImplTraits;

impl<T: SparseImplTraits, const N: usize, const M: usize> Index<(usize, usize)>
    for Sparse<T, N, M>
{
    type Output = T;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        self.get(idx).expect("Index out of range")
    }
}

impl<T: SparseImplTraits, const N: usize, const M: usize> IndexMut<(usize, usize)>
    for Sparse<T, N, M>
{
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        self.get_mut(idx).expect("IndexMut out of range")
    }
}

impl<T: SparseImplTraits, const N: usize, const M: usize> Display for Sparse<T, N, M> {
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
                row.push_str(&fmt[..len.min(fmt.len())]);
            }

            row.push_str(" ]");
            rows.push(row);
        }

        write!(f, "{}", rows.join("\n"))
    }
}

impl<T: SparseImplTraits, const N: usize, const M: usize> FromIterator<((usize, usize), T)>
    for Sparse<T, N, M>
{
    fn from_iter<I: IntoIterator<Item = ((usize, usize), T)>>(iter: I) -> Self {
        let mut mat = Self::zeros();

        for (idx, val) in iter.into_iter() {
            mat.set(idx, val);
        }

        mat
    }
}
