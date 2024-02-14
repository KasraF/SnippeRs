use std::ops::Index;

use crate::ops::Program;
use crate::utils::*;

pub trait Store<T: Value> {
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

pub struct Bank {
    examples: usize,
    int_vals: Vec<Int>,
    ints: Vec<Box<dyn Program<Int>>>,
    str_vals: Vec<Str>,
    strs: Vec<Box<dyn Program<Str>>>,
}

impl Bank {
    pub fn new(examples: usize) -> Self {
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
        &self.int_vals[idx.into()..idx + self.examples]
    }

    fn put_values(&mut self, values: &[Int]) -> VIdx<Int> {
        let start = self.int_vals.len();
        self.int_vals.extend(values);
        start.into()
    }

    fn get_program(&self, idx: PIdx<Int>) -> &Box<dyn Program<Int>> {
        &self.ints[idx]
    }

    fn put_program(&mut self, program: Box<dyn Program<Int>>) -> PIdx<Int> {
        self.ints.push(program);
        PIdx::from(self.ints.len() - 1)
    }
}

impl Store<Str> for Bank {
    fn get_values(&self, idx: VIdx<Str>) -> &[Str] {
        &self.str_vals[idx.into()..idx + self.examples]
    }

    fn put_values(&mut self, values: &[Str]) -> VIdx<Str> {
        let start = self.str_vals.len();
        self.str_vals.extend_from_slice(values);
        start.into()
    }

    fn get_program(&self, idx: PIdx<Str>) -> &Box<dyn Program<Str>> {
        &self.strs[idx]
    }

    fn put_program(&mut self, program: Box<dyn Program<Str>>) -> PIdx<Str> {
        self.strs.push(program);
        PIdx::from(self.strs.len() - 1)
    }
}
