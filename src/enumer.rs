use crate::ctx::{Contexts, VariableMap};
use crate::store::Store;
use crate::vocab::{NodeEnumWrapper, Vocab, VocabIter};

pub struct Enumerator {
    store: Store,
    node_enum: NodeEnumWrapper,
    vocab: VocabIter,
}

impl Enumerator {
    pub fn new(vocab: &Vocab, var_map: VariableMap, ctxs: Contexts) -> Self {
        let store = Store::new(ctxs, var_map);
        let mut vocab_iter = vocab.iter();
        let node_enum = vocab_iter.next().expect("VocabIter was completely empty.");
        Self {
            store,
            node_enum,
            vocab: vocab_iter,
        }
    }
}
