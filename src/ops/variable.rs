use crate::cond::*;
use crate::store::*;
use crate::utils::*;
use crate::SynthesisTask;

use super::Program;

pub struct Variable<T: Value> {
    name: String,
    values: VIdx<T>,
    cond: Condition,
}

impl Variable<Str> {
    pub fn new(name: String, values: VIdx<Str>, task: &SynthesisTask) -> Self {
        let cond = Condition::empty(task.var_map.len()).mutate(
            *task.var_map.get(&name).expect(&format!(
                "Variable initalized, but name doesn't exist in var map: {name}"
            )),
            Some(AnyVal::Str(values)),
        );
        Self { name, values, cond }
    }
}

impl Variable<Int> {
    pub fn new(name: String, values: VIdx<Int>, task: &SynthesisTask) -> Self {
        let cond = Condition::empty(task.var_map.len()).mutate(
            *task.var_map.get(&name).expect(&format!(
                "Variable initalized, but name doesn't exist in var map: {name}"
            )),
            Some(AnyVal::Int(values)),
        );
        Self { name, values, cond }
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
}
