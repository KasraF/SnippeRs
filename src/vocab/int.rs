use crate::*;

pub(crate) fn sum_eval(
    lhs: &[Int],
    rhs: &[Int],
    pre: PreCondition,
    post: PostCondition,
) -> Option<(Vec<Int>, PreCondition, PostCondition)> {
    debug_assert_eq!(lhs.len(), rhs.len());

    let rs = lhs
        .iter()
        .zip(rhs)
        .map(|(x, y)| x.checked_add(*y))
        .try_collect()?;
    Some((rs, pre, post))
}

pub(crate) fn sum_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

pub(crate) fn sub_eval(
    lhs: &[Int],
    rhs: &[Int],
    pre: PreCondition,
    post: PostCondition,
) -> Option<(Vec<Int>, PreCondition, PostCondition)> {
    let rs = lhs
        .iter()
        .zip(rhs)
        .map(|(x, y)| x.checked_sub(*y))
        .try_collect()?;
    Some((rs, pre, post))
}

pub(crate) fn sub_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} - {rhs}")
}

pub(crate) fn pow_eval(
    lhs: &[Int],
    rhs: &[Int],
    pre: PreCondition,
    post: PostCondition,
) -> Option<(Vec<Int>, PreCondition, PostCondition)> {
    let rs = lhs
        .iter()
        .zip(rhs)
        .map(|(x, y)| {
            if *y < 0 {
                None
            } else {
                x.checked_pow(*y as u32)
            }
        })
        .try_collect()?;
    Some((rs, pre, post))
}

pub(crate) fn pow_code(lhs: &str, rhs: &str) -> String {
    format!("Math.pow({lhs}, {rhs})")
}
