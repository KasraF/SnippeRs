use crate::{
    store::{ProgramStore, Store},
    utils::*,
};

mod binary;
mod unary;

pub type NodeEnumBuilder<O: Val> = &'static dyn Fn(&Store) -> dyn NodeEnum<O>;
pub trait NodeEnum<T: Val> = Iterator<Item = Box<dyn MaybeNode<T>>>;

pub trait MaybeNode<T: Val> {
    fn values<'a>(&'a self) -> &'a [T];

    /// This is a *weird* function. Basically, to convert a MaybeNode to
    /// a Node, we need to replace the *values* held by the MaybeNode
    /// with the *index* of those values in the store.
    /// So this function takes said index, and returns the Node, and the
    /// values to be placed in the Store.
    /// NOTE: This assumes that the caller will place the given values
    ///  at the *given index*.
    fn to_node(self: Box<Self>, node_index: Index<T>) -> (Box<dyn Node<T>>, Vec<T>);
}

pub trait Node<T: Val> {
    fn code(&self, store: &Store) -> String;
    fn values<'a>(&self, store: &'a Store) -> &'a [T];
}
