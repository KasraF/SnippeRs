use std::collections::HashSet;
use std::ops::Index;

use crate::ops::Program;
use crate::utils::*;

pub struct MaxPIdx {
    int: PIdx<Int>,
    str: PIdx<Str>,
}

pub trait MaxIdx<T: Value> {
    fn check(&self, idx: PIdx<T>) -> bool;
}

impl MaxIdx<Int> for MaxPIdx {
    fn check(&self, idx: PIdx<Int>) -> bool {
        idx < self.int
    }
}

impl MaxIdx<Str> for MaxPIdx {
    fn check(&self, idx: PIdx<Str>) -> bool {
        idx < self.str
    }
}

pub trait Store<T: Value> {
    fn get_values(&self, idx: VIdx<T>) -> &[T];
    fn put_values(&mut self, values: Vec<T>) -> Option<VIdx<T>>;
    fn get_program(&self, idx: PIdx<T>) -> &Box<dyn Program<T>>;
    fn put_program(&mut self, program: Box<dyn Program<T>>) -> PIdx<T>;
    fn has_program(&self, idx: PIdx<T>) -> bool;
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
    int_oe: HashSet<Vec<Int>>,
    str_vals: Vec<Str>,
    strs: Vec<Box<dyn Program<Str>>>,
    str_oe: HashSet<Vec<Str>>,
}

impl Bank {
    pub fn new(examples: usize) -> Self {
        // TODO allocate larger chunks here?
        Self {
            examples,
            int_vals: Vec::new(),
            ints: Vec::new(),
            int_oe: HashSet::new(),
            str_vals: Vec::new(),
            strs: Vec::new(),
            str_oe: HashSet::new(),
        }
    }

    pub fn curr_max(&self) -> MaxPIdx {
        MaxPIdx {
            int: self.ints.len().into(),
            str: self.strs.len().into(),
        }
    }
}

impl Store<Int> for Bank {
    fn get_values(&self, idx: VIdx<Int>) -> &[Int] {
        &self.int_vals[idx.into()..idx + self.examples]
    }

    fn put_values(&mut self, values: Vec<Int>) -> Option<VIdx<Int>> {
        // First, check OE:
        if self.int_oe.contains(&values) {
            return None;
        }

        let start = self.int_vals.len();
        self.int_vals.extend(&values);
        self.int_oe.insert(values);
        Some(start.into())
    }

    fn get_program(&self, idx: PIdx<Int>) -> &Box<dyn Program<Int>> {
        &self.ints[idx]
    }

    fn put_program(&mut self, program: Box<dyn Program<Int>>) -> PIdx<Int> {
        self.ints.push(program);
        PIdx::from(self.ints.len() - 1)
    }

    fn has_program(&self, idx: PIdx<Int>) -> bool {
        self.ints.len() > idx.into()
    }
}

impl Store<Str> for Bank {
    fn get_values(&self, idx: VIdx<Str>) -> &[Str] {
        &self.str_vals[idx.into()..idx + self.examples]
    }

    fn put_values(&mut self, values: Vec<Str>) -> Option<VIdx<Str>> {
        if self.str_oe.contains(&values) {
            return None;
        }

        let start = self.str_vals.len();
        self.str_vals.extend_from_slice(&values);
        self.str_oe.insert(values);
        Some(start.into())
    }

    fn get_program(&self, idx: PIdx<Str>) -> &Box<dyn Program<Str>> {
        &self.strs[idx]
    }

    fn put_program(&mut self, program: Box<dyn Program<Str>>) -> PIdx<Str> {
        self.strs.push(program);
        PIdx::from(self.strs.len() - 1)
    }

    fn has_program(&self, idx: PIdx<Str>) -> bool {
        self.strs.len() > idx.into()
    }
}
