use std::collections::HashMap;

use crate::utils::Anies;
use smallvec::SmallVec;

pub type VarMap = SmallVec<[String; 4]>;

pub struct SynthesisTask {
    /// Map each variable to a vector index,
    /// so we can use vecs instead of HashMaps
    /// to keep state.
    pub var_map: VarMap, // TODO No! Bad Crab!
    before_state: HashMap<String, Anies>,
    examples: usize,
}

impl SynthesisTask {
    pub fn new(before_state: HashMap<String, Anies>, examples: usize) -> Self {
        let var_map = before_state.keys().map(|n| n.clone()).collect();
        Self {
            var_map,
            examples,
            before_state,
        }
    }

    pub fn examples(&self) -> usize {
        self.examples
    }

    pub fn variables(&self) -> impl Iterator<Item = (&String, &Anies, usize)> {
        self.var_map
            .iter()
            .enumerate()
            .map(|(idx, name)| (name, &self.before_state[name], idx))
    }
}
