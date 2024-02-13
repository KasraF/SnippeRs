#[allow(dead_code)]
use std::marker::PhantomData;
use std::ops::Index;

type Int = i32;
type Str = String;
type IntArray = Vec<Int>;
type StrArray = Vec<Str>;

trait Value: Clone + Eq + 'static {}
impl Value for Int {}
impl Value for Str {}
impl Value for IntArray {}
impl Value for StrArray {}

#[derive(Clone)]
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
}

struct Variable<T: Value> {
    name: String,
    values: VIdx<T>,
}

impl<T: Value> Variable<T> {
    fn new(name: String, values: VIdx<T>) -> Self {
        Self { name, values }
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
}

struct BinProgram<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
{
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
    fn new(lhs: PIdx<L>, rhs: PIdx<R>, values: VIdx<O>, code: BinCode) -> Box<dyn Program<O>> {
        Box::new(Self {
            lhs,
            rhs,
            code,
            values,
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
}

type BinProof<L, R> = &'static dyn Fn(&[L], &[R]) -> bool;
type BinValues<L, R, O> = &'static dyn Fn(&[L], &[R]) -> Vec<O>;
type BinCode = &'static dyn Fn(&str, &str) -> String;

struct BinBuilder<L, R, O>
where
    L: Value,
    R: Value,
    O: Value,
{
    proof: BinProof<L, R>,
    values: BinValues<L, R, O>,
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
    fn new(proof: BinProof<L, R>, values: BinValues<L, R, O>, code: BinCode) -> Self {
        Self {
            proof,
            values,
            code,
        }
    }

    fn apply(&self, lhs: PIdx<L>, rhs: PIdx<R>, bank: &Bank) -> Option<Vec<O>> {
        let l_prog = &bank[lhs];
        let r_prog = &bank[rhs];

        let l_vals = l_prog.values(bank);
        let r_vals = r_prog.values(bank);

        if (self.proof)(l_vals, r_vals) {
            Some((self.values)(l_vals, r_vals))
        } else {
            None
        }
    }

    fn code(&self) -> BinCode {
        self.code
    }
}

fn sum_proof(_: &[Int], _: &[Int]) -> bool {
    true
}

fn sum_values(lhs: &[Int], rhs: &[Int]) -> Vec<Int> {
    let mut rs = Vec::with_capacity(lhs.len());
    for i in 0..lhs.len() {
        rs.push(lhs[i] + rhs[i]);
    }
    rs
}

fn sum_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

fn main() {
    let mut store = Bank::new(1);
    let one_val = store.put_values(&[1]);
    let x: PIdx<Int> = store.put_program(Box::new(Variable::new("x".to_string(), one_val)));
    let two_val = store.put_values(&[2]);
    let y = store.put_program(Box::new(Variable::new("y".to_string(), two_val)));

    let sum = BinBuilder::new(&sum_proof, &sum_values, &sum_code);
    match sum.apply(x, y, &store) {
        Some(vals) => {
            assert_eq!(&vals, &[3]);
            let three_val = store.put_values(&vals);
            let three = BinProgram::new(x, y, three_val, sum.code());
            let three = store.put_program(three);
            assert_eq!(store[three].values(&store), &[3]);
            assert_eq!(&store[three].code(&store), "x + y");
        }
        None => unreachable!(),
    }
}
