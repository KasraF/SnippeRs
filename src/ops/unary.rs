use crate::cond::*;
use crate::store::*;
use crate::utils::*;

use super::Level;
use super::Program;

pub type UniProof<I> = &'static dyn Fn(&[I]) -> bool;
pub type UniEval<I, O> =
    &'static dyn Fn(&[I], PreCondition, PostCondition) -> (Vec<O>, PreCondition, PostCondition);
pub type UniCode = &'static dyn Fn(&str) -> String;

#[derive(Clone)]
pub struct UniBuilder<I, O>
where
    I: Value,
    O: Value,
{
    proof: UniProof<I>,
    eval: UniEval<I, O>,
    code: UniCode,
}

impl<I, O> UniBuilder<I, O>
where
    I: Value,
    O: Value,
    Bank: Store<I>,
    Bank: Store<O>,
{
    pub fn new(proof: UniProof<I>, eval: UniEval<I, O>, code: UniCode) -> Self {
        Self { proof, eval, code }
    }

    pub fn apply(
        &self,
        arg: PIdx<I>,
        bank: &Bank,
    ) -> Option<(Vec<O>, PreCondition, PostCondition)> {
        let prog = &bank[arg];
        let vals = prog.values(bank);

        if !(self.proof)(vals) {
            return None;
        }
        let (pre, post) = prog.conditions();
        Some((self.eval)(vals, pre.clone(), post.clone()))
    }

    pub fn code(&self) -> UniCode {
        self.code
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
        level: Level,
    ) -> Box<dyn Program<O>> {
        Box::new(Self {
            arg,
            code,
            values,
            pre,
            post,
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
}
