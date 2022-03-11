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

fn unary_true_validator<A>(_: &[A]) -> bool {
    true
}

fn bin_true_validator<L, R>(_: &[L], _: &[R]) -> bool {
    true
}
fn add_value((lhs, rhs): (&Int, &Int)) -> Int {
    lhs + rhs
}

fn add_code(lhs: &str, rhs: &str) -> String {
    format!("{} + {}", lhs, rhs)
}

fn sub_value((lhs, rhs): (&Int, &Int)) -> Int {
    lhs - rhs
}

fn sub_code(lhs: &str, rhs: &str) -> String {
    format!("{} - {}", lhs, rhs)
}

fn to_string_code(arg: &str) -> String {
    format!("str({})", arg)
}

fn to_string_value(arg: &Int) -> String {
    arg.to_string()
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
        }
    }

    Ok(())
}
