use crate::bool_nodes::BoolNode;
use crate::int_nodes::IntNode;
use crate::str_nodes::StrNode;
use crate::Snippers;

// Traits
pub trait Node<T> {
    fn values(&self) -> &[T];
    fn to_string(&self, s: &Snippers) -> String;
}

pub trait Enumerator<R, T: Node<R>> {
    fn next(&mut self, contexts: usize) -> Option<T>;
}

// Structs
pub struct NullaryNode<T> {
    pub values: Vec<T>,
}

pub struct UnaryNode<T> {
    pub values: Vec<T>,
    pub arg: NodeIdx,
}

pub struct BinaryNode<T> {
    pub values: Vec<T>,
    pub args: (NodeIdx, NodeIdx),
}

pub struct TernaryNode<T> {
    pub valuse: Vec<T>,
    pub args: (NodeIdx, NodeIdx, NodeIdx),
}

pub struct NodeVec {
    pub str_nodes: Vec<StrNode>,
    pub int_nodes: Vec<IntNode>,
    pub bool_nodes: Vec<BoolNode>,
}

impl Default for NodeVec {
    fn default() -> Self {
        NodeVec {
            str_nodes: Vec::new(),
            int_nodes: Vec::new(),
            bool_nodes: Vec::new(),
        }
    }
}

pub enum Type {
    Str,
    Int,
    Bool,
}

pub trait Map<K, V> {
    fn put(&mut self, height: usize, value: V) -> K;
    fn get(&self, key: &K) -> &V;
    fn has(&self, key: &K) -> bool;
}

pub type NodeIdx = (usize, usize);

pub struct NodesIter<'a, T> {
    nodes: &'a dyn Map<NodeIdx, T>,
    curr: NodeIdx,
}

impl<'a, T> NodesIter<'a, T> {
    pub fn curr(&self) -> &'a T {
        &self.nodes.get(&self.curr)
    }

    pub fn curr_idx(&self) -> &NodeIdx {
        &self.curr
    }

    pub fn next(&mut self) -> Option<&'a T> {
        self.curr.1 += 1;

        if self.nodes.has(&self.curr) {
            Some(self.nodes.get(&self.curr))
        } else {
            self.curr.0 += 1;
            self.curr.1 = 0;

            if self.nodes.has(&self.curr) {
                Some(self.nodes.get(&self.curr))
            } else {
                None
            }
        }
    }
}

pub struct Nodes {
    nodes: [NodeVec; 8],
}

impl Nodes {
    pub fn new() -> Self {
        Nodes {
            nodes: [
                NodeVec::default(),
                NodeVec::default(),
                NodeVec::default(),
                NodeVec::default(),
                NodeVec::default(),
                NodeVec::default(),
                NodeVec::default(),
                NodeVec::default(),
            ],
        }
    }

    pub fn iter<T>(&self) -> NodesIter<T>
    where
        Nodes: Map<NodeIdx, T>,
    {
        NodesIter {
            nodes: self,
            curr: (0, 0),
        }
    }
}

impl Map<NodeIdx, StrNode> for Nodes {
    fn put(&mut self, height: usize, value: StrNode) -> NodeIdx {
        let nodes = &mut self.nodes[height].str_nodes;
        nodes.push(value);
        (height, nodes.len() - 1)
    }

    fn get(&self, key: &NodeIdx) -> &StrNode {
        &self.nodes[key.0].str_nodes[key.1]
    }

    fn has(&self, key: &NodeIdx) -> bool {
        self.nodes.len() > key.0 && self.nodes[key.0].str_nodes.len() > key.1
    }
}

impl Map<NodeIdx, IntNode> for Nodes {
    fn put(&mut self, height: usize, value: IntNode) -> NodeIdx {
        let nodes = &mut self.nodes[height].int_nodes;
        nodes.push(value);
        (height, nodes.len() - 1)
    }

    fn get(&self, key: &NodeIdx) -> &IntNode {
        &self.nodes[key.0].int_nodes[key.1]
    }

    fn has(&self, key: &NodeIdx) -> bool {
        self.nodes.len() > key.0 && self.nodes[key.0].int_nodes.len() > key.1
    }
}

impl Map<NodeIdx, BoolNode> for Nodes {
    fn put(&mut self, height: usize, value: BoolNode) -> NodeIdx {
        let nodes = &mut self.nodes[height].bool_nodes;
        nodes.push(value);
        (height, nodes.len() - 1)
    }

    fn get(&self, key: &NodeIdx) -> &BoolNode {
        &self.nodes[key.0].bool_nodes[key.1]
    }

    fn has(&self, key: &NodeIdx) -> bool {
        self.nodes.len() > key.0 && self.nodes[key.0].bool_nodes.len() > key.1
    }
}
