use crate::ctx::Contexts;
use crate::nodes::NodeEnum;
use crate::store::Store;
use crate::utils::*;
use crate::vocab::{Vocab, VocabIter};

enum NodeEnumWrapper {
    Int(Box<dyn NodeEnum<Int>>),
    IntArray(Box<dyn NodeEnum<IntArray>>),
}

pub struct Enumerator<'a, 's>
where
    's: 'a,
{
    ctx: Contexts,
    store: Store<'s>,
    node_enum: NodeEnumWrapper,
    vocab: VocabIter<'a>,
}

impl<'a, 's: 'a> Enumerator<'a, 's> {
    pub fn new(ctx: Contexts, vocab: &'a Vocab) -> Self {
        let store = Store::new(ctx.len());
        let vocab_iter = vocab.iter();
        let node_enum = vocab_iter.next();
        Self {
            ctx,
            store,
            node_enum,
            vocab: vocab_iter,
        }
    }
}
