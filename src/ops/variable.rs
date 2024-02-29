use crate::cond::*;
use crate::store::*;
use crate::utils::*;

use super::Program;

pub struct MaybeVariable<T: Value> {
    name: String,
    values: Option<Vec<T>>,
    pointer: Pointer,
    variables: usize,
}

impl<T: Value> MaybeVariable<T> {
    pub fn new(name: String, values: Vec<T>, pointer: Pointer, variables: usize) -> Self {
        Self {
            name,
            values: Some(values),
            variables,
            pointer,
        }
    }
}

impl MaybeVariable<Int> {
    pub fn into_program(self: Box<Self>, values: VIdx<Int>) -> Box<dyn Program<Int>> {
        Box::new(Variable {
            name: self.name,
            values,
            cond: Condition::empty(self.variables)
                .mutate_with_index(self.pointer, Some(AnyVal::Int(values))),
            pointer: self.pointer,
        })
    }
}

impl MaybeVariable<Str> {
    pub fn into_program(self: Box<Self>, values: VIdx<Str>) -> Box<dyn Program<Str>> {
        Box::new(Variable {
            name: self.name,
            values,
            cond: Condition::empty(self.variables)
                .mutate_with_index(self.pointer, Some(AnyVal::Str(values))),
            pointer: self.pointer,
        })
    }
}

pub struct Variable<T: Value> {
    name: String,
    values: VIdx<T>,
    cond: Condition,
    pointer: Pointer,
}

impl Variable<Str> {
    pub fn new(name: String, values: VIdx<Str>, pointer: Pointer, variables: usize) -> Box<Self> {
        let cond =
            Condition::empty(variables).mutate_with_index(pointer, Some(AnyVal::Str(values)));
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
        let cond =
            Condition::empty(variables).mutate_with_index(pointer, Some(AnyVal::Int(values)));
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

    fn values_idx(&self) -> VIdx<T> {
        self.values
    }
}
