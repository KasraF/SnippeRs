use std::ops::FromResidual;

use crate::ops::*;
use crate::store::*;
use crate::task::SynthesisTask;
use crate::utils::*;

pub enum Builder {
    UnaryIntInt(UniBuilder<Int, Int>),
    UnaryIntStr(UniBuilder<Int, Str>),
    UnaryStrInt(UniBuilder<Str, Int>),
    UnaryStrStr(UniBuilder<Str, Str>),
    BinaryIntIntInt(BinBuilder<Int, Int, Int>),
    BinaryIntIntStr(BinBuilder<Int, Int, Str>),
    BinaryIntStrInt(BinBuilder<Int, Str, Int>),
    BinaryIntStrStr(BinBuilder<Int, Str, Str>),
    BinaryStrIntInt(BinBuilder<Str, Int, Int>),
    BinaryStrIntStr(BinBuilder<Str, Int, Str>),
    BinaryStrStrInt(BinBuilder<Str, Str, Int>),
    BinaryStrStrStr(BinBuilder<Str, Str, Str>),
}

impl From<UniBuilder<Str, Int>> for Builder {
    fn from(value: UniBuilder<Str, Int>) -> Self {
        Builder::UnaryStrInt(value)
    }
}

impl From<BinBuilder<Int, Int, Int>> for Builder {
    fn from(value: BinBuilder<Int, Int, Int>) -> Self {
        Builder::BinaryIntIntInt(value)
    }
}

impl Builder {
    pub fn enumerator(&self, level: Level, store: &Bank) -> Box<dyn Enumerator> {
        let max_idx = store.curr_max();
        match &self {
            Builder::UnaryIntInt(builder) => builder.into_enum(level, max_idx),
            Builder::UnaryIntStr(builder) => builder.into_enum(level, max_idx),
            Builder::UnaryStrInt(builder) => builder.into_enum(level, max_idx),
            Builder::UnaryStrStr(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryIntIntInt(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryIntIntStr(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryIntStrInt(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryIntStrStr(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryStrIntInt(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryStrIntStr(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryStrStrInt(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryStrStrStr(builder) => builder.into_enum(level, max_idx),
        }
    }
}

pub type Vocab = Vec<Builder>;

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
