use crate::cond::*;
use crate::store::*;
use crate::synth;
use crate::synth::Enumerator;
use crate::utils::*;
use crate::MaybeProgram;

use super::Level;
use super::Program;

pub type UniEval<I, O> = &'static dyn Fn(
    &dyn Program<I>,
    &Condition,
    &Bank,
) -> Option<(Vec<O>, Option<Mutation>, Option<Pointer>)>;
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
        let (values, mutation, pointer) = (self.eval)(prog.as_ref(), post, &store)?;
        let pre = pre.clone(); // TODO Would be nice to avoid this clone if OE denies this program.

        // If it comes with a mutation, we need to see if we can add it to the store.
        // Basically, if it's a new variable that doesn't currently exist,
        // this is guaranteed to be a new program. Otherwise, it will return the index to that value.
        // Either way, we can proceed.
        let post = match mutation {
            Some(mutation) => mutation.apply(post.clone(), store),
            None => post.clone(),
        };

        // See if we can add this
        let maybe_program =
            UniMaybeProgram::new(curr_idx, values, self.code, pre, post, pointer, self.level);
        let prog_idx = store.put_program(maybe_program)?;
        synth::Result::Some(prog_idx.into())
    }
}

pub struct UniMaybeProgram<I, O>
where
    I: Value,
    O: Value,
{
    pre: PreCondition,
    post: PostCondition,
    arg: PIdx<I>,
    code: UniCode,
    values: Option<Vec<O>>,
    pointer: Option<Pointer>,
    level: Level,
}

impl<I, O> UniMaybeProgram<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
{
    pub fn new(
        arg: PIdx<I>,
        values: Vec<O>,
        code: UniCode,
        pre: PreCondition,
        post: PostCondition,
        pointer: Option<Pointer>,
        level: Level,
    ) -> Box<dyn MaybeProgram<O>> {
        Box::new(Self {
            arg,
            code,
            values: Some(values),
            pre,
            post,
            pointer,
            level,
        })
    }
}

impl<I, O> MaybeProgram<O> for UniMaybeProgram<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
{
    fn values(&self) -> Option<&[O]> {
        self.values.as_deref()
    }

    fn extract_values(&mut self) -> Option<Vec<O>> {
        let mut rs = None;
        std::mem::swap(&mut rs, &mut self.values);
        rs
    }

    fn into_program(self: Box<Self>, values: VIdx<O>) -> Box<dyn Program<O>> {
        UniProgram::new(
            self.arg,
            values,
            self.code,
            self.pre,
            self.post,
            self.pointer,
            self.level,
        )
    }

    fn pointer(&self) -> Option<Pointer> {
        self.pointer
    }

    fn pre_condition(&self) -> &PreCondition {
        &self.pre
    }

    fn post_condition(&self) -> &PostCondition {
        &self.post
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
    #[inline]
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

    fn values_idx(&self) -> VIdx<O> {
        self.values
    }
}
