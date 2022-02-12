use crate::utils::*;
use std::collections::HashSet;

pub trait OE<T: NodeType> {
    fn is_unique(&self, values: &[T]) -> bool;

    // TODO In theory, the cache should be able to just get a reference
    //  to the values stored in the Store as a slice. But I can't get
    //  the lifetimes to behave.
    fn insert(&mut self, values: &[T]);
}

pub struct OECache {
    int: HashSet<Vec<Int>>,
    string: HashSet<Vec<Str>>,
    boolean: HashSet<Vec<Bool>>,
}

impl OECache {
    pub fn new() -> Self {
        Self {
            int: HashSet::new(),
            string: HashSet::new(),
            boolean: HashSet::new(),
        }
    }
}

impl OE<Int> for OECache {
    fn is_unique(&self, values: &[Int]) -> bool {
        !self.int.contains(values)
    }

    fn insert(&mut self, values: &[Int]) {
        self.int.insert(values.to_vec());
    }
}
