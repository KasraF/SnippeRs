use crate::*;

use self::store::{Bank, Store};

pub struct Constant<T: Value> {
    name: String,
    values: VIdx<T>,
    condition: Condition,
}

impl<T> Constant<T>
where
    T: Value,
    Bank: Store<T>,
{
    pub fn new(name: String, values: VIdx<T>, vars: usize) -> Box<dyn Program<T>> {
        Box::new(Self {
            name,
            values,
            condition: Condition::empty(vars),
        })
    }
}

impl<T> Program<T> for Constant<T>
where
    T: Value,
    Bank: Store<T>,
{
    fn code(&self, _: &store::Bank) -> String {
        self.name.clone()
    }

    fn values<'s>(&self, store: &'s store::Bank) -> &'s [T] {
        &store[self.values]
    }

    fn conditions(&self) -> (&PreCondition, &PostCondition) {
        (&self.condition, &self.condition)
    }

    fn level(&self) -> Level {
        0.into()
    }

    fn pointer(&self) -> Option<Pointer> {
        None
    }
}
