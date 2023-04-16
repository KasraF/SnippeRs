use crate::ctx::Contexts;
use crate::store::Store;
use crate::vocab::{NodeEnumWrapper, Vocab, VocabIter};

pub struct Enumerator {
    ctx: Contexts,
    store: Store,
    node_enum: NodeEnumWrapper,
    vocab: VocabIter,
}

impl Enumerator {
    pub fn new(ctx: Contexts, vocab: &Vocab) -> Self {
        let store = Store::new(ctx.len());
        let mut vocab_iter = vocab.iter();
        let node_enum = vocab_iter.next().expect("VocabIter was completely empty.");
        Self {
            ctx,
            store,
            node_enum,
            vocab: vocab_iter,
        }
    }
}
