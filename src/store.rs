use crate::nodes::Node;
use crate::utils::*;

// TODO re-write API so these don't need to be public
pub struct StoreLevel {
    pub int_nodes: Vec<Box<dyn Node<Int>>>,
    pub int_values: Vec<Int>,

    pub string_nodes: Vec<Box<dyn Node<Str>>>,
    pub string_values: Vec<Str>,

    pub boolean_nodes: Vec<Box<dyn Node<Bool>>>,
    pub boolean_values: Vec<Bool>,
}

impl StoreLevel {
    pub fn new() -> Self {
        let int_nodes = Vec::new();
        let int_values = Vec::new();
        let string_nodes = Vec::new();
        let string_values = Vec::new();
        let boolean_nodes = Vec::new();
        let boolean_values = Vec::new();
        Self {
            int_nodes,
            int_values,
            string_nodes,
            string_values,
            boolean_nodes,
            boolean_values,
        }
    }
}

pub trait NodeStore<T: NodeType> {
    fn get_node<'a>(&'a self, idx: &Idx<T>) -> Option<&'a Box<dyn Node<T>>>;
    fn get_values<'a>(&'a self, idx: &Idx<T>) -> Option<&'a [T]>;
    fn put(&mut self, node: Box<dyn Node<T>>, values: Vec<T>) -> Idx<T>;
    fn step(&self, idx: &Idx<T>) -> Option<Idx<T>>;
}

pub struct Store {
    pub contexts: usize,
    pub levels: Vec<StoreLevel>,
}

impl<'a> Store {
    pub fn new(contexts: usize) -> Self {
        Self {
            contexts,
            levels: vec![StoreLevel::new()],
        }
    }
}

impl NodeStore<Int> for Store {
    fn get_node<'a>(&'a self, idx: &Idx<Int>) -> Option<&'a Box<dyn Node<Int>>> {
        self.levels
            .get(idx.lvl)
            .and_then(|lvl| lvl.int_nodes.get(idx.idx))
    }

    fn get_values<'a>(&'a self, idx: &Idx<Int>) -> Option<&'a [Int]> {
        self.levels.get(idx.lvl).and_then(|lvl| {
            let start_idx = idx.idx * self.contexts;
            let end_idx = start_idx + self.contexts;
            let len = lvl.int_values.len();
            if len > start_idx && len >= end_idx {
                Some(&lvl.int_values[start_idx..end_idx])
            } else {
                None
            }
        })
    }

    fn put(&mut self, node: Box<dyn Node<Int>>, values: Vec<Int>) -> Idx<Int> {
        let lvl = node.level();

        while self.levels.len() <= lvl {
            self.levels.push(StoreLevel::new());
        }

        let idx = self.levels[lvl].int_nodes.len();

        self.levels[lvl].int_nodes.push(node);
        self.levels[lvl].int_values.extend(values);

        Idx::new(lvl, idx)
    }

    fn step(&self, idx: &Idx<Int>) -> Option<Idx<Int>> {
        if self.levels[idx.lvl].int_nodes.len() > idx.idx {
            Some(Idx::new(idx.lvl, idx.idx + 1))
        } else if self.levels.len() > idx.lvl && !self.levels[idx.lvl + 1].int_nodes.is_empty() {
            Some(Idx::new(idx.lvl + 1, 0))
        } else {
            None
        }
    }
}
