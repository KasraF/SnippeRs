use std::marker::PhantomData;

use crate::{ops::Program, store::Bank, Level, Pointer, PostCondition, PreCondition};

pub type Int = i32;
pub type Str = String;
pub type Array<T> = Vec<T>;
pub type IntArray = Array<Int>;
pub type StrArray = Array<Str>;

pub trait Value: Clone + Eq + std::fmt::Debug + 'static {}
impl Value for Int {}
impl Value for Str {}
impl Value for IntArray {}
impl Value for StrArray {}

pub type Error = Box<dyn std::error::Error>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VIdx<T: Value> {
    i: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Value> std::ops::Add<usize> for VIdx<T> {
    type Output = usize;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self.i + rhs
    }
}

impl<T: Value> Copy for VIdx<T> {}

impl<T: Value> From<usize> for VIdx<T> {
    #[inline]
    fn from(value: usize) -> Self {
        VIdx {
            i: value,
            _phantom_data: PhantomData,
        }
    }
}

impl<T: Value> From<VIdx<T>> for usize {
    #[inline]
    fn from(value: VIdx<T>) -> Self {
        value.i
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PIdx<T: Value> {
    i: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Value> Copy for PIdx<T> {}

impl<T: Value> From<usize> for PIdx<T> {
    #[inline]
    fn from(value: usize) -> Self {
        PIdx {
            i: value,
            _phantom_data: PhantomData,
        }
    }
}

impl<T: Value> From<PIdx<T>> for usize {
    #[inline]
    fn from(value: PIdx<T>) -> Self {
        value.i
    }
}

impl<T: Value> std::ops::Index<PIdx<T>> for Vec<Box<dyn Program<T>>> {
    type Output = Box<dyn Program<T>>;

    #[inline]
    fn index(&self, index: PIdx<T>) -> &Self::Output {
        self.index(index.i)
    }
}

impl<T: Value> std::ops::AddAssign<usize> for PIdx<T> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.i += rhs;
    }
}

impl<T: Value> std::ops::Add<usize> for PIdx<T> {
    type Output = PIdx<T>;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        (self.i + rhs).into()
    }
}

impl<T: Value> std::ops::Sub<usize> for PIdx<T> {
    type Output = PIdx<T>;

    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        (self.i - rhs).into()
    }
}

#[derive(Clone)]
pub enum Any {
    Int(Int),
    Str(Str),
}

pub enum Anies {
    Int(Vec<Int>),
    Str(Vec<Str>),
    IntArray(Vec<IntArray>),
}

impl From<Vec<Int>> for Anies {
    fn from(value: Vec<Int>) -> Self {
        Anies::Int(value)
    }
}

impl From<Vec<Str>> for Anies {
    fn from(value: Vec<Str>) -> Self {
        Anies::Str(value)
    }
}

impl From<Vec<IntArray>> for Anies {
    fn from(value: Vec<IntArray>) -> Self {
        Anies::IntArray(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnyVal {
    Int(VIdx<Int>),
    Str(VIdx<Str>),
    IntArray(VIdx<IntArray>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnyProg {
    Int(PIdx<Int>),
    Str(PIdx<Str>),
    IntArray(PIdx<IntArray>),
}

impl AnyProg {
    pub fn code(&self, store: &Bank) -> String {
        match self {
            AnyProg::Int(prog) => store[*prog].code(store),
            AnyProg::Str(prog) => store[*prog].code(store),
            AnyProg::IntArray(prog) => store[*prog].code(store),
        }
    }

    pub fn conditions<'s>(&self, store: &'s Bank) -> (&'s PreCondition, &'s PostCondition) {
        match self {
            AnyProg::Int(prog) => store[*prog].conditions(),
            AnyProg::Str(prog) => store[*prog].conditions(),
            AnyProg::IntArray(prog) => store[*prog].conditions(),
        }
    }

    pub fn pointer(&self, store: &Bank) -> Option<Pointer> {
        match self {
            AnyProg::Int(prog) => store[*prog].pointer(),
            AnyProg::Str(prog) => store[*prog].pointer(),
            AnyProg::IntArray(prog) => store[*prog].pointer(),
        }
    }

    pub fn level(&self, store: &Bank) -> Level {
        match self {
            AnyProg::Int(prog) => store[*prog].level(),
            AnyProg::Str(prog) => store[*prog].level(),
            AnyProg::IntArray(prog) => store[*prog].level(),
        }
    }
}

impl From<PIdx<Int>> for AnyProg {
    fn from(value: PIdx<Int>) -> Self {
        Self::Int(value)
    }
}

impl From<PIdx<Str>> for AnyProg {
    fn from(value: PIdx<Str>) -> Self {
        Self::Str(value)
    }
}

impl From<PIdx<IntArray>> for AnyProg {
    fn from(value: PIdx<IntArray>) -> Self {
        Self::IntArray(value)
    }
}
