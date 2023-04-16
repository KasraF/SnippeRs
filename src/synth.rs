use crate::{store::Store, task::Task};

struct Vocab {}

impl Vocab {
    fn new(_task: &Task) -> Self {
        Vocab {}
    }
}

pub struct Synth<'a> {
    store: Store<'a>,
    vocab: Vocab,
}

impl<'a> Synth<'a> {
    pub fn new(task: Task) -> Self {
        let vocab = Vocab::new(&task);
        let store = Store::new(task.examples.len());
        Self { vocab, store }
    }

    pub fn solve(&mut self) -> Option<String> {
        None
    }
}
