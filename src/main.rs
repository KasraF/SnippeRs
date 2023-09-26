#![feature(trait_alias)]
// TODO Remove!
#![allow(dead_code, unused_variables, unused_imports)]

use clap::Parser;
use synth::Synth;
use task::Task;

mod args;
mod ctx;
mod enumer;
mod nodes;
mod store;
mod synth;
mod task;
mod utils;
mod vocab;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    let task = Task::from_file(&args.task)?;
    let mut synth = Synth::new(task);
    println!("Solution: {:?}", synth.solve());
    Ok(())
}
