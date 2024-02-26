use crate::cond::*;
use crate::store::*;
use crate::utils::*;

use super::Program;

pub struct Variable<T: Value> {
    name: String,
    values: VIdx<T>,
    cond: Condition,
    pointer: Pointer,
}

impl Variable<Str> {
    pub fn new(name: String, values: VIdx<Str>, pointer: Pointer, variables: usize) -> Box<Self> {
        let cond = Condition::empty(variables).mutate(pointer, Some(AnyVal::Str(values)));
        Box::new(Self {
            name,
            values,
            cond,
            pointer,
        })
    }
}

impl Variable<Int> {
    pub fn new(name: String, values: VIdx<Int>, pointer: Pointer, variables: usize) -> Box<Self> {
        let cond = Condition::empty(variables).mutate(pointer, Some(AnyVal::Int(values)));
        Box::new(Self {
            name,
            values,
            cond,
            pointer,
        })
    }
}

impl<T: Value> Program<T> for Variable<T>
where
    Bank: Store<T>,
{
    fn code(&self, _: &Bank) -> String {
        self.name.to_string()
    }

    fn values<'a>(&self, store: &'a Bank) -> &'a [T] {
        &store[self.values]
    }

    fn conditions(&self) -> (&PreCondition, &PostCondition) {
        (&self.cond, &self.cond)
    }

    fn level(&self) -> super::Level {
        0.into()
    }

    fn pointer(&self) -> Option<Pointer> {
        Some(self.pointer)
    }
}
