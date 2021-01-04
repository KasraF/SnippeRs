use crate::utils::{Node, NullaryNode};
use crate::Snippers;

pub enum IntNode {
	Literal(NullaryNode<i32>),
}

impl Node<i32> for IntNode {
	fn values(&self) -> &[i32] {
		match self {
			IntNode::Literal(NullaryNode { values }) => {
				&values
			}
		}
	}

	fn to_string(&self, s: &Snippers) -> String {
		match self {
			IntNode::Literal(NullaryNode { values }) => {
				format!("{}", values[0])
			},
		}
	}
}
