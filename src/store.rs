use crate::ctx::{Contexts, VariableMap};
use crate::nodes::{MaybeNode, Node};
use crate::utils::*;
use std::collections::HashSet;

pub trait ProgramStore<T: Val> {
    fn insert(&mut self, node: Box<dyn MaybeNode<T>>) -> Option<Index<T>>;
    fn program<'s>(&'s self, idx: Index<T>) -> &'s dyn Node<T>;
    fn values<'s>(&'s self, idx: Index<T>) -> &'s [T];
    fn has(&self, idx: Index<T>) -> bool;
}

impl ProgramStore<Int> for Store {
    fn insert(&mut self, node: Box<dyn MaybeNode<Int>>) -> Option<Index<Int>> {
        // Check if it's unique
        let values = node.values();
        debug_assert!(
            values.len() == self.ex,
            "Given MaybeNode has {} values, but examples is {}",
            values.len(),
            self.ex
        );
        if self.int_oe.contains(values) {
            return None;
        }

        // The values for this node are unique. So let's go!!!
        let nodes = &mut self.ints;
        let nodes_len = nodes.len();
        let idx = Index::new(nodes_len);
        let (node, mut node_values) = node.to_node(idx);

        // Add the node
        nodes.push(node);

        // Add the values
        // TODO This .clone() **hurts**. Can we do anything about it?!
        self.int_oe.insert(node_values.clone());
        let values = &mut self.int_vals;
        debug_assert!(
            values.len() == *idx * self.ex,
            "Nodes and values are out of sync: {} != {} (ex = {})",
            nodes_len,
            values.len(),
            self.ex
        );
        values.append(&mut node_values);

        Some(idx)
    }

    fn program<'s>(&'s self, idx: Index<Int>) -> &'s dyn Node<Int> {
        self.ints[*idx].as_ref()
    }

    fn values<'s>(&'s self, idx: Index<Int>) -> &'s [Int] {
        self.int_vals[*idx * self.ex..(*idx + 1) * self.ex].as_ref()
    }

    fn has(&self, idx: Index<Int>) -> bool {
        self.ints.len() > *idx
    }
}

pub struct Store {
    /// The number of examples we're working with.
    ex: usize,
    pub ctxs: Contexts,
    pub var_map: VariableMap,

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
            int_arrs: Vec::with_capacity(capacity),
            int_arr_vals: Vec::with_capacity(capacity * ex),
            int_arrs_oe: HashSet::new(),
        }
    }
}
