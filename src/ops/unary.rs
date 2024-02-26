use crate::cond::*;
use crate::store::*;
use crate::synth;
use crate::synth::Enumerator;
use crate::utils::*;

use super::Level;
use super::Program;

pub type UniEval<I, O> = &'static dyn Fn(
    &dyn Program<I>,
    Condition,
    &Bank,
) -> Option<(Vec<O>, PostCondition, Option<Pointer>)>;
pub type UniCode = &'static dyn Fn(&str) -> String;

#[derive(Clone)]
pub struct UniBuilder<I, O>
where
    I: Value,
    O: Value,
{
    eval: UniEval<I, O>,
    code: UniCode,
}

impl<I, O> UniBuilder<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
    PIdx<O>: Into<AnyProg>,
    MaxPIdx: MaxIdx<I>,
{
    pub fn new(eval: UniEval<I, O>, code: UniCode) -> Self {
        Self { eval, code }
    }

    pub fn into_enum(&self, level: Level, max_idx: MaxPIdx) -> Box<dyn Enumerator> {
        Box::new(UniEnumerator {
            eval: self.eval,
            code: self.code,
            arg_idx: 0.into(),
            level,
            max_idx,
        })
    }
}

impl<I, O> std::fmt::Debug for UniEnumerator<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UniBuilder<{}>", (*self.code)("arg"))
    }
}

pub struct UniEnumerator<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
{
    eval: UniEval<I, O>,
    code: UniCode,
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
    fn next(&mut self, store: &mut Bank) -> synth::Result<AnyProg> {
        if !self.max_idx.check(self.arg_idx) {
            return synth::Result::Done;
        }

        debug_assert!(store.has_program(self.arg_idx));

        let (prog, curr_idx) = {
            let mut prog = &store[self.arg_idx];
            self.arg_idx += 1;
            let prev_level = self.level.prev();

            while prog.level() != prev_level && self.max_idx.check(self.arg_idx) {
                prog = &store[self.arg_idx];
                self.arg_idx += 1;
            }

            (prog, self.arg_idx - 1)
        };

        let (pre, post) = prog.conditions();
        let (values, post, pointer) = (self.eval)(prog.as_ref(), post.clone(), &store)?;
        let pre = pre.clone(); // TODO Would be nice to avoid this clone if OE denies this program.

        // See if we can add this
        let val_idx = store.put_values(values)?;
        let program = UniProgram::new(
            curr_idx,
            val_idx,
            self.code,
            pre.clone(),
            post,
            pointer,
            self.level,
        );
        let prog_idx = store.put_program(program);
        synth::Result::Some(prog_idx.into())
    }
}

pub struct UniProgram<L, O>
where
    L: Value,
    O: Value,
{
    pre: PreCondition,
    post: PostCondition,
    arg: PIdx<L>,
    code: UniCode,
    values: VIdx<O>,
    pointer: Option<Pointer>,
    level: Level,
}

impl<L, O> UniProgram<L, O>
where
    L: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<O>,
{
    pub fn new(
        arg: PIdx<L>,
        values: VIdx<O>,
        code: UniCode,
        pre: PreCondition,
        post: PostCondition,
        pointer: Option<Pointer>,
        level: Level,
    ) -> Box<dyn Program<O>> {
        Box::new(Self {
            arg,
            code,
            values,
            pre,
            post,
            pointer,
            level,
        })
    }
}

impl<I, O> Program<O> for UniProgram<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
{
    fn code(&self, store: &Bank) -> String {
        let arg = store[self.arg].code(&store);
        (self.code)(&arg)
    }

    fn values<'s>(&self, store: &'s Bank) -> &'s [O] {
        &store[self.values]
    }

    fn conditions(&self) -> (&PreCondition, &PostCondition) {
        (&self.pre, &self.post)
    }

    #[inline]
    fn level(&self) -> Level {
        self.level
    }

    fn pointer(&self) -> Option<Pointer> {
        self.pointer
    }
}
