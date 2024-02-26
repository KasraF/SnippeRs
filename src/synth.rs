use std::ops::FromResidual;

use crate::ops::*;
use crate::store::*;
use crate::task::SynthesisTask;
use crate::utils::*;
use crate::vocab;
use crate::vocab::ConstVal;
use crate::vocab::Vocab;

pub struct Synthesizer {
    vocab: Vocab,
    store: Bank,
    task: SynthesisTask,
    curr_enum: Box<dyn Enumerator>,
    curr_level: Level,
    curr_vocab: usize,
}

impl Synthesizer {
    pub fn new(vocab: Vocab, task: SynthesisTask) -> Self {
        let curr_level = 1.into();
        let curr_vocab = 0;

        // Building the store takes a few steps
        let mut store = Bank::new(task.examples());

        // 3. Add the variables
        let variables = task.variables().count();
        for (name, values, var_idx) in task.variables() {
            match values {
                Anies::Int(values) => {
                    let idx = store.put_values(values.clone()).unwrap(); // FIXME
                    store.put_program(Variable::<Int>::new(name.clone(), idx, *var_idx, variables));
                }
                Anies::Str(values) => {
                    let idx = store.put_values(values.clone()).unwrap(); // FIXME
                    store.put_program(Variable::<Str>::new(name.clone(), idx, *var_idx, variables));
                }
            }
        }

        // 4. Add the constants
        for con in vocab::constants() {
            match con {
                ConstVal::Int(code, val) => {
                    let values = vec![val; task.examples()];
                    if let Some(idx) = store.put_values(values) {
                        store.put_program(Constant::<Int>::new(code.to_string(), idx, variables));
                    }
                }
                ConstVal::Str(code, val) => {
                    let values = vec![val; task.examples()];
                    if let Some(idx) = store.put_values(values) {
                        store.put_program(Constant::<Str>::new(code.to_string(), idx, variables));
                    }
                }
            }
        }

        let curr_enum = vocab[curr_vocab].enumerator(curr_level, &store);

        Self {
            vocab,
            store,
            task,
            curr_enum,
            curr_level,
            curr_vocab,
        }
    }

    #[inline]
    pub fn store(&self) -> &Bank {
        &self.store
    }

    pub fn next(&mut self) -> Box<AnyProg> {
        loop {
            match self.curr_enum.next(&mut self.store) {
                Result::Some(prog) => return Box::new(prog),
                Result::None => (), // try again
                Result::Done => {
                    // Move to next enumerator and try again!
                    self.curr_vocab += 1;

                    if self.vocab.len() <= self.curr_vocab {
                        // We're out of vocabs. Go to next level and reset.
                        self.curr_level.inc();
                        self.curr_vocab = 0;
                    }

                    self.curr_enum =
                        self.vocab[self.curr_vocab].enumerator(self.curr_level, &self.store);
                }
            }
        }
    }
}

pub trait Enumerator: std::fmt::Debug {
    fn next(&mut self, store: &mut Bank) -> Result<AnyProg>;
}

pub enum Result<T> {
    Some(T),
    None,
    Done,
}

impl<A, B> FromResidual<Option<A>> for Result<B> {
    fn from_residual(_: Option<A>) -> Self {
        Result::None
    }
}
