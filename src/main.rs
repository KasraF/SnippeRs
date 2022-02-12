use std::{collections::HashSet, fmt::Debug, hash::Hash, iter::Enumerate, marker::PhantomData};

type Int = i32;
type Str = String;
// type Bool = bool;

type BinValueFn<L, R, T> = fn((&L, &R)) -> T;
type BinCodeFn = fn(&str, &str) -> String;
type BinValidatorFn<L, R> = fn(&[L], &[R]) -> bool;

type Builder<T: Value> = Box<dyn Fn(usize) -> Box<dyn Enumerator<T>>>;

trait Value: Debug + Hash + 'static {}
impl Value for Int {}
impl Value for Str {}

const MAX_SIZE: usize = 4;

#[derive(Debug)]
struct Index<T: Value> {
    size: usize,
    idx: usize,
    _phantom: PhantomData<T>,
}

impl<T: Value> Clone for Index<T> {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            idx: self.idx,
            _phantom: PhantomData,
        }
    }
}

impl<T: Value> Copy for Index<T> {}

impl<T: Value> Index<T> {
    fn new(size: usize, idx: usize) -> Self {
        Self {
            size,
            idx,
            _phantom: PhantomData,
        }
    }
}

trait Enumerator<T> {
    fn next(&mut self, store: &Store) -> Option<Box<dyn Program<T>>>;
    fn has_next(&self, store: &Store) -> bool;
}

struct UnaryEnum<A: Value, T: Value> {
    arg: Index<A>,
    validator: fn(&[A]) -> bool,
    value_fn: fn(&A) -> T,
    code_fn: fn(&str) -> String,
}

impl<A, T> UnaryEnum<A, T>
where
    A: Value,
    T: Value,
    Store: ProgramStore<A>,
    Store: ProgramStore<T>,
{
    fn builder(
        validator: fn(&[A]) -> bool,
        value_fn: fn(&A) -> T,
        code_fn: fn(&str) -> String,
    ) -> Box<dyn Fn(usize) -> Box<dyn Enumerator<T>>> {
        Box::new(move |size: usize| Box::new(Self::new(size, validator, value_fn, code_fn)))
    }

    fn new(
        size: usize,
        validator: fn(&[A]) -> bool,
        value_fn: fn(&A) -> T,
        code_fn: fn(&str) -> String,
    ) -> Self {
        let arg = Index::new(size - 1, 0);
        Self {
            arg,
            validator,
            value_fn,
            code_fn,
        }
    }
}

impl<A, T> Enumerator<T> for UnaryEnum<A, T>
where
    A: Value,
    T: Value,
    Store: ProgramStore<A>,
    Store: ProgramStore<T>,
{
    fn next(&mut self, store: &Store) -> Option<Box<dyn Program<T>>> {
        if !store.has(self.arg) {
            return None;
        }

        let arg = (self.arg, store.get_unchecked(self.arg));
        self.arg.idx += 1;

        if (self.validator)(arg.1.values()) {
            Some(UnaryOp::new(arg, self.value_fn, self.code_fn))
        } else {
            self.next(store)
        }
    }

    fn has_next(&self, store: &Store) -> bool {
        store.has(self.arg)
    }
}

struct BinEnum<L: Value, R: Value, T: Value> {
    lhs: Index<L>,
    rhs: Index<R>,
    validator: BinValidatorFn<L, R>,
    value_fn: BinValueFn<L, R, T>,
    code_fn: BinCodeFn,
}

