use crate::nodes::*;
use crate::utils::*;

pub enum NodeEnumWrapper {
    Int(NodeEnumBuilder<Int>),
    IntArray(NodeEnumBuilder<IntArray>),
}

pub struct Vocab {
    node_enums: Vec<NodeEnumWrapper>,
}

impl Vocab {
    pub fn new(node_enums: Vec<NodeEnumWrapper>) -> Self {
        Self { node_enums }
    }

    pub fn iter<'a>(&'a self) -> VocabIter<'a> {
        VocabIter::new(self)
    }
}

pub struct VocabIter<'a> {
    idx: usize,
    vocab: &'a Vocab,
}

impl<'a> VocabIter<'a> {
    fn new(vocab: &'a Vocab) -> Self {
        Self { idx: 0, vocab }
    }
}

impl<'a> Iterator for VocabIter<'a> {
    type Item = NodeEnumWrapper;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.vocab.node_enums.len() {
            let rs = self.vocab.node_enums[self.idx];
            self.idx += 1;
            Some(rs)
        } else {
            None
        }
    }
}
