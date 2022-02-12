// TODO DELETEME
#![allow(dead_code)]

// TODO Use macros to generate the Store code for each type

// fn synthesize<T>(inp: Vec<Context>, out: T) -> Box<dyn Node<T>> {
//     Box::
// }

mod enumerator;
mod nodes;
mod oe;
mod store;
mod utils;

use crate::enumerator::Enumerator;
use crate::nodes::binary::{BinaryGenerator, IntAddition};
use crate::utils::GeneratorStore;
use crate::utils::{Error, Idx};

fn main() -> Result<(), Error> {
    let contexts = 1;	
    let gen = BinaryGenerator::new(1, Idx::new(0, 0), Idx::new(0, 0), &(IntAddition::new));

    let generator_store = GeneratorStore {
        int: vec![Box::new(gen)],
        string: Vec::new(),
        boolean: Vec::new(),
    };

    let mut enumerator = Enumerator::new(1, contexts, generator_store);

    dbg!(enumerator.next());

    Ok(())
}
