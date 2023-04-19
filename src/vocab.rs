use crate::nodes::*;
use crate::utils::*;

#[derive(Clone, Copy)]
pub enum NodeEnumWrapper {
    Int(NodeEnumBuilder<Int>),
    IntArray(NodeEnumBuilder<IntArray>),
}

#[derive(Clone)]
pub struct Vocab {
    node_enums: Vec<NodeEnumWrapper>,
}

impl Vocab {
    pub fn new(node_enums: Vec<NodeEnumWrapper>) -> Self {
        Self { node_enums }
    }

    pub fn iter<'a>(&'a self) -> VocabIter {
        VocabIter::new(self)
    }
}

impl Default for Vocab {
    fn default() -> Self {
        let mut enums = Vec::with_capacity(16);

        // TODO Is there a way to require using every available type here?
        enums.push(NodeEnumWrapper::Int(&variable_node_enum));

        // Then all the others

        Vocab::new(enums)
    }
}

pub struct VocabIter {
    idx: usize,
    vocab: Vocab,
}

impl VocabIter {
    fn new(vocab: &Vocab) -> Self {
        Self {
            idx: 0,
            vocab: vocab.clone(),
        }
    }
}

impl Iterator for VocabIter {
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
