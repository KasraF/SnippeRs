use crate::nodes::Node;
use crate::utils::*;
use std::collections::HashSet;

pub trait ProgramStore<'a, T: Val> {
    fn unique(&self, xs: &[T]) -> bool;
    fn oe_insert(&mut self, xs: &[T]) -> Option<Index<T>> {
        if self.unique(xs) {
            Some(self.insert(xs))
        } else {
            None
        }
    }
    fn insert(&mut self, xs: &[T]) -> Index<T>;
    fn program<'s>(&'s self, idx: Index<T>) -> &'s Node<T>;
    fn values<'s>(&'s self, idx: Index<T>) -> &'s [T];
    fn has(&self, idx: Index<T>) -> bool;
}

pub struct Store<'a> {
    /// The number of examples we're working with.
    ex: usize,
    ints: Vec<Box<dyn Node<Int>>>,
    int_vals: Vec<Int>,
    int_oe: HashSet<&'a [Int]>,
    int_arrs: Vec<Box<dyn Node<IntArray>>>,
    int_arr_vals: Vec<IntArray>,
    int_arrs_oe: HashSet<&'a [IntArray]>,
}

impl<'a> Store<'a> {
    pub fn new(examples: usize) -> Self {
        let capacity = 32;
        Self {
            ex: examples,
            ints: Vec::with_capacity(capacity),
            int_vals: Vec::with_capacity(capacity * examples),
            int_oe: HashSet::new(),
            int_arrs: Vec::with_capacity(capacity),
            int_arr_vals: Vec::with_capacity(capacity * examples),
            int_arrs_oe: HashSet::new(),
        }
    }
}

impl<'a> ProgramStore<'a, Int> for Store<'a> {
    fn unique(&self, xs: &[Int]) -> bool {
        self.int_oe.contains(xs)
    }

    fn insert(&mut self, xs: &[Int]) -> Index<Int> {
        let start = self.int_vals.len();
        self.int_vals.extend_from_slice(xs);
        Index::new(start)
    }

    fn program<'s>(&'s self, idx: Index<IntArray>) -> &'s Node<IntArray> {
        debug_assert!(self.ints.len() > idx.idx, "Index does not exist: {}", idx);
        debug_assert!(
            idx.start % self.ex == 0,
            "Index didn't start at a boundary: {}",
            idx
        );
        &self.ints[idx.idx]
    }

    fn values(&'a self, idx: Index<Int>) -> Option<&'a [Int]> {
        if self.int_vals.len() <= idx.start + self.ex {
            debug_assert!(
                idx.start % self.ex == 0,
                "Index didn't start at a boundary: {}",
                idx
            );
            Some(&self.int_vals[idx.start..idx.start + self.ex])
        } else {
            None
        }
    }

    fn has(&self, idx: Index<Int>) -> bool {
        self.int_vals.len() > idx.idx
    }
}

impl<'a> ProgramStore<'a, IntArray> for Store<'a> {
    fn unique(&self, xs: &[IntArray]) -> bool {
        self.int_arrs_oe.contains(xs)
    }

    fn insert(&mut self, xs: &[IntArray]) -> Index<IntArray> {
        let start = self.int_arr_vals.len();
        self.int_arr_vals.extend_from_slice(xs);
        Index::new(start)
    }

    fn program<'s>(&'s self, idx: Index<IntArray>) -> &'s Node<IntArray> {
        debug_assert!(
            self.int_arrs.len() > idx.idx,
            "Index does not exist: {}",
            idx
        );
        debug_assert!(
            idx.start % self.ex == 0,
            "Index didn't start at a boundary: {}",
            idx
        );
        &self.int_arrs[idx.idx]
    }

    fn values(&'a self, idx: Index<IntArray>) -> &'a [IntArray] {
        if self.int_vals.len() <= idx.start + self.ex {
            debug_assert!(
                idx.start % self.ex == 0,
                "Index didn't start at a boundary: {:?}",
                idx
            );
            Some(&self.int_arr_vals[idx.start..idx.start + self.ex])
        } else {
            None
        }
    }

    fn has(&self, idx: Index<Int>) -> bool {
        self.int_arr_vals.len() > idx.idx
    }
}
