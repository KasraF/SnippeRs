use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader};

use crate::{
    ctx::{Context, Contexts, Ctx, Var, VarMap, VariableMap},
    utils::*,
};

impl<'de> Deserialize<'de> for Typ {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Int" => Ok(Typ::Int),
            "Str" => Ok(Typ::Str),
            "Bool" => Ok(Typ::Bool),
            "[Int]" => Ok(Typ::IntArray),
            "[Str]" => Ok(Typ::StrArray),
            "[Bool]" => Ok(Typ::BoolArray),
            s => Err(serde::de::Error::unknown_variant(
                s,
                &["Int", "Str", "Bool", "[Int]", "[Str]", "[Bool]"],
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Example {
    pub input: HashMap<String, Value>,
    pub state: Option<HashMap<String, Value>>,
    pub output: Option<Value>,
}

impl Example {
    fn to_context(&self, map: &mut VariableMap) -> Context {
        let ctx = Context::default();
        for (var, val) in &self.input {
            match val {
                Value::Int(val) => {
                    let idx: Var<Int> = map.insert(&var);
                    ctx.set(idx, *val);
                }
                Value::Str(val) => todo!(),
                Value::Bool(val) => todo!(),
                Value::IntArray(val) => {
                    let idx: Var<IntArray> = map.insert(&var);
                    ctx.set(idx, val.clone());
                }
                Value::StrArray(val) => todo!(),
                Value::BoolArray(val) => todo!(),
            }
        }
        ctx
    }
}

/// A synthesis "Task", parsed from JSON.
#[derive(Deserialize, Debug)]
pub struct Task {
    /// A URL pointing to the source/inspiration for this task.
    #[serde(rename = "source")]
    pub src: String,
    /// The available variables, and their types.
    #[serde(rename = "variables")]
    pub vars: HashMap<String, Typ>,
    /// Additional Int literals.
    #[serde(rename = "intLiterals", default)]
    pub int_lit: Vec<Int>,
    /// Additional Str literals.
    #[serde(rename = "strLiterals", default)]
    pub str_lit: Vec<String>,
    /// The return type of the task. `None` if the task only modifies the state.
    #[serde(rename = "returnType")]
    pub ret_typ: Option<Typ>,
    /// The set of examples. Cannot be empty.
    #[serde(rename = "examples")]
    pub examples: Vec<Example>,
    /// A list of hand-written "solutions".
    #[serde(rename = "solution")]
    pub sol: Option<Vec<String>>,
}

impl Task {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rs = serde_json::from_reader(reader)?;
        Ok(rs)
    }

    pub fn get_contexts(&self) -> (VariableMap, Contexts) {
        let mut varmap = VariableMap::default();
        let ctx = self
            .examples
            .iter()
            .map(|ex| ex.to_context(&mut varmap))
            .collect();
        (varmap, ctx)
    }
}

#[cfg(test)]
mod task_tests {
    use super::*;

    #[test]
    fn basic_parsing() {
        let task = "{
            \"source\": \"blah\",
            \"variables\": {
                \"list\": \"[Int]\"
            },
            \"returnType\": \"Int\",
            \"examples\": [
                {
                    \"input\": {
                        \"list\": [-99, 88, -32, 3, 10, 999, 9991, 0, 99]
                    },
                    \"output\": 9991
                }
            ]
        }";
        let rs = serde_json::from_str::<Task>(task);
        assert!(rs.is_ok(), "Failed to parse task: {:?}", rs);

        let rs = rs.unwrap();
        assert_eq!(rs.src, "blah");
        assert!(rs.vars.contains_key("list"));
        assert_eq!(rs.vars.get("list"), Some(&Typ::IntArray));
    }

    fn to_ctx() {
        let task = "{
            \"source\": \"blah\",
            \"variables\": {
                \"list\": \"[Int]\"
            },
            \"returnType\": \"Int\",
            \"examples\": [
                {
                    \"input\": {
                        \"list\": [-99, 88, -32, 3, 10, 999, 9991, 0, 99]
                    },
                    \"output\": 9991
                }
            ]
        }";
        let rs = serde_json::from_str::<Task>(task);
        assert!(rs.is_ok(), "Failed to parse task: {:?}", rs);

        let rs = rs.unwrap();
        let (mut varmap, ctxs) = rs.get_contexts();
        assert_eq!(
            ctxs.len(),
            1,
            "Expected 1 context, but found {}: {:?}",
            ctxs.len(),
            ctxs
        );

        let list: Var<IntArray> = varmap.insert("list");
        assert_eq!(
            *list, 0,
            "'list' variable wasn't inserted at the beginning: {} ({:?})",
            list, varmap
        );

        let val: Vec<&IntArray> = ctxs.iter().map(|ctx| ctx.get(list)).collect();
        assert_eq!(
            val.len(),
            1,
            "Expected 1 context, found {}: {:?}",
            val.len(),
            val
        );
        assert_eq!(
            val[0].as_slice(),
            [-99, 88, -32, 3, 10, 999, 9991, 0, 99],
            "Incorrect value for list: {:?}",
            val[0]
        );
    }
}
