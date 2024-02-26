#![allow(dead_code)]
#![feature(try_trait_v2)]
#![feature(iterator_try_collect)]

use std::borrow::Borrow;

mod cond;
mod ops;
mod store;
mod synth;
mod task;
mod utils;
mod vocab;

pub(crate) use cond::*;
pub(crate) use ops::*;
pub(crate) use task::SynthesisTask;
pub(crate) use utils::*;

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
    let mut synth = synth::Synthesizer::new(vocab::vocab(), task);

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
