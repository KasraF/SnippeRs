use crate::nodes::Node;
use crate::store::Store;
use std::hash::Hash;
use std::marker::PhantomData;

pub type Error = Box<dyn std::error::Error>;
pub type Int = i32;
pub type Str = String;
pub type Bool = bool;

pub trait NodeType: Clone + Hash {}
impl NodeType for Int {}
impl NodeType for Str {}
impl NodeType for Bool {}

#[derive(Copy, Clone, Debug)]
pub struct Idx<T: NodeType> {
    pub lvl: usize,
    pub idx: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: NodeType> Idx<T> {
    pub fn new(lvl: usize, idx: usize) -> Self {
        Self {
            lvl,
            idx,
            _phantom_data: PhantomData,
        }
    }
}

// TODO Rewrite API so this doesn't need to be pub
pub struct GeneratorStore {
    pub int: Vec<Box<dyn NodeGenerator<Int>>>,
    pub string: Vec<Box<dyn NodeGenerator<Str>>>,
    pub boolean: Vec<Box<dyn NodeGenerator<Bool>>>,
}

pub trait NodeGenerator<T> {
    fn next(&mut self, store: &Store) -> Option<(Box<dyn Node<T>>, Vec<T>)>;
}
