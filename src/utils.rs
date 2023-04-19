use serde::Deserialize;
use smallvec::SmallVec;
use std::{fmt::Display, marker::PhantomData, ops::Deref};

pub type Int = i32;
pub type Str = String;
pub type Bool = bool;
pub type IntArray = SmallVec<[Int; 8]>;
pub type StrArray = SmallVec<[Str; 8]>;
pub type BoolArray = SmallVec<[Bool; 8]>;

#[derive(Debug, PartialEq, Eq)]
pub enum Typ {
    Int,
    Str,
    Bool,
    IntArray,
    StrArray,
    BoolArray,
}

pub enum Typed<T> {
    Int(T),
    Str(T),
    Bool(T),
    IntArray(T),
    StrArray(T),
    BoolArray(T),
}

impl<T> Typed<T> {
    pub fn inner(&self) -> &T {
        match self {
            Typed::Int(v) => v,
            Typed::Str(v) => v,
            Typed::Bool(v) => v,
            Typed::IntArray(v) => v,
            Typed::StrArray(v) => v,
            Typed::BoolArray(v) => v,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Value {
    Int(Int),
    Str(Str),
    Bool(Bool),
    IntArray(IntArray),
    StrArray(StrArray),
    BoolArray(BoolArray),
}

pub trait Val: Eq + std::hash::Hash + Clone + 'static {}
impl Val for Int {}
impl Val for Str {}
impl Val for Bool {}
impl Val for IntArray {}
impl Val for StrArray {}
impl Val for BoolArray {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Index<T: Val> {
    idx: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Val> Index<T> {
    pub fn new(idx: usize) -> Self {
        Self {
            idx,
            _phantom_data: PhantomData,
        }
    }
}

impl<T: Val> Deref for Index<T> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.idx
    }
}

impl<T: Val> Display for Index<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index {{{}}}", self.idx)
    }
}

impl<T: Val> Copy for Index<T> {}