impl<L, R, T> BinEnum<L, R, T>
where
    L: Value,
    R: Value,
    T: Value,
    Store: ProgramStore<L>,
    Store: ProgramStore<R>,
{
    fn builder(
        validator: BinValidatorFn<L, R>,
        value_fn: BinValueFn<L, R, T>,
        code_fn: BinCodeFn,
    ) -> Box<dyn Fn(usize) -> Box<dyn Enumerator<T>>> {
        Box::new(move |size: usize| Box::new(Self::new(size, validator, value_fn, code_fn)))
    }

    fn new(
        size: usize,
        validator: BinValidatorFn<L, R>,
        value_fn: BinValueFn<L, R, T>,
        code_fn: BinCodeFn,
    ) -> Self {
        let lhs = Index::new(0, 0);
        let rhs = Index::new(size - 1, 0);
        Self {
            lhs,
            rhs,
            validator,
            value_fn,
            code_fn,
        }
    }

    fn update_indices(&mut self, store: &Store) {
        self.rhs.idx += 1;
        if !store.has(self.rhs) {
            self.lhs.idx += 1;
            self.rhs.idx = 0;

            if !store.has(self.lhs) {
                // Change height!
                if self.rhs.size > 0 {
                    self.lhs.size += 1;
                    self.lhs.idx = 0;

                    self.rhs.size -= 1;
                    self.rhs.idx = 0;
                }
            }
        }
    }
}

impl<L, R, T> Enumerator<T> for BinEnum<L, R, T>
where
    L: Value,
    R: Value,
    T: Value,
    Store: ProgramStore<L>,
    Store: ProgramStore<R>,
{
    fn next(&mut self, store: &Store) -> Option<Box<dyn Program<T>>> {
        if !self.has_next(store) {
            return None;
        }

        let lhs = (self.lhs, store.get_unchecked(self.lhs));
        let rhs = (self.rhs, store.get_unchecked(self.rhs));

        self.update_indices(store);

        if (self.validator)(lhs.1.values(), rhs.1.values()) {
            Some(BinOp::new(lhs, rhs, self.value_fn, self.code_fn))
        } else {
            self.next(store)
        }
    }

    fn has_next(&self, store: &Store) -> bool {
        store.has(self.lhs) && store.has(self.rhs)
    }
}

trait ProgramStore<T: Value> {
    fn get(&self, index: Index<T>) -> Option<&Box<dyn Program<T>>>;
    fn put(&mut self, program: Box<dyn Program<T>>) -> Option<Index<T>>;
    fn get_unchecked(&self, index: Index<T>) -> &Box<dyn Program<T>>;
    fn has(&self, index: Index<T>) -> bool;
}

struct Store {
    ints: [Vec<Box<dyn Program<Int>>>; MAX_SIZE + 1],
    int_set: HashSet<Vec<Int>>,
    strs: [Vec<Box<dyn Program<Str>>>; MAX_SIZE + 1],
    str_set: HashSet<Vec<Str>>,
}

impl ProgramStore<Int> for Store {
    fn get(&self, index: Index<Int>) -> Option<&Box<dyn Program<Int>>> {
        if self.has(index) {
            Some(&self.ints[index.size][index.idx])
        } else {
            None
        }
    }

    fn put(&mut self, program: Box<dyn Program<Int>>) -> Option<Index<Int>> {
        let values = program.values();

        if self.int_set.contains(values) {
            None
        } else {
            self.int_set.insert(values.to_vec());
            let size = program.size() as usize;
            self.ints[size].push(program);
            Some(Index::new(size, self.ints[size].len() - 1))
        }
    }

    fn get_unchecked(&self, index: Index<Int>) -> &Box<dyn Program<Int>> {
        &self.ints[index.size][index.idx]
    }

    fn has(&self, index: Index<Int>) -> bool {
        index.size <= MAX_SIZE && index.idx < self.ints[index.size].len()
    }
}

impl ProgramStore<Str> for Store {
    fn get(&self, index: Index<Str>) -> Option<&Box<dyn Program<Str>>> {
        if self.has(index) {
            Some(&self.strs[index.size][index.idx])
        } else {
            None
        }
    }

    fn put(&mut self, program: Box<dyn Program<Str>>) -> Option<Index<Str>> {
        let values = program.values();

        if self.str_set.contains(values) {
            None
        } else {
            self.str_set.insert(values.to_vec());
            let size = program.size() as usize;
            self.strs[size].push(program);
            Some(Index::new(size, self.strs[size].len() - 1))
        }
    }

