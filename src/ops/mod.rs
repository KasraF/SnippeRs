use crate::cond::*;
use crate::store::Bank;

mod binary;
mod unary;
mod variable;

pub use binary::{BinBuilder, BinProgram};
pub use variable::Variable;

pub trait Program<T> {
    fn code(&self, store: &Bank) -> String;
    fn values<'s>(&self, store: &'s Bank) -> &'s [T];
    fn conditions(&self) -> (&PreCondition, &PostCondition);
}
