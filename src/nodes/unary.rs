use super::*;

pub struct UnaryNode<I: Val, O: Val> {
    child: Index<I>,
    values: Index<O>,
}

impl<I: Val, O: Val> Node<O> for UnaryNode<I, O> {}
