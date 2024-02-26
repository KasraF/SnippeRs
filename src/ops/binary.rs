use super::Level;
use super::Program;
use crate::cond::*;
use crate::store;
use crate::store::*;
use crate::synth;
use crate::synth::Enumerator;
use crate::utils::*;

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
    pub fn new(
        lhs: PIdx<L>,
        rhs: PIdx<R>,
        values: VIdx<O>,
        code: BinCode,
        pre: PreCondition,
        post: PostCondition,
        level: Level,
    ) -> Box<dyn Program<O>> {
        Box::new(Self {
            lhs,
            rhs,
            code,
            values,
            pre,
            post,
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
}

pub type BinEval<L, R, O> = &'static dyn Fn(
    &[L],
    &[R],
    PreCondition,
    PostCondition,
) -> Option<(Vec<O>, PreCondition, PostCondition)>;
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
            self.lhs_idx += 1; // FIXME off by one
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

        let l_vals = lhs.values(store);
        let r_vals = rhs.values(store);

        if let Some((pre, post)) = Condition::sequence(lhs.conditions(), rhs.conditions()) {
            let (values, pre, post) = (*self.eval)(l_vals, r_vals, pre, post)?;
            let values_idx = store.put_values(values)?;
            let program = BinProgram::new(
                self.lhs_idx,
                self.rhs_idx - 1,
                values_idx,
                self.code,
                pre,
                post,
                self.level,
            );

            synth::Result::Some(store.put_program(program).into())
        } else {
            synth::Result::None
        }
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
