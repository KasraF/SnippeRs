use smallvec::{smallvec, SmallVec};

use crate::utils::*;

pub type PreCondition = Condition;
pub type PostCondition = Condition;

pub struct Condition {
    inner: SmallVec<[Option<AnyVal>; 4]>, // Up to 5 variables on the stack!
}

impl From<SmallVec<[Option<AnyVal>; 4]>> for Condition {
    fn from(inner: SmallVec<[Option<AnyVal>; 4]>) -> Self {
        Self { inner }
    }
}

impl Condition {
    pub fn empty(vars: usize) -> Self {
        Self {
            inner: smallvec![None; vars],
        }
    }

    pub fn implies(&self, other: &Condition) -> bool {
        debug_assert_eq!(self.inner.len(), other.inner.len());
        for i in 0..self.inner.len() {
            match (self.inner[i], other.inner[i]) {
                (Some(this), Some(that)) if this != that => return false,
                _ => (),
            }
        }

        true
    }

    /// Apply the sequence rule if possible and return the overall pre- and
    /// post condition for the sequence of the two statements.
    /// TODO This is in the hot-loop of the synthesizer. Optimize!
    pub fn sequence(
        fst: (&PreCondition, &PostCondition),
        snd: (&PreCondition, &PostCondition),
    ) -> Option<(PreCondition, PostCondition)> {
        if !fst.1.implies(snd.0) {
            return None;
        }

        let mut pre_condition = fst.0.inner.clone();
        let mut post_condition = fst.1.inner.clone();

        for i in 0..pre_condition.len() {
            if post_condition[i].is_none() {
                pre_condition[i] = snd.0.inner[i];
            }
        }

        for i in 0..post_condition.len() {
            post_condition[i] = snd.1.inner[i];
        }

        Some((pre_condition.into(), post_condition.into()))
    }

    pub fn mutate(&self, var: usize, val: Option<AnyVal>) -> Condition {
        let mut inner = self.inner.clone();
        inner[var] = val;
        Condition { inner }
    }
}
