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
