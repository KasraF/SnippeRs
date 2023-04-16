use serde::Deserialize;
use smallvec::SmallVec;
use std::{fmt::Display, marker::PhantomData};

pub type Int = i32;
pub type Str = String;
pub type Bool = bool;
pub type IntArray = SmallVec<[Int; 8]>;
pub type StrArray = SmallVec<[Str; 8]>;
pub type BoolArray = SmallVec<[Bool; 8]>;

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
    pub idx: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Val> Index<T> {
    pub fn new(start: usize) -> Self {
        Self {
            idx: start,
            _phantom_data: PhantomData,
        }
    }
}

impl<T: Val> Display for Index<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index {{{}}}", self.idx)
    }
}

impl<T: Val> Copy for Index<T> {}
