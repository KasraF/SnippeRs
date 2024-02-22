#![allow(dead_code)]

use std::borrow::Borrow;

mod cond;
mod ops;
mod store;
mod synth;
mod task;
mod utils;

pub(crate) use cond::*;
pub(crate) use ops::*;
use synth::Vocab;
pub(crate) use task::SynthesisTask;
pub(crate) use utils::*;

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
    let task = SynthesisTask::new(
        [
            ("x".to_string(), Anies::Int(vec![0])),
            ("y".to_string(), Anies::Int(vec![1])),
        ]
        .into(),
        1,
    );
    let vocab: Vocab = vec![BinBuilder::new(&sum_proof, &sum_eval, &sum_code).into()];
    let mut synth = synth::Synthesizer::new(vocab, task);

    loop {
        let prog = synth.next();
        let store = synth.store();
        let code = match prog.borrow() {
            AnyProg::Int(p) => store[*p].code(store),
            AnyProg::Str(p) => store[*p].code(store),
        };
        println!("{code}");
    }
}
