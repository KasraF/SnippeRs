#![allow(dead_code)]
#![feature(try_trait_v2)]
#![feature(iterator_try_collect)]
#![feature(ascii_char)]

use std::io::Write;

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

fn main() -> Result<(), utils::Error> {
    let mut stdout = std::io::stdout();
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
        let code = prog.code(store);
        let (pre, post) = prog.conditions(store);

        pre.pretty_print(&mut stdout, store)?;
        write!(stdout, " {code} ")?;
        post.pretty_print(&mut stdout, store)?;
        write!(stdout, "\n")?;
        stdout.flush()?;
    }
}
