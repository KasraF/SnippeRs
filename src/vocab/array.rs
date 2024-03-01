use crate::*;

pub(crate) fn push_eval<T>(
    arr: &dyn Program<Array<T>>,
    elem: &dyn Program<T>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Int>, Option<Mutation>, Option<Pointer>)>
where
    T: Value,
    Array<T>: Value,
    Anies: From<Vec<Array<T>>>,
{
    let arr_values = arr.values(store);

    // TODO This check is overkill
    let rs = arr_values
        .iter()
        .map(|arr| -> Option<Int> {
            let rs = Int::try_from(arr.len()).ok()?;
            rs.checked_add(1)
        })
        .try_collect()?;

    let mutation = if let Some(pointer) = arr.pointer() {
        let values: Vec<Vec<T>> = arr_values
            .iter()
            .zip(elem.values(store))
            .map(|(arr, elem)| {
                let mut rs = arr.clone();
                rs.push(elem.clone());
                rs
            })
            .collect();
        Some(Mutation::new(pointer, values.into()))
    } else {
        None
    };

    Some((rs, mutation, None))
}

pub(crate) fn push_code(arr: &str, elem: &str) -> String {
    format!("{arr}.push({elem})")
}

pub(crate) fn deref_eval<T>(
    arr: &dyn Program<Array<T>>,
    idx: &dyn Program<Int>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<T>, Option<Mutation>, Option<Pointer>)>
where
    T: Value,
    Array<T>: Value,
{
    let rs = arr
        .values(store)
        .iter()
        .zip(idx.values(store))
        .map(|(arr, idx)| {
            if *idx >= 0 {
                arr.get(*idx as usize).map(|x| x.clone())
            } else {
                None
            }
        })
        .try_collect()?;
    Some((rs, None, None)) // TODO We should allow pointers to array elements.
}

pub(crate) fn deref_code(arr: &str, idx: &str) -> String {
    format!("{arr}[{idx}]")
}

pub(crate) fn len_eval<T>(
    arr: &dyn Program<Array<T>>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Int>, Option<Mutation>, Option<Pointer>)>
where
    Array<T>: Value,
{
    Some((
        arr.values(store)
            .iter()
            .map(|arr| arr.len().try_into().ok())
            .try_collect()?,
        None,
        None,
    ))
}

pub(crate) fn len_code(arr: &str) -> String {
    format!("{arr}.length")
}

pub(crate) fn bin_slice_eval<T>(
    arr: &dyn Program<Array<T>>,
    idx: &dyn Program<Int>,
    _: &Condition,
    store: &Bank,
) -> Option<(Vec<Array<T>>, Option<Mutation>, Option<Pointer>)>
where
    T: Value,
    Array<T>: Value,
{
    let rs = arr
        .values(store)
        .iter()
        .zip(idx.values(store))
        .map(|(arr, &idx)| {
            let idx = if idx < 0 {
                std::cmp::max(0, idx + arr.len() as Int) as usize
            } else if idx as usize > arr.len() {
                arr.len()
            } else {
                idx as usize
            };

            arr[idx..].to_vec()
        })
        .collect();

    Some((rs, None, None))
}

pub(crate) fn bin_slice_code(arr: &str, idx: &str) -> String {
    format!("{arr}.slice({idx})")
}
