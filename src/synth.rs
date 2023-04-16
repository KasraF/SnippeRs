use crate::{enumer::Enumerator, store::Store, task::Task};

struct Vocab {}

impl Vocab {
    fn new(_task: &Task) -> Self {
        Vocab {}
    }
}

pub struct Synth {
    store: Store,
    vocab: Vocab,
    enumerator: Enumerator,
}

impl Synth {
    pub fn new(task: Task) -> Self {
        let vocab = Vocab::new(&task);
        let store = Store::new(task.examples.len());
        let enumerator = Enumerator::new(task.get_context(), &vocab);
        Self {
            vocab,
            store,
            enumerator,
        }
    }

    pub fn solve(&mut self) -> Option<String> {
        None
    }
}
