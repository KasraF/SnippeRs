use crate::nodes::{MaybeNode, Node};
use crate::utils::*;
use std::collections::HashSet;

/// A private trait, used to simplify defining `Store` as an instance of
/// variour ProgramStores.
pub trait PrivateStore<T: Val> {
    fn examples(&self) -> usize;
    fn nodes(&self) -> &[Box<dyn Node<T>>];
    fn nodes_mut(&mut self) -> &mut Vec<Box<dyn Node<T>>>;
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut Vec<T>;
    fn oe(&self) -> &HashSet<Vec<T>>;
    fn oe_mut(&mut self) -> &mut HashSet<Vec<T>>;
}

pub trait ProgramStore<T: Val> {
    fn insert(&mut self, node: Box<dyn MaybeNode<T>>) -> Option<Index<T>>;
    fn program<'s>(&'s self, idx: Index<T>) -> &'s dyn Node<T>;
    fn values<'s>(&'s self, idx: Index<T>) -> &'s [T];
    fn has(&self, idx: Index<T>) -> bool;
}

impl<T: Val> ProgramStore<T> for dyn PrivateStore<T> {
    fn insert(&mut self, node: Box<dyn MaybeNode<T>>) -> Option<Index<T>> {
        // Check if it's unique
        let examples = self.examples();
        let values = node.values();
        debug_assert!(
            values.len() == examples,
            "Given MaybeNode has {} values, but examples is {}",
            values.len(),
            examples
        );
        if self.oe().contains(values) {
            return None;
        }

        // The values for this node are unique. So let's go!!!
        let nodes = self.nodes_mut();
        let nodes_len = nodes.len();
        let idx = Index::new(nodes_len);
        let (node, mut node_values) = node.to_node(idx);

        // Add the node
        nodes.push(node);

        // Add the values
        // TODO This .clone() **hurts**. Can we do anything about it?!
        self.oe_mut().insert(node_values.clone());
        let values = self.values_mut();
        debug_assert!(
            values.len() == idx.idx * examples,
            "Nodes and values are out of sync: {} != {} (ex = {})",
            nodes_len,
            values.len(),
            examples
        );
        values.append(&mut node_values);

        Some(idx)
    }

    fn program<'s>(&'s self, idx: Index<T>) -> &'s dyn Node<T> {
        self.nodes()[idx.idx].as_ref()
    }

    fn values<'s>(&'s self, idx: Index<T>) -> &'s [T] {
        let examples = self.examples();
        self.values()[idx.idx * examples..(idx.idx + 1) * examples].as_ref()
    }

    fn has(&self, idx: Index<T>) -> bool {
        self.nodes().len() > idx.idx
    }
}

pub struct Store {
    /// The number of examples we're working with.
    ex: usize,

    // Integers
    ints: Vec<Box<dyn Node<Int>>>,
    int_vals: Vec<Int>,
    int_oe: HashSet<Vec<Int>>,

    // Integer arrays
    int_arrs: Vec<Box<dyn Node<IntArray>>>,
    int_arr_vals: Vec<IntArray>,
    int_arrs_oe: HashSet<Vec<IntArray>>,
}

impl Store {
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

impl PrivateStore<Int> for Store {
    fn examples(&self) -> usize {
        self.ex
    }

    fn nodes(&self) -> &[Box<dyn Node<Int>>] {
        self.ints.as_slice()
    }

    fn nodes_mut(&mut self) -> &mut Vec<Box<dyn Node<Int>>> {
        &mut self.ints
    }

    fn values(&self) -> &[Int] {
        &self.int_vals
    }

    fn values_mut(&mut self) -> &mut Vec<Int> {
        &mut self.int_vals
    }

    fn oe(&self) -> &HashSet<Vec<Int>> {
        &self.int_oe
    }

    fn oe_mut(&mut self) -> &mut HashSet<Vec<Int>> {
        &mut self.int_oe
    }
}
