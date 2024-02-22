use crate::utils::Value;

#[derive(Clone)]
pub struct VarBuilder<T>
where
    T: Value,
{
    name: String,
    value: Vec<T>,
}
