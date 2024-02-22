use super::Level;
use super::Program;
use crate::cond::*;
use crate::store::*;
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

pub type BinProof<L, R> = &'static dyn Fn(&[L], &[R]) -> bool;
pub type BinEval<L, R, O> = &'static dyn Fn(
    &[L],
    &[R],
    PreCondition,
    PostCondition,
) -> (Vec<O>, PreCondition, PostCondition);
pub type BinCode = &'static dyn Fn(&str, &str) -> String;

#[derive(Clone)]
pub struct BinBuilder<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
{
    proof: BinProof<L, R>,
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
{
    pub fn new(proof: BinProof<L, R>, eval: BinEval<L, R, O>, code: BinCode) -> Self {
        Self { proof, eval, code }
    }

    pub fn apply(
        &self,
        lhs: PIdx<L>,
        rhs: PIdx<R>,
        bank: &Bank,
    ) -> Option<(Vec<O>, PreCondition, PostCondition)> {
        let l_prog = &bank[lhs];
        let r_prog = &bank[rhs];

        let l_vals = l_prog.values(bank);
        let r_vals = r_prog.values(bank);

        if !(self.proof)(l_vals, r_vals) {
            return None;
        }

        if let Some((pre, post)) = Condition::sequence(l_prog.conditions(), r_prog.conditions()) {
            Some((self.eval)(l_vals, r_vals, pre, post))
        } else {
            None
        }
    }

    pub fn code(&self) -> BinCode {
        self.code
    }
}
