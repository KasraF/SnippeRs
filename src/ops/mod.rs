use std::cmp::max;

use crate::cond::*;
use crate::store::Bank;

mod binary;
mod nullary;
mod unary;
mod variable;

pub(crate) use binary::{BinBuilder, BinEnumerator, BinProgram};
pub(crate) use unary::{UniBuilder, UniProgram};
pub(crate) use variable::Variable;

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

pub trait Program<T> {
    fn code(&self, store: &Bank) -> String;
    fn values<'s>(&self, store: &'s Bank) -> &'s [T];
    fn conditions(&self) -> (&PreCondition, &PostCondition);
    fn level(&self) -> Level;
}