    fn get_unchecked(&self, index: Index<Str>) -> &Box<dyn Program<Str>> {
        &self.strs[index.size][index.idx]
    }

    fn has(&self, index: Index<Str>) -> bool {
        index.size < MAX_SIZE && index.idx < self.strs[index.size].len()
    }
}

impl Store {
    fn new() -> Self {
        let ints_zero: Vec<Box<dyn Program<Int>>> = vec![
            Variable::new("x".to_string(), vec![1, 2]),
            Variable::new("y".to_string(), vec![2, 4]),
        ];

        let mut ints = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        ints[0] = ints_zero;

        let strs = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        Self {
            ints,
            int_set: HashSet::new(),
            strs,
            str_set: HashSet::new(),
        }
    }
}

trait Program<T> {
    fn values(&self) -> &[T];
    fn size(&self) -> u8;
    fn code(&self, store: &Store) -> String;
}

struct UnaryOp<A: Value, T: Value> {
    arg: Index<A>,
    values: Vec<T>,
    size: u8,
    code_fn: fn(&str) -> String,
}

impl<A, T> UnaryOp<A, T>
where
    A: Value,
    T: Value,
{
    fn new(
        arg: (Index<A>, &Box<dyn Program<A>>),
        value_fn: fn(&A) -> T,
        code_fn: fn(&str) -> String,
    ) -> Box<Self> {
        let values = arg.1.values().iter().map(value_fn).collect();
        let size = arg.1.size() + 1;
        Box::new(Self {
            arg: arg.0,
            values,
            size,
            code_fn,
        })
    }
}

impl<A, T> Program<T> for UnaryOp<A, T>
where
    A: Value,
    T: Value,
    Store: ProgramStore<A>,
    Store: ProgramStore<T>,
{
    fn values(&self) -> &[T] {
        &self.values
    }

    fn code(&self, store: &Store) -> String {
        let arg = store.get_unchecked(self.arg);
        (self.code_fn)(&arg.code(store))
    }

    fn size(&self) -> u8 {
        self.size
    }
}

struct BinOp<L: Value, R: Value, T: Value> {
    lhs: Index<L>,
    rhs: Index<R>,
    values: Vec<T>,
    size: u8,
    code_fn: BinCodeFn,
}

impl<L, R, T> BinOp<L, R, T>
where
    L: Value,
    R: Value,
    T: Value,
{
    fn new(
        lhs: (Index<L>, &Box<dyn Program<L>>),
        rhs: (Index<R>, &Box<dyn Program<R>>),
        value_fn: fn((&L, &R)) -> T,
        code_fn: fn(&str, &str) -> String,
    ) -> Box<Self> {
        let values = lhs
            .1
            .values()
            .iter()
            .zip(rhs.1.values().iter())
            .map(value_fn)
            .collect();
        let size = lhs.1.size() + rhs.1.size() + 1;
        Box::new(Self {
            lhs: lhs.0,
            rhs: rhs.0,
            values,
            size,
            code_fn,
        })
    }
}

impl<L, R, T> Program<T> for BinOp<L, R, T>
where
    L: Value,
    R: Value,
    T: Value,
    Store: ProgramStore<L>,
    Store: ProgramStore<R>,
{
    fn values(&self) -> &[T] {
        &self.values
    }

    fn code(&self, store: &Store) -> String {
        let lhs = store.get_unchecked(self.lhs);
        let rhs = store.get_unchecked(self.rhs);
        (self.code_fn)(&lhs.code(store), &rhs.code(store))
    }

    fn size(&self) -> u8 {
        self.size
    }
}

fn unary_true_validator<A>(_: &[A]) -> bool {
    true
}

fn bin_true_validator<L, R>(_: &[L], _: &[R]) -> bool {
    true
}
fn add_value((lhs, rhs): (&Int, &Int)) -> Int {
    lhs + rhs
}

