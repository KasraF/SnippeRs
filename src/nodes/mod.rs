use crate::store::Store;
use std::fmt::Debug;
use std::hash::Hash;

pub mod binary;
pub mod literal;

pub trait Node<T: Eq + Hash>: Debug {
    fn code(&self, store: &Store) -> String;
    fn level(&self) -> usize;
}
