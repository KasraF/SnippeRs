use crate::ctx::{Contexts, VariableMap};
use crate::nodes::{MaybeNode, Node};
use crate::utils::*;
use snippers_macros::derive_store;
use std::collections::HashSet;

pub trait ProgramStore<T: Val> {
    fn insert(&mut self, node: Box<dyn MaybeNode<T>>) -> Option<Index<T>>;
    fn program<'s>(&'s self, idx: Index<T>) -> &'s dyn Node<T>;
    fn values<'s>(&'s self, idx: Index<T>) -> &'s [T];
    fn has(&self, idx: Index<T>) -> bool;
}

#[derive_store(Int, Str, IntArray)]
pub struct Store {
    /// The number of examples we're working with.
    ex: usize,
    pub ctxs: Contexts,
    pub var_map: VariableMap,

    // Integers
    ints: Vec<Box<dyn Node<Int>>>,
    int_vals: Vec<Int>,
    int_oe: HashSet<Vec<Int>>,

    // Strings
    strs: Vec<Box<dyn Node<Str>>>,
    str_vals: Vec<Str>,
    str_oe: HashSet<Vec<Str>>,

    // Integer arrays
    int_arrays: Vec<Box<dyn Node<IntArray>>>,
    int_array_vals: Vec<IntArray>,
    int_array_oe: HashSet<Vec<IntArray>>,
}

impl Store {
    pub fn new(ctxs: Contexts, var_map: VariableMap) -> Self {
        let ex = ctxs.len();
        let capacity = 32;

        Self {
            ex,
            ctxs,
            var_map,
            ints: Vec::with_capacity(capacity),
            int_vals: Vec::with_capacity(capacity * ex),
            int_oe: HashSet::new(),
            strs: Vec::with_capacity(capacity),
            str_vals: Vec::with_capacity(capacity * ex),
            str_oe: HashSet::new(),
            int_arrays: Vec::with_capacity(capacity),
            int_array_vals: Vec::with_capacity(capacity * ex),
            int_array_oe: HashSet::new(),
        }
    }
}
