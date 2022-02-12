use crate::nodes::literal::IntLiteral;
use crate::oe::{OECache, OE};
use crate::store::{NodeStore, Store};
use crate::utils::GeneratorStore;

pub struct Enumerator {
    lvl: usize,
    oe_cache: OECache,
    store: Store,
    generators: GeneratorStore,
    current_gen: usize,
}

impl Enumerator {
    pub fn new(lvl: usize, contexts: usize, generators: GeneratorStore) -> Self {
        let mut oe_cache = OECache::new();
        let mut store = Store::new(contexts);

        // TODO Where should this go?
        // Add our literals
        for i in -1..3 {
            let (node, values) = IntLiteral::new(i, contexts);
            oe_cache.insert(&values);
            store.levels[0].int_nodes.push(node);
            store.levels[0].int_values.extend(values);
        }

        Self {
            lvl,
            oe_cache,
            store,
            generators,
            current_gen: 0,
        }
    }

    pub fn next(&mut self) -> Option<String> {
        let mut next = self.generators.int[self.current_gen].next(&self.store);
        while next.is_none() {
            self.current_gen += 1;
            if self.current_gen >= self.generators.int.len() {
                return None;
            } else {
                next = self.generators.int[self.current_gen].next(&self.store);
            }
        }

        let (node, values) = next.unwrap();

        // We need to do some housekeeping
        // First, check that this node is not OE to one we have
        if self.oe_cache.is_unique(&values) {
            // Then, add both the node and its values to the store
            let code = node.code(&self.store);
            let idx = self.store.put(node, values);
            self.oe_cache.insert(self.store.get_values(&idx).unwrap());

            // Finally, we can return it!
            Some(code)
        } else {
            // Otherwise, grab the next one!
            self.next()
        }
    }
}
