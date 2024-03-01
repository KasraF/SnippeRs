use smallvec::{smallvec, SmallVec};

use crate::{
    store::{Bank, Store},
    utils::*,
};

pub type Pointer = usize;
pub type PreCondition = Condition;
pub type PostCondition = Condition;

pub struct Mutation {
    pointer: Pointer,
    values: Anies,
}

impl Mutation {
    pub fn new(pointer: Pointer, values: Anies) -> Self {
        Self { pointer, values }
    }

    /// This function "applies" the mutation.
    /// It simply means adding the mutation as a new variable to the store,
    /// and returning a new condition containing the mutated value for the variable.
    pub fn apply(self, cond: Condition, store: &mut Bank) -> Condition {
        // First, add the variable
        let name = store.var_map()[self.pointer].clone();

        // TODO can we let the type system handle this?
        match self.values {
            Anies::Int(values) => {
                let prog_idx = match store.put_variable(name, values, self.pointer) {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                };
                let val_idx = store[prog_idx].values_idx();
                cond.mutate_with_index(self.pointer, Some(AnyVal::Int(val_idx)))
            }
            Anies::Str(values) => {
                let prog_idx = match store.put_variable(name, values, self.pointer) {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                };
                let val_idx = store[prog_idx].values_idx();
                cond.mutate_with_index(self.pointer, Some(AnyVal::Str(val_idx)))
            }
            Anies::IntArray(values) => {
                let prog_idx = match store.put_variable(name, values, self.pointer) {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                };
                let val_idx = store[prog_idx].values_idx();
                cond.mutate_with_index(self.pointer, Some(AnyVal::IntArray(val_idx)))
            }
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
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
            if snd.1.inner[i].is_some() {
                post_condition[i] = snd.1.inner[i];
            }
        }

        // println!(
        //     "fst: {:?}\nsnd: {:?}\npre: {:?}\npost: {:?}",
        //     &fst, &snd, &pre_condition, &post_condition
        // );

        Some((pre_condition.into(), post_condition.into()))
    }

    pub fn mutate_with_index(&self, var: Pointer, val: Option<AnyVal>) -> Condition {
        let mut inner = self.inner.clone();
        inner[var] = val;
        Condition { inner }
    }

    pub fn pretty_print(&self, out: &mut dyn std::io::Write, store: &Bank) -> std::io::Result<()> {
        write!(out, "{{ ")?;

        for (i, val) in self.inner.iter().enumerate() {
            let name = &store.var_map()[i];
            write!(out, "{name} -> ")?;

            match val {
                Some(idx) => {
                    match idx {
                        AnyVal::Int(idx) => {
                            let values = &store.get_values(*idx);
                            write!(out, "{values:?}")?;
                        }
                        AnyVal::Str(idx) => {
                            let values = &store.get_values(*idx);
                            write!(out, "{values:?}")?;
                        }
                        AnyVal::IntArray(idx) => {
                            let values = &store.get_values(*idx);
                            write!(out, "{values:?}")?;
                        }
                    };
                }
                None => write!(out, "_")?,
            };

            write!(out, ", ")?;
        }

        write!(out, " }}")
    }
}
