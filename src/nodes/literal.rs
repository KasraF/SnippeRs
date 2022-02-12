use crate::nodes::Node;
use crate::store::Store;
use crate::utils::Int;

#[derive(Debug)]
pub struct IntLiteral {
    value: Int,
}

impl Node<Int> for IntLiteral {
    fn code(&self, _store: &Store) -> String {
        self.value.to_string()
    }

    fn level(&self) -> usize {
        0
    }
}

impl IntLiteral {
    pub fn new(value: Int, contexts: usize) -> (Box<dyn Node<Int>>, Vec<Int>) {
        let node = Box::new(IntLiteral { value });
        let values = vec![value; contexts];
        (node, values)
    }
}
