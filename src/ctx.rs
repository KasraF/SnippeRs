use std::{fmt::Display, marker::PhantomData, ops::Deref};

use smallvec::SmallVec;

use crate::utils::*;

// TODO It might be more efficient to flatten
// this vec into its own data structure.
pub type Contexts = Vec<Context>;

pub trait VarMap<T: Val> {
    fn insert(&mut self, var: &str) -> Var<T>;
    fn lookup(&self, var: &str) -> Option<Var<T>>;
    fn get(&self, var: Var<T>) -> &str;
    fn names(&self) -> &[String];
    fn iter(&self) -> VarIter<T> {
        let len = self.names().len();
        VarIter::new(len)
    }
}

pub struct VarIter<T: Val> {
    curr: usize,
    len: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Val> VarIter<T> {
    fn new(len: usize) -> Self {
        Self {
            curr: 0,
            len,
            _phantom_data: PhantomData,
        }
    }
}

impl<T: Val> Iterator for VarIter<T> {
    type Item = Var<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.len {
            let rs = Var::new(self.curr);
            self.curr += 1;
            Some(rs)
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
pub struct VariableMap {
    ints: SmallVec<[String; 8]>,
    int_arrs: SmallVec<[String; 8]>,
}

impl VarMap<Int> for VariableMap {
    fn insert(&mut self, var: &str) -> Var<Int> {
        // TODO Do we need to be better about this?
        for (i, v) in self.ints.iter().enumerate() {
            if v == var {
                return Var::new(i);
            }
        }

        let idx = Var::new(self.ints.len());
        self.ints.push(var.to_string());
        idx
    }

    fn get(&self, var: Var<Int>) -> &str {
        &self.ints[*var]
    }

    fn lookup(&self, var: &str) -> Option<Var<Int>> {
        for (i, v) in self.ints.iter().enumerate() {
            if v == var {
                return Some(Var::new(i));
            }
        }
        None
    }

    fn names(&self) -> &[String] {
        &self.ints
    }
}

impl VarMap<IntArray> for VariableMap {
    fn insert(&mut self, var: &str) -> Var<IntArray> {
        let idx = Var::new(self.int_arrs.len());
        self.int_arrs.push(var.to_string());
        idx
    }

    fn get(&self, var: Var<IntArray>) -> &str {
        &self.int_arrs[*var]
    }

    fn lookup(&self, var: &str) -> Option<Var<IntArray>> {
        for (i, v) in self.int_arrs.iter().enumerate() {
            if v == var {
                return Some(Var::new(i));
            }
        }
        None
    }

    fn names(&self) -> &[String] {
        &self.int_arrs
    }
}

pub trait Ctxs<T: Val> {
    fn has(&self, idx: Var<T>) -> bool;
    fn try_get(&self, idx: Var<T>) -> Option<Vec<T>> {
        if self.has(idx) {
            Some(self.get(idx))
        } else {
            None
        }
    }
    fn get(&self, idx: Var<T>) -> Vec<T>;
    fn set(&self, idx: Var<T>, val: &[T]) -> Self;
}

impl<T: Val> Ctxs<T> for Contexts
where
    Context: Ctx<T>,
{
    fn has(&self, idx: Var<T>) -> bool {
        self.first().map_or(false, |f| f.has(idx))
    }

    fn get(&self, idx: Var<T>) -> Vec<T> {
        self.iter().map(|ctx| ctx.get(idx).clone()).collect()
    }

    fn set(&self, idx: Var<T>, val: &[T]) -> Self {
        debug_assert!(
            val.len() == self.len(),
            "Invalid array length. Required {}, got {}.",
            self.len(),
            val.len()
        );
        self.iter()
            .zip(val)
            .map(|(ctx, val)| ctx.set(idx, val.clone()))
            .collect()
    }
}

pub trait Ctx<T: Val> {
    fn has(&self, idx: Var<T>) -> bool;
    fn try_get(&self, idx: Var<T>) -> Option<&T>;
    fn get(&self, idx: Var<T>) -> &T {
        self.try_get(idx)
            .expect("Ctx::get called, but variable does not exist")
    }
    fn set(&self, idx: Var<T>, val: T) -> Self;
}

#[derive(Clone, Default, Debug)]
pub struct Context {
    ints: SmallVec<[Int; 4]>,
    int_arrs: SmallVec<[IntArray; 4]>,
}

impl Ctx<Int> for Context {
    fn try_get(&self, idx: Var<Int>) -> Option<&Int> {
        if self.ints.len() < *idx {
            Some(&self.ints[*idx])
        } else {
            None
        }
    }

    fn set(&self, idx: Var<Int>, val: Int) -> Self {
        debug_assert!(
            self.has(idx),
            "Tried to set variable {} for context, but it doesn't exist.",
            idx
        );
        let mut rs = self.clone();
        rs.ints[*idx] = val;
        rs
    }

    fn has(&self, idx: Var<Int>) -> bool {
        self.ints.len() < *idx
    }
}

impl Ctx<IntArray> for Context {
    fn try_get(&self, idx: Var<IntArray>) -> Option<&IntArray> {
        if self.int_arrs.len() < *idx {
            Some(&self.int_arrs[*idx])
        } else {
            None
        }
    }

    fn set(&self, idx: Var<IntArray>, val: IntArray) -> Self {
        debug_assert!(
            self.has(idx),
            "Tried to set variable {} for context, but it doesn't exist.",
            idx
        );
        let mut rs = self.clone();
        rs.int_arrs[*idx] = val;
        rs
    }

    fn has(&self, idx: Var<IntArray>) -> bool {
        self.int_arrs.len() < *idx
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Var<T: Val> {
    idx: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Val> Var<T> {
    pub fn new(idx: usize) -> Self {
        Self {
            idx,
            _phantom_data: PhantomData,
        }
    }
}

impl<T: Val> Deref for Var<T> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.idx
    }
}

impl<T: Val> Display for Var<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Var {{{}}}", self.idx)
    }
}

impl<T: Val> Copy for Var<T> {}
