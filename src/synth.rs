use crate::{enumer::Enumerator, task::Task, vocab::Vocab};

pub struct Synth {
    vocab: Vocab,
    enumerator: Enumerator,
}

impl Synth {
    pub fn new(task: Task) -> Self {
        let (variables, ctxs) = task.get_contexts();
        let vocab = Vocab::default();
        let enumerator = Enumerator::new(&vocab, variables, ctxs);
        Self { vocab, enumerator }
    }

    pub fn solve(&mut self) -> Option<String> {
        None
    }
}
