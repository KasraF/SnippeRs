use std::collections::HashMap;

use crate::utils::Anies;

pub struct SynthesisTask {
    /// Map each variable to a vector index,
    /// so we can use vecs instead of HashMaps
    /// to keep state.
    var_map: HashMap<String, usize>,
    before_state: HashMap<String, Anies>,
    examples: usize,
}

impl SynthesisTask {
    pub fn new(before_state: HashMap<String, Anies>, examples: usize) -> Self {
        let var_map = before_state
            .keys()
            .enumerate()
            .map(|(a, b)| (b.clone(), a))
            .collect();
        Self {
            var_map,
            examples,
            before_state,
        }
    }

    pub fn examples(&self) -> usize {
        self.examples
    }

    pub fn variables(&self) -> impl Iterator<Item = (&String, &Anies, &usize)> {
        self.var_map
            .iter()
            .map(|(name, idx)| (name, &self.before_state[name], idx))
    }
}
