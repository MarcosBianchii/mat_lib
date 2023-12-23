use std::{collections::HashMap, mem};

use crate::Entry;

#[derive(Debug)]
pub struct Sparse {
    data: HashMap<(usize, usize), Entry>,
    n: usize,
    m: usize,
}

impl Sparse {
    pub fn zeros(n: usize, m: usize) -> Self {
        let data = HashMap::new();
        Self { data, n, m }
    }

    fn is_in_range(&self, (i, j): (usize, usize)) -> bool {
        i < self.n && j < self.m
    }

    pub fn get(&self, idx: (usize, usize)) -> Option<&Entry> {
        self.data.get(&idx).or_else(|| {
            if self.is_in_range(idx) {
                Some(&0.0)
            } else {
                None
            }
        })
    }

    pub fn get_mut(&mut self, idx: (usize, usize)) -> Option<&mut Entry> {
        if !self.is_in_range(idx) {
            return None;
        }

        Some(self.data.entry(idx).or_insert(0.0))
    }

    pub fn set(&mut self, idx: (usize, usize), val: Entry) -> Option<Entry> {
        self.data.insert(idx, val)
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn is_square(&self) -> bool {
        let (n, m) = self.shape();
        n == m
    }

    #[allow(unused_variables)]
    pub fn reshape(&mut self, shape @ (n, m): (usize, usize)) -> &mut Self {
        self.n = n;
        self.m = m;

        self.data = mem::take(&mut self.data)
            .into_iter()
            .filter(|(idx, _)| self.is_in_range(*idx))
            .collect();

        self
    }

    pub fn scalar_mul(&mut self, rhs: Entry) -> &mut Self {
        self.apply(|e| e * rhs)
    }

    pub fn apply<F: Fn(Entry) -> Entry>(&mut self, f: F) -> &mut Self {
        self.data.values_mut().for_each(|e| *e = f(*e));
        self
    }

    pub fn det(&self) -> Option<Entry> {
        todo!()
    }

    pub fn inv(&mut self) -> Option<&mut Self> {
        todo!()
    }
}
