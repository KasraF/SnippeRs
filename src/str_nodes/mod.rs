use crate::int_nodes::IntNode;
use crate::utils::{Enumerator, Map, Node, NodeIdx, Nodes, NodesIter};
use crate::Snippers;

type NullaryNode = crate::utils::NullaryNode<String>;
type UnaryNode = crate::utils::UnaryNode<String>;
type BinaryNode = crate::utils::BinaryNode<String>;

// The node enum and its implementations

pub enum StrNode {
    Literal(NullaryNode),
    IntToStr(UnaryNode),
    Append(BinaryNode),
    Index(BinaryNode),
}

impl Node<String> for StrNode {
    fn values(&self) -> &[String] {
        match self {
            StrNode::Literal(NullaryNode { values }) => &values,
            StrNode::IntToStr(UnaryNode { values, .. }) => &values,
            StrNode::Append(BinaryNode { values, .. }) => &values,
            StrNode::Index(BinaryNode { values, .. }) => &values,
        }
    }

    fn to_string(&self, s: &Snippers) -> String {
        match self {
            StrNode::Literal(NullaryNode { values }) => values[0].clone(),
            StrNode::IntToStr(UnaryNode { arg, .. }) => {
                let arg: &IntNode = s.node_cache.get(arg);
                format!("str({})", arg.to_string(s))
            }
            StrNode::Append(BinaryNode { args, .. }) => {
                let left: &StrNode = s.node_cache.get(&args.0);
                let right: &StrNode = s.node_cache.get(&args.1);
                format!("{} + {}", left.to_string(s), right.to_string(s))
            }
            StrNode::Index(BinaryNode { args, .. }) => {
                let left: &StrNode = s.node_cache.get(&args.0);
                let right: &IntNode = s.node_cache.get(&args.1);
                format!("{} + {}", left.to_string(s), right.to_string(s))
            }
        }
    }
}

// Enumerator implementations for each node
struct StringLiteralEnumerator {
    s: String,
    done: bool,
}

impl Enumerator<String, StrNode> for StringLiteralEnumerator {
    fn next(&mut self, contexts: usize) -> Option<StrNode> {
        if self.done {
            None
        } else {
            Some(StrNode::Literal(NullaryNode {
                values: vec![self.s.clone(); contexts],
            }))
        }
    }
}

struct StringAppendEnumerator<'a> {
    nodes: &'a Nodes,
    left: NodesIter<'a, StrNode>,
    right: NodesIter<'a, StrNode>,
}

impl<'a> Enumerator<String, StrNode> for StringAppendEnumerator<'a> {
    fn next(&mut self, _contexts: usize) -> Option<StrNode> {
        let right = match self.right.next() {
            Some(right) => Some(right),
            None => {
                if self.left.next().is_some() {
                    self.right = self.nodes.iter();
                    Some(self.right.curr())
                } else {
                    None
                }
            }
        };

        match right {
            Some(right) => {
                // Build the new values
                let left = self.left.curr().values();
                let right = right.values();
                let mut values = Vec::with_capacity(left.len());

                for i in 0..left.len() {
                    let value = left[i].clone() + &right[i];
                    values.push(value);
                }

                let args = (self.left.curr_idx().clone(), self.right.curr_idx().clone());

                Some(StrNode::Append(BinaryNode { args, values }))
            }
            None => None,
        }
    }
}
