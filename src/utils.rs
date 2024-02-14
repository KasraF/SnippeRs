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

#[derive(Clone)]
pub struct PIdx<T: Value> {
    i: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Value> Copy for PIdx<T> {}

impl<T: Value> From<usize> for PIdx<T> {
    fn from(value: usize) -> Self {
        PIdx {
            i: value,
            _phantom_data: PhantomData,
        }
    }
}

impl<T: Value> From<PIdx<T>> for usize {
    fn from(value: PIdx<T>) -> Self {
        value.i
    }
}

impl<T: Value> std::ops::Index<PIdx<T>> for Vec<Box<dyn Program<T>>> {
    type Output = Box<dyn Program<T>>;

    fn index(&self, index: PIdx<T>) -> &Self::Output {
        self.index(index.i)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnyVal {
    Int(VIdx<Int>),
    Str(VIdx<Str>),
}
