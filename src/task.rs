use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::BuildHasher,
};

use serde::Deserialize;

use crate::utils::*;

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
enum InputValue {
    Wot,
    Int(i32),
    Str(String),
}

impl BuildHasher for InputValue {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        DefaultHasher::new()
    }
}

impl Default for InputValue {
    fn default() -> Self {
        InputValue::Wot
    }
}

#[derive(Deserialize, Debug)]
pub struct Example {
    inputs: HashMap<String, InputValue>,
    output: InputValue,
}

#[derive(Deserialize, Debug)]
pub struct Task {
    examples: Vec<Example>,
}

impl Task {
    pub fn validate(&self) -> Result<(), String> {
        if self.examples.is_empty() {
            return Err("No examples provided.".to_string());
        }

        let variables = &self.examples[0]
            .inputs
            .keys()
            .fold(HashSet::new(), |mut acc, s| {
                acc.insert(s);
                acc
            });

        for (var, val) in &self.examples[0].inputs {
            match val {
                InputValue::Wot => {
                    return Err(format!("Input variable type not recognized: {}", var))
                }
                _ => (),
            }
        }

        for example in &self.examples[1..] {
            let context = &example.inputs;

            let vars = context.keys().fold(HashSet::new(), |mut acc, s| {
                acc.insert(s);
                acc
            });
            let missing: HashSet<_> = vars.symmetric_difference(&variables).collect();

            if !missing.is_empty() {
                return Err(format!(
                    "Some variables missing from some inputs: {:?}",
                    missing
                ));
            }
            drop(vars);

            for &var in variables {
                match (&self.examples[0].inputs[var], &context[var]) {
                    (&InputValue::Int(_), &InputValue::Int(_))
                    | (&InputValue::Str(_), &InputValue::Str(_)) => (),
                    (_, _) => {
                        return Err(format!("Incorrect variable type: {}", var));
                    }
                }
            }
        }

        let first_out = &self.examples[0].output;

        if first_out == &InputValue::Wot {
            return Err("Failed to recognize output value type".to_string());
        }

        for out in self.examples.iter().map(|ex| &ex.output) {
            match (first_out, out) {
                (InputValue::Int(_), InputValue::Int(_))
                | (InputValue::Str(_), InputValue::Str(_)) => (),
                (_, _) => {
                    return Err("Incorrect output type.".to_string());
                }
            }
        }

        Ok(())
    }

    pub fn ints(&self) -> Vec<(String, Vec<Int>)> {
        let mut rs = Vec::new();
        for (var, value) in &self.examples[0].inputs {
            match value {
                InputValue::Int(_) => {
                    rs.push((
                        var.to_string(),
                        self.examples
                            .iter()
                            .map(|ex| match ex.inputs[var] {
                                InputValue::Int(v) => v,
                                _ => unreachable!(),
                            })
                            .collect(),
                    ));
                }
                _ => (),
            }
        }
        rs
    }

    pub fn strs(&self) -> Vec<(String, Vec<Str>)> {
        let mut rs = Vec::new();
        for (var, value) in &self.examples[0].inputs {
            match value {
                InputValue::Str(_) => {
                    rs.push((
                        var.to_string(),
                        self.examples
                            .iter()
                            .map(|ex| -> String {
                                match &ex.inputs[var] {
                                    InputValue::Str(v) => v.clone(),
                                    _ => unreachable!(),
                                }
                            })
                            .collect(),
                    ));
                }
                _ => (),
            }
        }
        rs
    }
}
