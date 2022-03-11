use crate::{
    task::{InputValue, Task},
    utils::{GenericPred, Int, Predicate, Str},
    Program,
};

pub enum ValuePredicate {
    Int(Vec<Int>),
    Str(Vec<Str>),
}

impl ValuePredicate {
    pub fn new(task: &Task) -> Self {
        let outputs: Vec<InputValue> = task.outputs();
        match outputs[0] {
            InputValue::Int(_) => {
                let values: Vec<Int> = outputs
                    .iter()
                    .map(|o| match o {
                        InputValue::Int(i) => *i,
                        _ => unreachable!(),
                    })
                    .collect();
                Self::Int(values)
            }
            InputValue::Str(_) => {
                let values: Vec<Str> = outputs
                    .iter()
                    .map(|o| match o {
                        InputValue::Str(s) => s.clone(),
                        _ => unreachable!(),
                    })
                    .collect();
                Self::Str(values)
            }
            _ => unreachable!(),
        }
    }
}

impl GenericPred<Int> for ValuePredicate {
    fn matches(&self, program: &dyn Program<Int>) -> bool {
        if let ValuePredicate::Int(is) = self {
            program.values() == is
        } else {
            false
        }
    }
}

impl GenericPred<Str> for ValuePredicate {
    fn matches(&self, program: &dyn Program<Str>) -> bool {
        if let ValuePredicate::Str(ss) = self {
            program.values() == ss
        } else {
            false
        }
    }
}

impl Predicate for ValuePredicate {}
