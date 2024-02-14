#![allow(dead_code)]

use smallvec::{smallvec, SmallVec};
use std::marker::PhantomData;
use std::{collections::HashMap, ops::Index};

type Int = i32;
type Str = String;
type IntArray = Vec<Int>;
type StrArray = Vec<Str>;

trait Value: Clone + Eq + 'static {}
impl Value for Int {}
impl Value for Str {}
impl Value for IntArray {}
impl Value for StrArray {}

#[derive(Clone, PartialEq, Eq)]
struct VIdx<T: Value> {
    i: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Value> Copy for VIdx<T> {}

impl<T: Value> From<usize> for VIdx<T> {
    fn from(value: usize) -> Self {
        VIdx {
            i: value,
            _phantom_data: PhantomData,
        }
    }
}

#[derive(Clone)]
struct PIdx<T: Value> {
    i: usize,
    _phantom_data: PhantomData<T>,
}

impl<T: Value> Copy for PIdx<T> {}

impl<T: Value> From<usize> for PIdx<T> {
    fn from(value: usize) -> Self {
        PIdx {
            i: value,
            _phantom_data: PhantomData,
        }
    }
}

trait Store<T: Value> {
    fn get_values(&self, idx: VIdx<T>) -> &[T];
    fn put_values(&mut self, values: &[T]) -> VIdx<T>;
    fn get_program(&self, idx: PIdx<T>) -> &Box<dyn Program<T>>;
    fn put_program(&mut self, program: Box<dyn Program<T>>) -> PIdx<T>;
}

impl<T> Index<VIdx<T>> for Bank
where
    T: Value,
    Bank: Store<T>,
{
    type Output = [T];

    fn index(&self, index: VIdx<T>) -> &Self::Output {
        self.get_values(index)
    }
}

impl<T> Index<PIdx<T>> for Bank
where
    T: Value,
    Bank: Store<T>,
{
    type Output = Box<dyn Program<T>>;

    fn index(&self, index: PIdx<T>) -> &Self::Output {
        self.get_program(index)
    }
}

struct Bank {
    examples: usize,
    int_vals: Vec<Int>,
    ints: Vec<Box<dyn Program<Int>>>,
    str_vals: Vec<Str>,
    strs: Vec<Box<dyn Program<Str>>>,
}

impl Bank {
    fn new(examples: usize) -> Self {
        // TODO allocate larger chunks here?
        Self {
            examples,
            int_vals: Vec::new(),
            ints: Vec::new(),
            str_vals: Vec::new(),
            strs: Vec::new(),
        }
    }
}

impl Store<Int> for Bank {
    fn get_values(&self, idx: VIdx<Int>) -> &[Int] {
        &self.int_vals[idx.i..idx.i + self.examples]
    }

    fn put_values(&mut self, values: &[Int]) -> VIdx<Int> {
        let start = self.int_vals.len();
        self.int_vals.extend(values);
        start.into()
    }

    fn get_program(&self, idx: PIdx<Int>) -> &Box<dyn Program<Int>> {
        &self.ints[idx.i]
    }

    fn put_program(&mut self, program: Box<dyn Program<Int>>) -> PIdx<Int> {
        self.ints.push(program);
        PIdx::from(self.ints.len() - 1)
    }
}

impl Store<Str> for Bank {
    fn get_values(&self, idx: VIdx<Str>) -> &[Str] {
        &self.str_vals[idx.i..idx.i + self.examples]
    }

    fn put_values(&mut self, values: &[Str]) -> VIdx<Str> {
        let start = self.str_vals.len();
        self.str_vals.extend_from_slice(values);
        start.into()
    }

    fn get_program(&self, idx: PIdx<Str>) -> &Box<dyn Program<Str>> {
        &self.strs[idx.i]
    }

    fn put_program(&mut self, program: Box<dyn Program<Str>>) -> PIdx<Str> {
        self.strs.push(program);
        PIdx::from(self.strs.len() - 1)
    }
}

trait Program<T> {
    fn code(&self, store: &Bank) -> String;
    fn values<'s>(&self, store: &'s Bank) -> &'s [T];
    fn conditions(&self) -> (&PreCondition, &PostCondition);
}

