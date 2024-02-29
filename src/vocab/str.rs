use crate::*;

use self::store::Bank;

pub(crate) fn len_eval(
    arg: &dyn Program<Str>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Int>, Option<Mutation>, Option<Pointer>)> {
    let rs = arg.values(store).iter().map(|s| s.len() as i32).collect();
    Some((rs, None, None))
}

pub(crate) fn len_code(arg: &str) -> String {
    format!("{arg}.length")
}

pub(crate) fn deref_eval(
    lhs: &dyn Program<Str>,
    rhs: &dyn Program<Int>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Str>, Option<Mutation>, Option<Pointer>)> {
    let rs = lhs
        .values(store)
        .iter()
        .zip(rhs.values(store))
        .map(|(s, i)| -> Option<Str> {
            if *i >= 0 {
                let chars = s.as_ascii()?;
                let char = chars.get(*i as usize)?;
                Some(char.to_string())
            } else {
                None
            }
        })
        .try_collect()?;
    Some((rs, None, None)) // JavaScript strings are immutable, so this is bottom
}

pub(crate) fn deref_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs}[{rhs}]")
}
