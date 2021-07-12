use crate::nodes::Node;
use crate::store::{NodeStore, Store};
use crate::utils::*;

pub struct BinaryGenerator<'a, L: NodeType, R: NodeType, T: NodeType>
where
    Store: NodeStore<L>,
    Store: NodeStore<R>,
    Store: NodeStore<T>,
{
    lvl: usize,
    lhs: Idx<L>,
    rhs: Idx<R>,
    generate: &'a dyn Fn(Idx<L>, Idx<R>, &Store) -> (Box<dyn Node<T>>, Vec<T>),
}

impl<'a, L: NodeType, R: NodeType, T: NodeType> BinaryGenerator<'a, L, R, T>
where
    Store: NodeStore<L>,
    Store: NodeStore<R>,
    Store: NodeStore<T>,
{
    pub fn new(
        lvl: usize,
        lhs: Idx<L>,
        rhs: Idx<R>,
        generate: &'a dyn Fn(Idx<L>, Idx<R>, &Store) -> (Box<dyn Node<T>>, Vec<T>),
    ) -> Self {
        Self {
            lvl,
            lhs,
            rhs,
            generate,
        }
    }

    pub fn step(&mut self, store: &Store) -> bool {
        let curr_rhs_max = store.levels[self.rhs.lvl].int_nodes.len() - 1;
        let curr_lhs_max = store.levels[self.lhs.lvl].int_nodes.len() - 1;

        if self.rhs.idx < curr_rhs_max {
            self.rhs.idx += 1;
            true
        } else if self.lhs.idx < curr_lhs_max {
            self.lhs.idx += 1;
            self.rhs.idx = 0;
            true
        } else {
            // We need to go to the next level if we can
            self.lhs.lvl += 1;

            if self.lhs.lvl < self.lvl {
                self.lhs.idx = 0;
                self.rhs.lvl = self.lvl - self.lhs.lvl;
                self.rhs.idx = 0;

                assert!(store.get_node(&self.lhs).is_some());
                assert!(store.get_node(&self.rhs).is_some());

                true
            } else {
                false
            }
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
        if self.step(store) {
            let node = (self.generate)(self.lhs.clone(), self.rhs.clone(), store);
            Some(node)
        } else {
            None
        }
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
