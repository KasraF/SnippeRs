use crate::*;

use self::store::Bank;

pub(crate) fn sum_eval(
    lhs: &dyn Program<Int>,
    rhs: &dyn Program<Int>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Int>, Option<Mutation>, Option<Pointer>)> {
    let lhs_vals = lhs.values(store);
    let rhs_vals = rhs.values(store);
    let rs = lhs_vals
        .iter()
        .zip(rhs_vals)
        .map(|(x, y)| x.checked_add(*y))
        .try_collect()?;
    Some((rs, None, None))
}

pub(crate) fn sum_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

pub(crate) fn sub_eval(
    lhs: &dyn Program<Int>,
    rhs: &dyn Program<Int>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Int>, Option<Mutation>, Option<Pointer>)> {
    let rs = lhs
        .values(store)
        .iter()
        .zip(rhs.values(store))
        .map(|(x, y)| x.checked_sub(*y))
        .try_collect()?;
    Some((rs, None, None))
}

pub(crate) fn sub_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} - {rhs}")
}

pub(crate) fn minus_eval(
    arg: &dyn Program<Int>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Int>, Option<Mutation>, Option<Pointer>)> {
    let rs = arg
        .values(store)
        .iter()
        .map(|x| x.checked_neg())
        .try_collect()?;
    Some((rs, None, None))
}

pub(crate) fn minus_code(arg: &str) -> String {
    format!("-{arg}")
}

pub(crate) fn inc_eval(
    arg: &dyn Program<Int>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Int>, Option<Mutation>, Option<Pointer>)> {
    if let Some(pointer) = arg.pointer() {
        // This is a postscript increment (x++),
        // so we return the original value, but mutate the state
        let values = arg.values(store).to_vec();
        let post_cond = Some(Mutation::new(
            pointer,
            Anies::Int(values.iter().map(|x| x.checked_add(1)).try_collect()?),
        ));

        Some((values, post_cond, None))
    } else {
        None
    }
}

pub(crate) fn inc_code(arg: &str) -> String {
    format!("{arg}++")
}
