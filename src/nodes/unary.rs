use super::*;

pub struct UnaryNode<I: Val, O: Val> {
    child: Index<I>,
    values: Index<O>,
}

impl<I: Val, O: Val> Node<O> for UnaryNode<I, O> {
    fn code(&self, store: &Store) -> String {
        todo!()
    }

    fn values<'a>(&self, store: &'a Store) -> &'a [O] {
        todo!()
    }
}
