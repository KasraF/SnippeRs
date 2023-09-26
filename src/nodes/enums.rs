use crate::{
    ctx::{Context, Ctx, VarMap, VariableMap},
    store::{ProgramStore, Store},
    utils::{Int, Val},
};

use super::{binary::BinEnum, nullary::VariableNodeEnum, NodeEnum};

// ---
// Variables
// ---

pub fn variable_node_enum<'s, T: Val>(store: &'s Store) -> Box<dyn NodeEnum<T>>
where
    Context: Ctx<T>,
    Store: ProgramStore<T>,
    VariableMap: VarMap<T>,
{
    Box::new(VariableNodeEnum::new(store))
}

// ---
// Binary
// ---
fn true_check<L, R>(_lhs: &[L], _rhs: &[R]) -> bool {
    true
}

fn add_op(lhs: &[Int], rhs: &[Int]) -> Vec<Int> {
    debug_assert!(
        lhs.len() == rhs.len(),
        "Arrays of different lengths given to add_op: {:?} vs. {:?}",
        lhs,
        rhs
    );
    lhs.iter().zip(rhs.iter()).map(|(l, r)| l + r).collect()
}

fn add_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

pub fn int_add<'s>(store: &'s Store) -> Box<dyn NodeEnum<Int> + 's> {
    Box::new(BinEnum::new(&add_op, &true_check, &add_code, store))
}
