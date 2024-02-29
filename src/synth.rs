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
        let mut store = Bank::new(task.examples(), task.var_map.clone());

        // 3. Add the variables
        for (name, values, var_idx) in task.variables() {
            match values {
                Anies::Int(values) => {
                    store
                        .put_variable(name.clone(), values.clone(), var_idx)
                        .expect("Int variable already exists.");
                }
                Anies::Str(values) => {
                    store
                        .put_variable(name.clone(), values.clone(), var_idx)
                        .expect("Str variable already exists.");
                }
            }
        }

        // 4. Add the constants
        for con in vocab::constants() {
            match con {
                ConstVal::Int(code, val) => {
                    store
                        .put_constant(code, val)
                        .expect("Constant {code} already exists.");
                }
                ConstVal::Str(code, val) => {
                    store
                        .put_constant(code, val)
                        .expect("Constant {code} already exists.");
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

impl<A, B, C> FromResidual<std::result::Result<A, B>> for Result<C> {
    fn from_residual(_: std::result::Result<A, B>) -> Self {
        Result::None
    }
}
