use std::cmp::max;

use crate::cond::*;
use crate::store::Bank;
use crate::*;

mod binary;
mod constant;
mod nullary;
mod unary;
mod variable;

pub(crate) use binary::{BinBuilder, BinEnumerator, BinMaybeProgram, BinProgram};
pub(crate) use constant::Constant;
pub(crate) use unary::{UniBuilder, UniMaybeProgram, UniProgram};
pub(crate) use variable::{MaybeVariable, Variable};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug)]
pub struct Level(u8);

impl From<u8> for Level {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Level> for u8 {
    fn from(value: Level) -> Self {
        value.0
    }
}

impl Level {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn prev(&self) -> Self {
        Self(self.0 - 1)
    }

    pub fn inc(&mut self) {
        self.0 += 1;
    }

    pub fn bin_next(&self, rhs: Self) -> Self {
        Self(max(self.0, rhs.0) + 1)
    }
}

pub trait MaybeProgram<T>
where
    T: Value,
{
    fn values(&self) -> Option<&[T]>;
    fn extract_values(&mut self) -> Option<Vec<T>>;
    fn pointer(&self) -> Option<Pointer>;
    fn pre_condition(&self) -> &PreCondition;
    fn post_condition(&self) -> &PostCondition;
    fn into_program(self: Box<Self>, values: VIdx<T>) -> Box<dyn Program<T>>;
}

pub trait Program<T: Value> {
    fn code(&self, store: &Bank) -> String;
    fn values<'s>(&self, store: &'s Bank) -> &'s [T];
    fn values_idx(&self) -> VIdx<T>;
    fn conditions(&self) -> (&PreCondition, &PostCondition);
    fn pointer(&self) -> Option<Pointer>;
    fn level(&self) -> Level;
}
