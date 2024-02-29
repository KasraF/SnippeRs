use super::Level;
use super::Program;
use crate::cond::*;
use crate::store;
use crate::store::*;
use crate::synth;
use crate::synth::Enumerator;
use crate::utils::*;
use crate::MaybeProgram;

pub struct BinMaybeProgram<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
{
    pre: PreCondition,
    post: PostCondition,
    lhs: PIdx<L>,
    rhs: PIdx<R>,
    code: BinCode,
    values: Option<Vec<O>>,
    pointer: Option<Pointer>,
    level: Level,
}

impl<L, R, O> BinMaybeProgram<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    pub fn new(
        lhs: PIdx<L>,
        rhs: PIdx<R>,
        values: Vec<O>,
        code: BinCode,
        pre: PreCondition,
        post: PostCondition,
        pointer: Option<Pointer>,
        level: Level,
    ) -> Box<dyn MaybeProgram<O>> {
        Box::new(Self {
            lhs,
            rhs,
            code,
            values: Some(values),
            pre,
            post,
            pointer,
            level,
        })
    }
}

impl<L, R, O> MaybeProgram<O> for BinMaybeProgram<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
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
        BinProgram::new(
            self.lhs,
            self.rhs,
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

pub struct BinProgram<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
{
    pre: PreCondition,
    post: PostCondition,
    lhs: PIdx<L>,
    rhs: PIdx<R>,
    code: BinCode,
    values: VIdx<O>,
    pointer: Option<Pointer>,
    level: Level,
}

impl<L, R, O> BinProgram<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    #[inline]
    pub fn new(
        lhs: PIdx<L>,
        rhs: PIdx<R>,
        values: VIdx<O>,
        code: BinCode,
        pre: PreCondition,
        post: PostCondition,
        pointer: Option<Pointer>,
        level: Level,
    ) -> Box<dyn Program<O>> {
        Box::new(Self {
            lhs,
            rhs,
            code,
            values,
            pre,
            post,
            pointer,
            level,
        })
    }
}

impl<L: Value, R: Value, O: Value> Program<O> for BinProgram<L, R, O>
where
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    fn code(&self, store: &Bank) -> String {
        let lhs = &store[self.lhs];
        let rhs = &store[self.rhs];
        let lhs = lhs.code(store);
        let rhs = rhs.code(store);
        (self.code)(&lhs, &rhs)
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

pub type BinEval<L, R, O> = &'static dyn Fn(
    &dyn Program<L>,
    &dyn Program<R>,
    &Condition,
    &Bank,
) -> Option<(Vec<O>, Option<Mutation>, Option<Pointer>)>;
pub type BinCode = &'static dyn Fn(&str, &str) -> String;

#[derive(Clone)]
pub struct BinBuilder<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
{
    eval: BinEval<L, R, O>,
    code: BinCode,
}

impl<L, R, O> BinBuilder<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
    MaxPIdx: store::MaxIdx<L>,
    MaxPIdx: store::MaxIdx<R>,
    MaxPIdx: store::MaxIdx<O>,
    AnyProg: From<PIdx<O>>,
{
    pub fn new(eval: BinEval<L, R, O>, code: BinCode) -> Self {
        Self { eval, code }
    }

    pub fn into_enum(&self, level: Level, max_idx: MaxPIdx) -> Box<dyn Enumerator> {
        Box::new(BinEnumerator {
            eval: self.eval,
            code: self.code,
            lhs_idx: 0.into(),
            rhs_idx: 0.into(),
            level,
            max_idx,
        })
    }
}

pub struct BinEnumerator<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    eval: BinEval<L, R, O>,
    code: BinCode,
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
    fn next(&mut self, store: &mut Bank) -> synth::Result<AnyProg> {
        if !self.max_idx.check(self.rhs_idx) {
            if !self.max_idx.check(self.lhs_idx + 1) {
                // We're out of programs
                return synth::Result::Done;
            }

            // Move to the next lhs child.
            self.lhs_idx += 1;
            self.rhs_idx = 0.into();
        }

        debug_assert!(store.has_program(self.lhs_idx));
        debug_assert!(store.has_program(self.rhs_idx));

        let lhs = &store[self.lhs_idx];
        let rhs = &store[self.rhs_idx];
        self.rhs_idx += 1;

        if lhs.level().bin_next(rhs.level()) != self.level {
            return synth::Result::None;
        }

        let (pre, post) = Condition::sequence(lhs.conditions(), rhs.conditions())?;
        let (values, mutation, pointer) = (*self.eval)(lhs.as_ref(), rhs.as_ref(), &post, &store)?;

        let post = match mutation {
            Some(mutation) => mutation.apply(post, store),
            None => post.clone(),
        };

        let maybe_program = BinMaybeProgram::new(
            self.lhs_idx,
            self.rhs_idx - 1,
            values,
            self.code,
            pre,
            post,
            pointer,
            self.level,
        );
        let rs = store.put_program(maybe_program)?;

        synth::Result::Some(rs.into())
    }
}

impl<L, R, O> std::fmt::Debug for BinEnumerator<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BinBuilder<{}>", (*self.code)("lhs", "rhs"))
    }
}