struct Variable<T: Value> {
    name: String,
    values: VIdx<T>,
    cond: Condition,
}

impl Variable<Str> {
    fn new(name: String, values: VIdx<Str>, task: &SynthesisTask) -> Self {
        let cond = Condition::empty(task.var_map.len()).mutate(
            *task.var_map.get(&name).expect(&format!(
                "Variable initalized, but name doesn't exist in var map: {name}"
            )),
            Some(AnyVal::Str(values)),
        );
        Self { name, values, cond }
    }
}

impl Variable<Int> {
    fn new(name: String, values: VIdx<Int>, task: &SynthesisTask) -> Self {
        let cond = Condition::empty(task.var_map.len()).mutate(
            *task.var_map.get(&name).expect(&format!(
                "Variable initalized, but name doesn't exist in var map: {name}"
            )),
            Some(AnyVal::Int(values)),
        );
        Self { name, values, cond }
    }
}

impl<T: Value> Program<T> for Variable<T>
where
    Bank: Store<T>,
{
    fn code(&self, _: &Bank) -> String {
        self.name.to_string()
    }

    fn values<'a>(&self, store: &'a Bank) -> &'a [T] {
        &store[self.values]
    }

    fn conditions(&self) -> (&PreCondition, &PostCondition) {
        (&self.cond, &self.cond)
    }
}

struct BinProgram<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
{
    pre: PreCondition,
    post: PostCondition,
    lhs: PIdx<L>,
    rhs: PIdx<R>,
    code: BinCode,
    values: VIdx<O>,
}

impl<L, R, O> BinProgram<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    fn new(
        lhs: PIdx<L>,
        rhs: PIdx<R>,
        values: VIdx<O>,
        code: BinCode,
        pre: PreCondition,
        post: PostCondition,
    ) -> Box<dyn Program<O>> {
        Box::new(Self {
            lhs,
            rhs,
            code,
            values,
            pre,
            post,
        })
    }
}

impl<L: Value, R: Value, O: Value> Program<O> for BinProgram<L, R, O>
where
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    fn code(&self, store: &Bank) -> String {
        let lhs = &store[self.lhs];
        let rhs = &store[self.rhs];
        let lhs = lhs.code(store);
        let rhs = rhs.code(store);
        (self.code)(&lhs, &rhs)
    }

    fn values<'s>(&self, store: &'s Bank) -> &'s [O] {
        &store[self.values]
    }

    fn conditions(&self) -> (&PreCondition, &PostCondition) {
        (&self.pre, &self.post)
    }
}

type BinProof<L, R> = &'static dyn Fn(&[L], &[R]) -> bool;
type BinEval<L, R, O> = &'static dyn Fn(
    &[L],
    &[R],
    PreCondition,
    PostCondition,
) -> (Vec<O>, PreCondition, PostCondition);
type BinCode = &'static dyn Fn(&str, &str) -> String;

struct BinBuilder<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
{
    proof: BinProof<L, R>,
    eval: BinEval<L, R, O>,
    code: BinCode,
}

