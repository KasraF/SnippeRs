use crate::{
    store::{ProgramStore, Store},
    utils::*,
};

mod binary;
mod unary;

pub type NodeEnumBuilder<O: Val> = &'static dyn Fn(&Store) -> dyn NodeEnum<O>;

pub trait NodeEnum<O: Val> {
    fn next(&mut self) -> Box<dyn MaybeNode<O>>;
}

impl<O: Val> Iterator for dyn NodeEnum<O> {
    type Item = Box<dyn MaybeNode<O>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next())
    }
}

trait MaybeNode<O: Val> {
    fn values<'a>(&self, store: &'a Store) -> &'a [O];
    fn to_node(self, store: &mut Store) -> Box<dyn Node<O>>;
}

pub struct UnaryNode<I: Val, O: Val> {
    child: Index<I>,
    values: Index<O>,
}

pub trait Node<T: Val> {
    fn code(&self, store: &Store) -> String;
    fn values<'a>(&self, store: &'a Store) -> &'a [T];
}