fn add_code(lhs: &str, rhs: &str) -> String {
    format!("{} + {}", lhs, rhs)
}

fn sub_value((lhs, rhs): (&Int, &Int)) -> Int {
    lhs - rhs
}

fn sub_code(lhs: &str, rhs: &str) -> String {
    format!("{} - {}", lhs, rhs)
}

fn to_string_code(arg: &str) -> String {
    format!("str({})", arg)
}

fn to_string_value(arg: &Int) -> String {
    arg.to_string()
}

struct Variable<T> {
    name: String,
    values: Vec<T>,
}

impl<T> Variable<T> {
    fn new(name: String, values: Vec<T>) -> Box<Self> {
        Box::new(Self { name, values })
    }
}

impl<T> Program<T> for Variable<T> {
    fn values(&self) -> &[T] {
        &self.values
    }

    fn size(&self) -> u8 {
        0
    }

    fn code(&self, _: &Store) -> String {
        self.name.clone()
    }
}

enum SynthEnum {
    Int(usize, Box<dyn Enumerator<Int>>),
    Str(usize, Box<dyn Enumerator<Str>>),
}

struct Synthesizer {
    store: Store,
    int_enums: Vec<Builder<Int>>,
    str_enums: Vec<Builder<Str>>,
    curr_enum: SynthEnum,
    size: usize,
}

impl Synthesizer {
    fn new() -> Self {
        let int_enums = vec![
            // UnaryEnum::builder(unary_true_validator, to_string_value, to_string_code),
            BinEnum::builder(bin_true_validator, add_value, add_code),
            BinEnum::builder(bin_true_validator, sub_value, sub_code),
        ];

        let str_enums = vec![UnaryEnum::builder(
            unary_true_validator,
            to_string_value,
            to_string_code,
        )];

        let size = 1;

        let curr_enum = SynthEnum::Int(0, int_enums[0](size));

        Self {
            store: Store::new(),
            int_enums,
            str_enums,
            curr_enum,
            size,
        }
    }
}

impl Iterator for Synthesizer {
    type Item = Option<String>;

    fn next(&mut self) -> Option<Self::Item> {
        while match &self.curr_enum {
            SynthEnum::Int(_, e) => !e.has_next(&self.store),
            SynthEnum::Str(_, e) => !e.has_next(&self.store),
        } {
            // Get the next enum
            self.curr_enum = match &self.curr_enum {
                SynthEnum::Int(i, _) => {
                    if self.int_enums.len() > i + 1 {
                        SynthEnum::Int(i + 1, (self.int_enums[*i])(self.size))
                    } else {
                        SynthEnum::Str(0, (self.str_enums[0])(self.size))
                    }
                }
                SynthEnum::Str(i, _) => {
                    if self.str_enums.len() > i + 1 {
                        SynthEnum::Str(i + 1, (self.str_enums[*i])(self.size))
                    } else {
                        // We need to go to the next size!
                        if self.size == MAX_SIZE {
                            return None;
                        }
                        self.size += 1;
                        SynthEnum::Int(0, (self.int_enums[0])(self.size))
                    }
                }
            }
        }

        // We are at a valid enumerator, so just enumerate!
        match &mut self.curr_enum {
            SynthEnum::Int(_, e) => {
                let program = e.next(&self.store).unwrap();
                let idx = self.store.put(program);
                if let Some(idx) = idx {
                    let code = self.store.get_unchecked(idx).code(&self.store);
                    dbg!(code);
                }
            }
            SynthEnum::Str(_, e) => {
                let program = e.next(&self.store).unwrap();
                let idx = self.store.put(program);
                if let Some(idx) = idx {
                    let code = self.store.get_unchecked(idx).code(&self.store);
                    dbg!(code);
                }
            }
        }

        Some(None)
    }
}

fn main() {
    let synth = Synthesizer::new();

    for p in synth {
        if let Some(code) = p {
            println!("Done: {}", code);
        }
    }
}
