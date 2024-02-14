#![allow(dead_code)]

use std::collections::HashMap;

mod cond;
mod ops;
mod store;
mod utils;

use cond::*;
use ops::*;
use store::*;
use utils::*;

struct SynthesisTask {
    /// Map each variable to a vector index,
    /// so we can use vecs instead of HashMaps
    /// to keep state.
    var_map: HashMap<String, usize>,
    examples: usize,
}

fn sum_proof(_: &[Int], _: &[Int]) -> bool {
    true
}

fn sum_eval(
    lhs: &[Int],
    rhs: &[Int],
    pre: PreCondition,
    post: PostCondition,
) -> (Vec<Int>, PreCondition, PostCondition) {
    let rs = lhs.iter().zip(rhs).map(|(x, y)| x + y).collect();
    (rs, pre, post)
}

fn sum_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

fn main() {
    let mut store = Bank::new(1);
    let task = SynthesisTask {
        var_map: [("x".to_string(), 0), ("y".to_string(), 1)].into(),
        examples: 1,
    };

    let one_val = store.put_values(&[1]);
    let x: PIdx<Int> = store.put_program(Box::new(Variable::<Int>::new(
        "x".to_string(),
        one_val,
        &task,
    )));
    let two_val = store.put_values(&[2]);
    let y = store.put_program(Box::new(Variable::<Int>::new(
        "y".to_string(),
        two_val,
        &task,
    )));

    let sum = BinBuilder::new(&sum_proof, &sum_eval, &sum_code);
    match sum.apply(x, y, &store) {
        Some((vals, pre, post)) => {
            assert_eq!(&vals, &[3]);
            let three_val = store.put_values(&vals);
            let three = BinProgram::new(x, y, three_val, sum.code(), pre, post);
            let three = store.put_program(three);
            assert_eq!(store[three].values(&store), &[3]);
            let code = &store[three].code(&store);
            assert_eq!(code, "x + y");
            println!("Synthesized {code}");
        }
        None => unreachable!(),
    }
}
