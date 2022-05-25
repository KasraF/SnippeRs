#![feature(trait_alias)]

use crate::store::Store;
use crate::synth::*;
use crate::task::Task;
use crate::utils::*;
use std::collections::HashSet;
use std::fs::File;

mod nodes;
mod predicates;
mod store;
mod synth;
mod task;
mod utils;

pub trait Enumerator<T> {
    fn next(&mut self, store: &Store) -> Option<Box<dyn Program<T>>>;
    fn has_next(&self, store: &Store) -> bool;
}

pub trait ProgramStore<T: Value> {
    fn get(&self, index: Index<T>) -> Option<&Box<dyn Program<T>>>;
    fn put(&mut self, program: Box<dyn Program<T>>) -> Option<Index<T>>;
    fn get_unchecked(&self, index: Index<T>) -> &Box<dyn Program<T>>;
    fn has(&self, index: Index<T>) -> bool;
}

pub trait Program<T> {
    fn values(&self) -> &[T];
    fn size(&self) -> u8;
    fn code(&self, store: &Store) -> String;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a task!
    let mut args = std::env::args();
    let _ = args.next();
    let file_path = match args.next() {
        Some(path) => path,
        None => {
            println!("Please provide the path to the synthesis task.");
            std::process::exit(1);
        }
    };

    let task: Task = serde_json::from_reader(File::open(file_path)?)?;
    dbg!(&task);
    task.validate()?;

    let synth = Synthesizer::new(task);

    for p in synth {
        if let Some(code) = p {
            println!("Done: {}", code);
            break;
        }
    }

    Ok(())
}
