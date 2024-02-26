use crate::*;

pub(crate) fn len_eval(
    arg: &[Str],
    pre: PreCondition,
    post: PostCondition,
) -> Option<(Vec<Int>, PreCondition, PostCondition)> {
    let rs = arg.iter().map(|s| s.len() as i32).collect();
    Some((rs, pre, post))
}

pub(crate) fn len_code(arg: &str) -> String {
    format!("{arg}.length")
}

pub(crate) fn deref_eval(
    lhs: &[Str],
    rhs: &[Int],
    pre: PreCondition,
    post: PostCondition,
) -> Option<(Vec<Str>, PreCondition, PostCondition)> {
    let rs = lhs
        .iter()
        .zip(rhs)
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
    Some((rs, pre, post))
}

pub(crate) fn deref_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs}[{rhs}]")
}
