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
    pub fn zeros() -> Self {
        let data = BTreeMap::new();
        let zero = T::from(0);
        Self { data, zero }
    }

    fn is_in_range(&self, (i, j): (usize, usize)) -> bool {
        i < N && j < M
    }

    pub fn get(&self, idx: (usize, usize)) -> Option<&T> {
        self.data
            .get(&idx)
            .or_else(|| self.is_in_range(idx).then(|| &self.zero))
    }

    pub fn get_mut(&mut self, idx: (usize, usize)) -> Option<&mut T> {
        if !self.is_in_range(idx) {
            return None;
        }

        Some(self.data.entry(idx).or_insert(T::from(0)))
    }

    pub fn set(&mut self, idx: (usize, usize), val: T) -> Option<T> {
        self.data.insert(idx, val)
    }

    pub fn shape(&self) -> (usize, usize) {
        (N, M)
    }

    pub fn is_square(&self) -> bool {
        N == M
    }

    pub fn scalar_mul(&mut self, rhs: T) -> &mut Self {
        self.apply(|e| e * rhs)
    }

    pub fn apply<F: Fn(T) -> T>(&mut self, f: F) -> &mut Self {
        self.data.values_mut().for_each(|e| *e = f(*e));
        self
    }

    pub fn det(&self) -> Option<T> {
        todo!()
    }

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
