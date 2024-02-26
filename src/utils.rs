use std::marker::PhantomData;

use crate::ops::Program;

pub type Int = i32;
pub type Str = String;
pub type IntArray = Vec<Int>;
pub type StrArray = Vec<Str>;

pub trait Value: Clone + Eq + 'static {}
impl Value for Int {}
impl Value for Str {}
impl Value for IntArray {}
impl Value for StrArray {}

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnyVal {
    Int(VIdx<Int>),
    Str(VIdx<Str>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnyProg {
    Int(PIdx<Int>),
    Str(PIdx<Str>),
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
