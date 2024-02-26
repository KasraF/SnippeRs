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
            Builder::UnaryIntInt(builder) => Box::new(UniEnumerator {
                builder: builder.clone(),
                arg_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::UnaryIntStr(builder) => Box::new(UniEnumerator {
                builder: builder.clone(),
                arg_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::UnaryStrInt(builder) => Box::new(UniEnumerator {
                builder: builder.clone(),
                arg_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::UnaryStrStr(builder) => Box::new(UniEnumerator {
                builder: builder.clone(),
                arg_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::BinaryIntIntInt(builder) => Box::new(BinEnumerator {
                builder: builder.clone(),
                lhs_idx: 0.into(),
                rhs_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::BinaryIntIntStr(builder) => Box::new(BinEnumerator {
                builder: builder.clone(),
                lhs_idx: 0.into(),
                rhs_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::BinaryIntStrInt(builder) => Box::new(BinEnumerator {
                builder: builder.clone(),
                lhs_idx: 0.into(),
                rhs_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::BinaryIntStrStr(builder) => Box::new(BinEnumerator {
                builder: builder.clone(),
                lhs_idx: 0.into(),
                rhs_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::BinaryStrIntInt(builder) => Box::new(BinEnumerator {
                builder: builder.clone(),
                lhs_idx: 0.into(),
                rhs_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::BinaryStrIntStr(builder) => Box::new(BinEnumerator {
                builder: builder.clone(),
                lhs_idx: 0.into(),
                rhs_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::BinaryStrStrInt(builder) => Box::new(BinEnumerator {
                builder: builder.clone(),
                lhs_idx: 0.into(),
                rhs_idx: 0.into(),
                level,
                max_idx,
            }),
            Builder::BinaryStrStrStr(builder) => Box::new(BinEnumerator {
                builder: builder.clone(),
                lhs_idx: 0.into(),
                rhs_idx: 0.into(),
                level,
                max_idx,
            }),
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
    fn from_residual(residual: Option<A>) -> Self {
        Result::None
    }
}

#[derive(Debug)]
struct UniEnumerator<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
{
    builder: UniBuilder<I, O>,
    arg_idx: PIdx<I>,
    level: Level,
    max_idx: MaxPIdx,
}

impl<I, O> Enumerator for UniEnumerator<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
    PIdx<O>: Into<AnyProg>,
    MaxPIdx: MaxIdx<I>,
{
    fn next(&mut self, store: &mut Bank) -> Result<AnyProg> {
        if !self.max_idx.check(self.arg_idx) {
            return Result::Done;
        }

        debug_assert!(store.has_program(self.arg_idx));

        let curr_idx = {
            let mut prog = &store[self.arg_idx];
            self.arg_idx += 1;
            let prev_level = self.level.prev();

            while prog.level() != prev_level && self.max_idx.check(self.arg_idx) {
                prog = &store[self.arg_idx];
                self.arg_idx += 1;
            }

            self.arg_idx - 1
        };

        match self.builder.apply(curr_idx, store) {
            Some((values, pre, post)) => {
                // See if we can add this
                let val_idx = store.put_values(values)?;
                let program = UniProgram::new(
                    curr_idx,
                    val_idx,
                    self.builder.code(),
                    pre,
                    post,
                    self.level,
                );
                let prog_idx = store.put_program(program);
                Result::Some(prog_idx.into())
            }
            None => Result::None,
        }
    }
}

#[derive(Debug)]
struct BinEnumerator<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    builder: BinBuilder<L, R, O>,
    lhs_idx: PIdx<L>,
    rhs_idx: PIdx<R>,
    level: Level,
    max_idx: MaxPIdx,
}

impl<L, R, O> Enumerator for BinEnumerator<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
    PIdx<O>: Into<AnyProg>,
    MaxPIdx: MaxIdx<L>,
    MaxPIdx: MaxIdx<R>,
{
    fn next(&mut self, store: &mut Bank) -> Result<AnyProg> {
        if !self.max_idx.check(self.rhs_idx) {
            if !self.max_idx.check(self.lhs_idx + 1) {
                // We're out of programs
                return Result::Done;
            }

            // Move to the next lhs child.
            self.lhs_idx += 1; // FIXME off by one
            self.rhs_idx = 0.into();
        }

        debug_assert!(store.has_program(self.lhs_idx));
        debug_assert!(store.has_program(self.rhs_idx));

        let lhs = &store[self.lhs_idx];
        let rhs = &store[self.rhs_idx];
        self.rhs_idx += 1;

        if lhs.level().bin_next(rhs.level()) != self.level {
            return Result::None;
        }

        // TODO Move the logic entirely out of Builder. Builder should just contain references to the methods,
        // so we can initialize an Enumerator from it.

        match self.builder.apply(self.lhs_idx, self.rhs_idx - 1, store) {
            Some((values, pre, post)) => {
                let values_idx = store.put_values(values)?;
                let program = BinProgram::new(
                    self.lhs_idx,
                    self.rhs_idx - 1,
                    values_idx,
                    self.builder.code(),
                    pre,
                    post,
                    self.level,
                );
                Result::Some(store.put_program(program).into())
            }
            None => Result::None,
        }
    }
}
