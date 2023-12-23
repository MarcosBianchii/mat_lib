use crate::Entry;
use std::ops::*;

pub trait Matrix: Index<(usize, usize)> + IndexMut<(usize, usize)> {
    fn zeros(n: usize, m: usize) -> Self;

    fn get(&self, idx: (usize, usize)) -> &Entry;
    fn get_mut(&mut self, idx: (usize, usize)) -> &mut Entry;
    fn set(&mut self, idx: (usize, usize), val: Entry) -> Option<Entry>;
    fn shape(&self) -> (usize, usize);

    fn add<M: Matrix>(&mut self, rhs: M) -> Self;
    fn mul<M: Matrix>(&mut self, rhs: M) -> Self;
    fn sub<M: Matrix>(&mut self, rhs: M) -> Self;

    fn t(&self) -> Self;
}
