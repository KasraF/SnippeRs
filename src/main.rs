use std::{slice::SliceIndex, vec::Drain};

type Error = Box<dyn std::error::Error>;
type Int = i32;

struct IntNodeIdx {
    level: usize,
    idx: usize,
}

struct IntIdx {
    level: usize,
    idx: usize,
}

trait ValueStore<'a, I, T> {
    fn peek(&'a self, idx: &I) -> &'a [T];
    fn push_values(&mut self, values: &[T]) -> I;
    fn pop(&mut self) -> Drain<T>;
}

trait ProgramStore<'a, K, V: ?Sized> {
    fn get(&'a self, idx: &K) -> &Box<V>;
    fn push_node(&'a mut self, value: Box<V>) -> K;
}

struct LevelStore {
    int_programs: Vec<Box<dyn Node<Int>>>,
    int_values: Vec<Int>,
}

impl LevelStore {
    fn new() -> Self {
        Self {
            int_programs: Vec::with_capacity(1024),
            int_values: Vec::with_capacity(1024),
        }
    }
}

struct Store {
    contexts: usize,
    levels: Vec<LevelStore>,
}

impl Store {
    fn new(contexts: usize) -> Self {
        Self {
            contexts,
            levels: vec![LevelStore::new()],
        }
    }
}

impl<'a> ValueStore<'a, IntIdx, Int> for Store {
    fn peek(&'a self, idx: &IntIdx) -> &'a [Int] {
        &self.int_values[idx.0..idx.0 + self.contexts]
    }

    fn push_values(&mut self, values: &[Int]) -> IntIdx {
        assert!(values.len() == self.contexts);

        let idx = self.int_values.len();
        for v in values {
            self.int_values.push(v.clone());
        }

        idx.into()
    }

    fn pop(&mut self) -> Drain<Int> {
        self.int_values
            .drain(self.int_values.len() - self.contexts..self.int_values.len())
    }
}

impl<'a> ProgramStore<'a, IntNodeIdx, dyn Node<Int>> for Store {
    fn get(&'a self, idx: &IntNodeIdx) -> &Box<dyn Node<Int>> {
        &self.int_programs[idx.0]
    }

    fn push_node(&'a mut self, value: Box<dyn Node<Int>>) -> IntNodeIdx {
        let idx = self.int_programs.len();
        self.int_programs.push(value);
        idx.into()
    }
}

trait Node<T> {
    fn values<'a>(&self, store: &'a Store) -> &'a [T];
    fn code(&self, store: &Store) -> String;
}

trait LiteralNode<T>: Node<T> {
    fn new(value: T, store: &mut Store) -> Self;
}

trait BinaryNode<T>: Node<T> {
    fn new(lhs_idx: usize, rhs_idx: usize, store: &mut Store) -> Self;
}

struct IntLiteral {
    value: Int,
    idx: IntIdx,
}

impl Node<Int> for IntLiteral {
    fn values<'a>(&self, store: &'a Store) -> &'a [Int] {
        store.peek(&self.idx)
    }

    fn code(&self, _: &Store) -> String {
        self.value.to_string()
    }
}

impl LiteralNode<Int> for IntLiteral {
    fn new(value: Int, store: &mut Store) -> Self {
        // Insert the value into the store |context| times
        let values = vec![value; store.contexts];
        let idx = store.push_values(&values);
        Self { value, idx }
    }
}

struct IntAddition {
    lhs: IntNodeIdx,
    rhs: IntNodeIdx,
    idx: IntIdx,
}

impl Node<Int> for IntAddition {
    fn values<'a>(&self, store: &'a Store) -> &'a [Int] {
        store.peek(&self.idx)
    }

    fn code(&self, store: &Store) -> String {
        format!(
            "{} + {}",
            store.get(&self.lhs).code(store),
            store.get(&self.rhs).code(store)
        )
    }
}

impl BinaryNode<Int> for IntAddition {
    fn new(lhs_idx: usize, rhs_idx: usize, store: &mut Store) -> Self {
        let lhs_idx = IntNodeIdx(lhs_idx);
        let rhs_idx = IntNodeIdx(rhs_idx);
        let lsh = store.get(&lhs_idx);
        let rhs = store.get(&rhs_idx);
        let values: Vec<i32> = lsh
            .values(store)
            .iter()
            .zip(rhs.values(store))
            .map(|(l, r)| l + r)
            .collect();
        let idx: IntIdx = store.push_values(&values);

        IntAddition {
            lhs: lhs_idx,
            rhs: rhs_idx,
            idx,
        }
    }
}

trait NodeBuilder<T> {
    // TODO Use Iter trait
    fn next(&mut self) -> Option<Box<dyn Node<T>>>;
}

struct IntAdditionBuilder<'a> {
    store: &'a mut Store,
    curr_lhs: usize,
    curr_rhs: usize,
}

impl<'a> IntAdditionBuilder<'a> {
    fn new(store: &'a mut Store) -> Self {
        Self {
            store,
            curr_lhs: 0,
            curr_rhs: 0,
        }
    }
}

impl<'a> NodeBuilder<Int> for IntAdditionBuilder<'a> {
    fn next(&'a mut self) -> Option<Box<dyn Node<Int>>> {
        let int_values = self.store.int_values.len();
        if int_values <= self.curr_rhs {
            self.curr_lhs += 1;

            if int_values <= self.curr_lhs {
                return None;
            }

            self.curr_rhs = 0;
        }

        let rs = IntAddition::new(self.curr_lhs, self.curr_rhs, self.store);
        self.curr_rhs += 1;
        Some(Box::new(rs))
    }
}

struct Vocab {
    int: Vec<Box<dyn NodeBuilder<Int>>>,
}

struct Synthesizer {
    store: Store,
    vocab: Vocab,
}

impl Synthesizer {
    fn new() -> Self {
        let store = Store::new(1);

        // Add literals

        builders = vec![]
    }
}

fn main() -> Result<(), Error> {
    println!("Hello, synthesis!");
    Ok(())
}
