use crate::*;

use self::store::Bank;

pub(crate) fn sum_eval(
    lhs: &dyn Program<Int>,
    rhs: &dyn Program<Int>,
    cond: Condition,
    store: &Bank,
) -> Option<(Vec<Int>, PostCondition, Option<Pointer>)> {
    let lhs_vals = lhs.values(store);
    let rhs_vals = rhs.values(store);
    let rs = lhs_vals
        .iter()
        .zip(rhs_vals)
        .map(|(x, y)| x.checked_add(*y))
        .try_collect()?;
    Some((rs, cond, None))
}

pub(crate) fn sum_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

pub(crate) fn sub_eval(
    lhs: &dyn Program<Int>,
    rhs: &dyn Program<Int>,
    cond: Condition,
    store: &Bank,
) -> Option<(Vec<Int>, PostCondition, Option<Pointer>)> {
    let rs = lhs
        .values(store)
        .iter()
        .zip(rhs.values(store))
        .map(|(x, y)| x.checked_sub(*y))
        .try_collect()?;
    Some((rs, cond, None))
}

pub(crate) fn sub_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} - {rhs}")
}

pub(crate) fn minus_eval(
    arg: &dyn Program<Int>,
    cond: Condition,
    store: &Bank,
) -> Option<(Vec<Int>, PostCondition, Option<Pointer>)> {
    let rs = arg
        .values(store)
        .iter()
        .map(|x| x.checked_neg())
        .try_collect()?;
    Some((rs, cond, None))
}

pub(crate) fn minus_code(arg: &str) -> String {
    format!("-{arg}")
}
