use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader};

use crate::utils::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Typ {
    Int,
    Str,
    Bool,
    IntArray,
    StrArray,
    BoolArray,
}

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
}