impl<L, R, O> BinBuilder<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
    Bank: Store<L>,
    Bank: Store<R>,
    Bank: Store<O>,
{
    fn new(proof: BinProof<L, R>, eval: BinEval<L, R, O>, code: BinCode) -> Self {
        Self { proof, eval, code }
    }

    fn apply(
        &self,
        lhs: PIdx<L>,
        rhs: PIdx<R>,
        bank: &Bank,
    ) -> Option<(Vec<O>, PreCondition, PostCondition)> {
        let l_prog = &bank[lhs];
        let r_prog = &bank[rhs];

        let l_vals = l_prog.values(bank);
        let r_vals = r_prog.values(bank);

        if !(self.proof)(l_vals, r_vals) {
            return None;
        }

        if let Some((pre, post)) = Condition::sequence(l_prog.conditions(), r_prog.conditions()) {
            Some((self.eval)(l_vals, r_vals, pre, post))
        } else {
            None
        }
    }

    fn code(&self) -> BinCode {
        self.code
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AnyVal {
    Int(VIdx<Int>),
    Str(VIdx<Str>),
}

type PreCondition = Condition;
type PostCondition = Condition;

struct Condition {
    inner: SmallVec<[Option<AnyVal>; 4]>, // Up to 5 variables on the stack!
}

impl From<SmallVec<[Option<AnyVal>; 4]>> for Condition {
    fn from(inner: SmallVec<[Option<AnyVal>; 4]>) -> Self {
        Self { inner }
    }
}

impl Condition {
    fn empty(vars: usize) -> Self {
        Self {
            inner: smallvec![None; vars],
        }
    }

    fn implies(&self, other: &Condition) -> bool {
        debug_assert_eq!(self.inner.len(), other.inner.len());
        for i in 0..self.inner.len() {
            match (self.inner[i], other.inner[i]) {
                (Some(this), Some(that)) if this != that => return false,
                _ => (),
            }
        }

        true
    }

    /// Apply the sequence rule if possible and return the overall pre- and
    /// post condition for the sequence of the two statements.
    /// TODO This is in the hot-loop of the synthesizer. Optimize!
    fn sequence(
        fst: (&PreCondition, &PostCondition),
        snd: (&PreCondition, &PostCondition),
    ) -> Option<(PreCondition, PostCondition)> {
        if !fst.1.implies(snd.0) {
            return None;
        }

        let mut pre_condition = fst.0.inner.clone();
        let mut post_condition = fst.1.inner.clone();

        for i in 0..pre_condition.len() {
            if post_condition[i].is_none() {
                pre_condition[i] = snd.0.inner[i];
            }
        }

        for i in 0..post_condition.len() {
            post_condition[i] = snd.1.inner[i];
        }

        Some((pre_condition.into(), post_condition.into()))
    }

    fn mutate(&self, var: usize, val: Option<AnyVal>) -> Condition {
        let mut inner = self.inner.clone();
        inner[var] = val;
        Condition { inner }
    }
}

struct SynthesisTask {
    /// Map each variable to a vector index,
    /// so we can use vecs instead of HashMaps
    /// to keep state.
    var_map: HashMap<String, usize>,
    examples: usize,
}

fn sum_proof(_: &[Int], _: &[Int]) -> bool {
    true
}

fn sum_eval(
    lhs: &[Int],
    rhs: &[Int],
    pre: PreCondition,
    post: PostCondition,
) -> (Vec<Int>, PreCondition, PostCondition) {
    let rs = lhs.iter().zip(rhs).map(|(x, y)| x + y).collect();
    (rs, pre, post)
}

fn sum_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

fn main() {
    let mut store = Bank::new(1);
    let task = SynthesisTask {
        var_map: [("x".to_string(), 0), ("y".to_string(), 1)].into(),
        examples: 1,
    };

    let one_val = store.put_values(&[1]);
    let x: PIdx<Int> = store.put_program(Box::new(Variable::<Int>::new(
        "x".to_string(),
        one_val,
        &task,
    )));
    let two_val = store.put_values(&[2]);
    let y = store.put_program(Box::new(Variable::<Int>::new(
        "y".to_string(),
        two_val,
        &task,
    )));

    let sum = BinBuilder::new(&sum_proof, &sum_eval, &sum_code);
    match sum.apply(x, y, &store) {
        Some((vals, pre, post)) => {
            assert_eq!(&vals, &[3]);
            let three_val = store.put_values(&vals);
            let three = BinProgram::new(x, y, three_val, sum.code(), pre, post);
            let three = store.put_program(three);
            assert_eq!(store[three].values(&store), &[3]);
            let code = &store[three].code(&store);
            assert_eq!(code, "x + y");
            println!("Synthesized {code}");
        }
        None => unreachable!(),
    }
}
