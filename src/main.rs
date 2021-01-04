// TODO Remove me!
#![allow(dead_code)]

use crate::utils::{ Node, Enumerator, NullaryNode, Nodes };
use crate::str_nodes::StrNode;
use crate::int_nodes::IntNode;
use crate::bool_nodes::BoolNode;

type Error = Box<dyn std::error::Error>;

mod utils;
mod int_nodes;
mod str_nodes;
mod bool_nodes;

pub struct Snippers {
	thread_count: usize,
	curr_height: usize,
	node_cache: Nodes,
	vocab: (
		Vec<Box<dyn Enumerator<String, StrNode>>>,
		Vec<Box<dyn Enumerator<i32, IntNode>>>,
		Vec<Box<dyn Enumerator<bool, BoolNode>>>)
}


impl Snippers {
	pub fn new(thread_count: usize) -> Self {
		// TODO Initialize the node caches by pre-allocating
		// the memory the vecs at each height need.

		Snippers {
			thread_count,
			curr_height: 0,
			node_cache: Nodes::new(),
			vocab: (
				vec![],
				vec![],
				vec![],
			)
		}
	}

	pub fn next_int(&mut self) -> impl Node<i32> {
		IntNode::Literal(NullaryNode {
			values: vec![0],
		})
	}

	pub fn next_str(&mut self) -> impl Node<String> {
		StrNode::Literal(NullaryNode {
			values: vec!["".to_string()],
		})
	}
}



fn main() -> Result<(), Error> {
	println!("Hello, world!");

	let mut snippers = Snippers::new(4);
	let mut start = std::time::Instant::now();

	for _ in 0..1000 {
		println!("{}", snippers.next_int().to_string(&snippers));
	}

	println!("Total time: {}", (std::time::Instant::now() - start).as_secs());

	Ok(())
}
