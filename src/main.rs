use std::vec::Drain;

type Error = Box<dyn std::error::Error>;
type Int = i32;

#[derive(Default, Copy, Clone)]
struct IntNodeIdx {
    level: usize,
    idx: usize,
}

#[derive(Default, Copy, Clone)]
struct IntIdx {
    level: usize,
    idx: usize,
}

trait ValueStore<'a, I, T> {
    fn peek(&'a self, idx: &I) -> &'a [T];
    fn push_values(&mut self, level: usize, values: &[T]) -> I;
    fn pop(&mut self, level: usize) -> Drain<T>;
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
        let level_idx = idx.level;
        let level_store = &self.levels[level_idx];
        &level_store.int_values[idx.idx..idx.idx + self.contexts]
    }

    fn push_values(&mut self, level_idx: usize, values: &[Int]) -> IntIdx {
        assert!(values.len() == self.contexts);

        while level_idx >= self.levels.len() {
            self.levels.push(LevelStore::new());
        }

        let level = &mut self.levels[level_idx];

        let idx = level.int_values.len();
        for v in values {
            level.int_values.push(v.clone());
        }

        IntIdx {
            level: level_idx,
            idx: idx.into(),
        }
    }

    fn pop(&mut self, level: usize) -> Drain<Int> {
        let level = &mut self.levels[level];
        level
            .int_values
            .drain(level.int_values.len() - self.contexts..level.int_values.len())
    }
}

impl<'a> ProgramStore<'a, IntNodeIdx, dyn Node<Int>> for Store {
    fn get(&'a self, idx: &IntNodeIdx) -> &Box<dyn Node<Int>> {
        &self.levels[idx.level].int_programs[idx.idx]
    }

    fn push_node(&'a mut self, node: Box<dyn Node<Int>>) -> IntNodeIdx {
        let level_idx = node.level();
        let level = &mut self.levels[level_idx];
        let idx = level.int_programs.len();
        level.int_programs.push(node);
        IntNodeIdx {
            level: level_idx,
            idx,
        }
    }
}

trait Node<T> {
    fn values<'a>(&self, store: &'a Store) -> &'a [T];
    fn code(&self, store: &Store) -> String;
    fn level(&self) -> usize;
}

trait LiteralNode<T>: Node<T> {
    fn new(value: T, store: &mut Store) -> Self;
}

trait BinaryNode<'a, T, I>: Node<T>
where
    Store: ProgramStore<'a, I, dyn Node<T>>,
{
    fn new(lhs_idx: I, rhs_idx: I, store: &mut Store) -> Self;
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

    fn level(&self) -> usize {
        0
    }
}

impl LiteralNode<Int> for IntLiteral {
    fn new(value: Int, store: &mut Store) -> Self {
        // Insert the value into the store |context| times
        let values = vec![value; store.contexts];
        let idx = store.push_values(0, &values);
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

    fn level(&self) -> usize {
        self.lhs.level + self.rhs.level + 1
    }
}

impl BinaryNode<'_, Int, IntNodeIdx> for IntAddition {
    fn new(lhs_idx: IntNodeIdx, rhs_idx: IntNodeIdx, store: &mut Store) -> Self {
        // Level is size!
        let level = lhs_idx.level + rhs_idx.level + 1;
        let lsh = store.get(&lhs_idx);
        let rhs = store.get(&rhs_idx);
        let values: Vec<i32> = lsh
            .values(store)
            .iter()
            .zip(rhs.values(store))
            .map(|(l, r)| l + r)
            .collect();
        let idx: IntIdx = store.push_values(level, &values);

        IntAddition {
            lhs: lhs_idx,
            rhs: rhs_idx,
            idx,
        }
    }
}

trait NodeBuilder<T> {
    // TODO Use Iter trait
    fn next(&mut self, store: &mut Store) -> Option<Box<dyn Node<T>>>;
    fn set_level(&mut self, level: usize);
}

struct IntAdditionBuilder {
    level: usize,
    curr_lhs: IntNodeIdx,
    curr_rhs: IntNodeIdx,
}

impl IntAdditionBuilder {
    fn new() -> Self {
        Self {
            level: 1,
            curr_lhs: IntNodeIdx::default(),
            curr_rhs: IntNodeIdx::default(),
        }
    }
}

fn next_indices(
    lhs: &mut IntNodeIdx,
    rhs: &mut IntNodeIdx,
    level: usize,
    store: &Store) -> bool {

    // First, step!
    let rhs_nodes = store.levels[rhs.level].int_programs.len();
    if rhs.idx >= rhs_nodes {
        // Move lhs
        let lhs_nodes = store.levels[lhs.level].int_programs.len();

        if lhs.idx >= lhs_nodes {
            if lhs.level + 1 == level {
                return false;
            }
            // Move levels!
            lhs.level += 1;
            rhs.level = level - lhs.level - 1;
        } else {
            lhs.idx += 1;
            rhs.idx = 0;
        }
    } else {
        // Move rhs
        rhs.idx += 1;
    }

    // Now see if we succeeded
    if lhs.level < store.levels.len() &&
        rhs.level < store.levels.len() &&
        lhs.idx < store.levels[lhs.level].int_values.len() &&
        rhs.idx < store.levels[rhs.level].int_values.len() {
        true
    } else if lhs.level >= store.levels.len() {
        // We're out of levels!
        false
    } else {
        next_indices(lhs, rhs, level, store)
    }
}

impl NodeBuilder<Int> for IntAdditionBuilder {
    fn next(&mut self, store: &mut Store) -> Option<Box<dyn Node<Int>>> {
        if next_indices(&mut self.curr_lhs, &mut self.curr_rhs, self.level, store) {
            let rs = IntAddition::new(self.curr_lhs, self.curr_rhs, store);
            Some(Box::new(rs))
        } else {
            None
        }
    }

    fn set_level(&mut self, level: usize) {
        self.level = level;
    }
}

struct Vocab {
    int: Vec<Box<dyn NodeBuilder<Int>>>,
}

struct Synthesizer {
    store: Store,
    vocab: Vocab,
    curr_vocab: usize,
}

impl Synthesizer {
    fn new() -> Self {
        let mut store = Store::new(1);

        // Add literals
        for i in -1i32..2 {
            let node = Box::new(IntLiteral::new(i, &mut store));
            store.push_node(node);
        }

        let vocab = Vocab {
            int: vec![Box::new(IntAdditionBuilder::new())],
        };

        Self {
            store,
            vocab,
            curr_vocab: 0,
        }
    }

    fn next(&mut self) -> Box<dyn Node<Int>> {
        self.vocab.int[self.curr_vocab]
            .next(&mut self.store)
            .unwrap()
    }
}

fn main() -> Result<(), Error> {
    println!("Hello, synthesis!");
    let mut synth = Synthesizer::new();
    for _ in 0..100 {
        println!("{}", synth.next().code(&synth.store));
    }
    Ok(())
}
