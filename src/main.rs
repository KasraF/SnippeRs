#![allow(dead_code)]
#![feature(try_trait_v2)]

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

fn bin_true(_: &[Int], _: &[Int]) -> bool {
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

fn sub_eval(
    lhs: &[Int],
    rhs: &[Int],
    pre: PreCondition,
    post: PostCondition,
) -> (Vec<Int>, PreCondition, PostCondition) {
    let rs = lhs.iter().zip(rhs).map(|(x, y)| x - y).collect();
    (rs, pre, post)
}

fn sub_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} - {rhs}")
}

fn len_proof(_: &[Str]) -> bool {
    true
}

fn len_eval(
    arg: &[Str],
    pre: PreCondition,
    post: PostCondition,
) -> (Vec<Int>, PreCondition, PostCondition) {
    let rs = arg.iter().map(|s| s.len() as i32).collect();
    (rs, pre, post)
}

fn len_code(arg: &str) -> String {
    format!("{arg}.length")
}

fn main() {
    let task = SynthesisTask::new(
        [
            ("x".to_string(), Anies::Int(vec![0, 2])),
            ("y".to_string(), Anies::Int(vec![1, 1])),
            (
                "s".to_string(),
                Anies::Str(vec!["a".to_string(), "asdfmovie".to_string()]),
            ),
        ]
        .into(),
        2,
    );
    let vocab: Vocab = vec![
        UniBuilder::new(&len_proof, &len_eval, &len_code).into(),
        BinBuilder::new(&bin_true, &sum_eval, &sum_code).into(),
        BinBuilder::new(&bin_true, &sub_eval, &sub_code).into(),
    ];
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
