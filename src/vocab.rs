use crate::nodes::*;
use crate::utils::*;

pub enum NodeEnumWrapper {
    Int(Box<dyn NodeEnum<Int>>),
    IntArray(Box<dyn NodeEnum<IntArray>>),
}

pub struct Vocab {
    node_enums: Vec<NodeEnumWrapper>,
}

impl Vocab {
    pub fn new(node_enums: Vec<dyn NodeEnumBuilder>) -> Self {
        Self { node_enums }
    }

    pub fn iter<'a>(&'a self) -> VocabIter<'a> {
        VocabIter::new(self)
    }
}

impl Default for Vocab {
    fn default() -> Self {
        let node_enums = vec![];
        Vocab::new(node_enums)
    }
}

pub struct VocabIter<'a> {
    idx: usize,
    vocab: &'a Vocab,
}

impl<'a> Iterator for VocabIter<'a> {
    type Item = &'a dyn NodeEnumBuilder;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
