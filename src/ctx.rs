use smallvec::SmallVec;

use crate::utils::*;

pub type Contexts = Vec<Context>;

trait Ctx<T: Val> {
    fn has(&self, idx: Index<T>) -> bool;
    fn try_get(&self, idx: Index<T>) -> Option<&T>;
    fn get(&self, idx: Index<T>) -> &T {
        self.try_get(idx).unwrap()
    }
    fn set(&self, idx: Index<T>, val: T) -> Self;
}

#[derive(Clone)]
pub struct Context {
    ints: SmallVec<[Int; 4]>,
    int_arrs: SmallVec<[IntArray; 4]>,
}

impl Ctx<Int> for Context {
    fn try_get(&self, idx: Index<Int>) -> Option<&Int> {
        if self.ints.len() < idx.idx {
            Some(&self.ints[idx.idx])
        } else {
            None
        }
    }

    fn set(&self, idx: Index<Int>, val: Int) -> Self {
        debug_assert!(
            self.has(idx),
            "Tried to set variable {} for context, but it doesn't exist.",
            idx
        );
        let mut rs = self.clone();
        rs.ints[idx.idx] = val;
        rs
    }

    fn has(&self, idx: Index<Int>) -> bool {
        self.ints.len() < idx.idx
    }
}

impl Ctx<IntArray> for Context {
    fn try_get(&self, idx: Index<IntArray>) -> Option<&IntArray> {
        if self.int_arrs.len() < idx.idx {
            Some(&self.int_arrs[idx.idx])
        } else {
            None
        }
    }

    fn set(&self, idx: Index<IntArray>, val: IntArray) -> Self {
        debug_assert!(
            self.has(idx),
            "Tried to set variable {} for context, but it doesn't exist.",
            idx
        );
        let mut rs = self.clone();
        rs.int_arrs[idx.idx] = val;
        rs
    }

    fn has(&self, idx: Index<IntArray>) -> bool {
        self.int_arrs.len() < idx.idx
    }
}
