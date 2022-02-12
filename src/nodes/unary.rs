use crate::nodes::Node;
use crate::store::{NodeStore, Store};
use crate::utils::*;

pub struct UnaryGenerator<'a, I: NodeType, O: NodeType>
where
    Store: NodeStore<I>,
    Store: NodeStore<O>,
{
    lvl: usize,
    idx: Idx<I>,
    generate: &'a dyn Fn(Idx<I>, &Store) -> (Box<dyn Node<O>>, Vec<O>),
}

impl<'a, I: NodeType, O: NodeType> UnaryGenerator<'a, I, O>
where
    Store: NodeStore<I>,
    Store: NodeStore<O>,
{
    pub fn new(
        lvl: usize,
        idx: Idx<I>,
        generate: &'a dyn Fn(Idx<I>, &Store) -> (Box<dyn Node<O>>, Vec<O>),
    ) -> Self {
        Self { lvl, idx, generate }
    }

    pub fn step(&mut self, store: &Store) {
        let curr_max = store.levels[self.idx.lvl].int_nodes.len() - 1;

        if self.idx.idx < curr_max {
            self.idx.idx += 1;
        } else {
            self.idx.lvl += 1;
            self.idx = 0;
        }
    }
}

impl<'a, L: NodeType, R: NodeType, T: NodeType> NodeGenerator<T> for BinaryGenerator<'a, L, R, T>
where
    Store: NodeStore<L>,
    Store: NodeStore<R>,
    Store: NodeStore<T>,
{
    fn next(&mut self, store: &Store) -> Option<(Box<dyn Node<T>>, Vec<T>)> {
        // First, update the indices
        self.step(store);
        let node = (self.generate)(self.lhs.clone(), self.rhs.clone(), store);
        Some(node)
    }
}

#[derive(Debug)]
pub struct IntAddition {
    lhs: Idx<Int>,
    rhs: Idx<Int>,
}

impl Node<Int> for IntAddition {
    fn code(&self, store: &Store) -> String {
        let lhs = store.get_node(&self.lhs).unwrap();
        let rhs = store.get_node(&self.rhs).unwrap();
        format!("{} + {}", lhs.code(store), rhs.code(store))
    }

    fn level(&self) -> usize {
        self.lhs.lvl + self.rhs.lvl + 1
    }
}

impl IntAddition {
    pub fn new(lhs: Idx<Int>, rhs: Idx<Int>, store: &Store) -> (Box<dyn Node<Int>>, Vec<Int>) {
        // TODO Error handling
        let values = store
            .get_values(&lhs)
            .unwrap()
            .iter()
            .zip(store.get_values(&rhs).unwrap().iter())
            .map(|(l, r)| l + r)
            .collect();
        let node = Box::new(Self { lhs, rhs });
        (node, values)
    }
}
