use crate::*;
use std::{fmt::Debug, hash::Hash, marker::PhantomData};

pub type Int = i32;
pub type Str = String;
// type Bool = bool;

pub type BinValueFn<L, R, T> = fn((&L, &R)) -> T;
pub type BinCodeFn = fn(&str, &str) -> String;
pub type BinValidatorFn<L, R> = fn(&[L], &[R]) -> bool;

pub type Builder<T> = Box<dyn Fn(usize) -> Box<dyn Enumerator<T>>>;

pub trait Value: Debug + Hash + 'static {}
impl Value for Int {}
impl Value for Str {}

pub const MAX_SIZE: usize = 4;

#[derive(Debug)]
pub struct Index<T: Value> {
    pub size: usize,
    pub idx: usize,
    _phantom: PhantomData<T>,
}

impl<T: Value> Clone for Index<T> {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            idx: self.idx,
            _phantom: PhantomData,
        }
    }
}

impl<T: Value> Copy for Index<T> {}

impl<T: Value> Index<T> {
    pub fn new(size: usize, idx: usize) -> Self {
        Self {
            size,
            idx,
            _phantom: PhantomData,
        }
    }
}

pub trait GenericPred<T> {
    fn matches(&self, program: &dyn Program<T>) -> bool;
}

pub trait Predicate: GenericPred<Int> + GenericPred<Str> {}
