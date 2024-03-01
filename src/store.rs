use std::collections::HashMap;
use std::ops::Index;

use crate::utils::*;
use crate::*;

use self::task::VarMap;

type OEKey<T> = (Vec<T>, Option<Pointer>, PreCondition, PostCondition);
type VarKey<T> = (String, Vec<T>);

#[derive(Debug)]
pub struct MaxPIdx {
    int: PIdx<Int>,
    str: PIdx<Str>,
    int_arrs: PIdx<IntArray>,
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
impl MaxIdx<IntArray> for MaxPIdx {
    fn check(&self, idx: PIdx<IntArray>) -> bool {
        idx < self.int_arrs
    }
}

pub trait Store<T: Value> {
    fn get_values(&self, idx: VIdx<T>) -> &[T];
    fn get_program(&self, idx: PIdx<T>) -> &Box<dyn Program<T>>;

    /// Tries to add the MaybeProgram to the store.
    /// If this is a new program, it will add it to the store and return Ok(Idx).
    /// If an equivalent program exists, it returns Error() with the index of that program.
    fn put_program(&mut self, program: Box<dyn MaybeProgram<T>>) -> Result<PIdx<T>, PIdx<T>>;
    fn has_program(&self, idx: PIdx<T>) -> bool;

    fn put_variable(
        &mut self,
        name: String,
        values: Vec<T>,
        pointer: Pointer,
    ) -> Result<PIdx<T>, PIdx<T>>;

    fn put_constant(&mut self, code: &str, values: T) -> Result<PIdx<T>, PIdx<T>>;
}

impl<T> Index<VIdx<T>> for Bank
where
    T: Value,
    Bank: Store<T>,
{
    type Output = [T];

    #[inline]
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

    #[inline]
    fn index(&self, index: PIdx<T>) -> &Self::Output {
        self.get_program(index)
    }
}

pub struct Bank {
    examples: usize,
    var_map: VarMap,

    // Integers
    int_vals: Vec<Int>,
    ints: Vec<Box<dyn Program<Int>>>,
    int_oe: HashMap<OEKey<Int>, PIdx<Int>>,
    int_vars: HashMap<VarKey<Int>, PIdx<Int>>,

    // Strings
    str_vals: Vec<Str>,
    strs: Vec<Box<dyn Program<Str>>>,
    str_oe: HashMap<OEKey<Str>, PIdx<Str>>,
    str_vars: HashMap<VarKey<Str>, PIdx<Str>>,

    // Int Arrays
    int_arr_vals: Vec<IntArray>,
    int_arrs: Vec<Box<dyn Program<IntArray>>>,
    int_arr_oe: HashMap<OEKey<IntArray>, PIdx<IntArray>>,
    int_arr_vars: HashMap<VarKey<IntArray>, PIdx<IntArray>>,
}

impl Bank {
    pub fn new(examples: usize, var_map: VarMap) -> Self {
        // TODO allocate larger chunks here?
        Self {
            examples,
            var_map,
            int_vals: Vec::new(),
            ints: Vec::new(),
            int_oe: HashMap::new(),
            int_vars: HashMap::new(),
            str_vals: Vec::new(),
            strs: Vec::new(),
            str_oe: HashMap::new(),
            str_vars: HashMap::new(),
            int_arr_vals: Vec::new(),
            int_arrs: Vec::new(),
            int_arr_oe: HashMap::new(),
            int_arr_vars: HashMap::new(),
        }
    }

    pub fn curr_max(&self) -> MaxPIdx {
        MaxPIdx {
            int: self.ints.len().into(),
            str: self.strs.len().into(),
            int_arrs: self.int_arrs.len().into(),
        }
    }

    pub fn var_map(&self) -> &VarMap {
        &self.var_map
    }

    pub fn variables(&self) -> usize {
        self.var_map.len()
    }
}

impl Store<Int> for Bank {
    fn get_values(&self, idx: VIdx<Int>) -> &[Int] {
        &self.int_vals[idx.into()..idx + self.examples]
    }

    fn get_program(&self, idx: PIdx<Int>) -> &Box<dyn Program<Int>> {
        &self.ints[idx]
    }

    fn put_program(
        &mut self,
        mut program: Box<dyn MaybeProgram<Int>>,
    ) -> Result<PIdx<Int>, PIdx<Int>> {
        // First, check OE
        let values = program
            .extract_values()
            .expect("Incomplete MaybeProgram was given to the store. This should not happen.");

        // TODO This *hurts* :`(
        let pre = program.pre_condition().clone();
        let post = program.post_condition().clone();

        let oe_key: OEKey<Int> = (values, program.pointer(), pre, post);

        if let Some(idx) = self.int_oe.get(&oe_key) {
            return Err(*idx);
        }

        // insert the values
        let val_idx = self.int_vals.len().into();
        self.int_vals.extend(oe_key.0.clone());
        let prog_idx = self.ints.len().into();

        // add to OE
        self.int_oe.insert(oe_key, prog_idx);

        // add the program
        self.ints.push(program.into_program(val_idx));

        Ok(prog_idx)
    }

    fn has_program(&self, idx: PIdx<Int>) -> bool {
        self.ints.len() > idx.into()
    }

    fn put_variable(
        &mut self,
        name: String,
        values: Vec<Int>,
        pointer: Pointer,
    ) -> Result<PIdx<Int>, PIdx<Int>> {
        // First, check to see if this variable already exists.
        let key = (name, values);
        if let Some(idx) = self.int_vars.get(&key) {
            // This variable already exists!
            Err(*idx)
        } else {
            // Add the variable as a new program and return the index

            let val_idx = self.int_vals.len().into();
            let prog_idx = self.ints.len().into();
            self.int_vals.extend_from_slice(&key.1);
            let val_program =
                Variable::<Int>::new(key.0.clone(), val_idx, pointer, self.variables());
            let (pre, post) = val_program.conditions();
            let (pre, post) = (pre.clone(), post.clone());
            self.ints.push(val_program);

            // Each variable is provably unique
            let oe_key: OEKey<Int> = (key.1, Some(pointer), pre, post);
            debug_assert!(!self.int_oe.contains_key(&oe_key));
            self.int_oe.insert(oe_key, prog_idx);

            Ok(prog_idx)
        }
    }

    fn put_constant(&mut self, code: &str, value: Int) -> Result<PIdx<Int>, PIdx<Int>> {
        let values = vec![value; self.examples];
        let empty = Condition::empty(self.variables());
        let oe_key: OEKey<Int> = (values, None, empty.clone(), empty);

        if let Some(idx) = self.int_oe.get(&oe_key) {
            return Err(*idx);
        }

        let val_idx = self.int_vals.len().into();
        let prog_idx = self.ints.len().into();
        self.int_vals.extend_from_slice(&oe_key.0);
        let program = Constant::new(code.to_string(), val_idx, self.variables());
        self.ints.push(program);

        Ok(prog_idx)
    }
}

impl Store<Str> for Bank {
    fn get_values(&self, idx: VIdx<Str>) -> &[Str] {
        &self.str_vals[idx.into()..idx + self.examples]
    }

    fn get_program(&self, idx: PIdx<Str>) -> &Box<dyn Program<Str>> {
        &self.strs[idx]
    }

    fn put_program(
        &mut self,
        mut program: Box<dyn MaybeProgram<Str>>,
    ) -> Result<PIdx<Str>, PIdx<Str>> {
        // First, check OE
        let values = program
            .extract_values()
            .expect("Incomplete MaybeProgram was given to the store. This should not happen.");

        // TODO This *hurts* :`(
        let pre = program.pre_condition().clone();
        let post = program.post_condition().clone();

        let oe_key: OEKey<Str> = (values, program.pointer(), pre, post);

        if let Some(idx) = self.str_oe.get(&oe_key) {
            return Err(*idx);
        }

        // insert the values
        let val_idx = self.str_vals.len().into();
        self.str_vals.extend(oe_key.0.clone());
        let prog_idx = self.strs.len().into();

        // add to OE
        self.str_oe.insert(oe_key, prog_idx);

        // add the program
        self.strs.push(program.into_program(val_idx));

        Ok(prog_idx)
    }

    fn has_program(&self, idx: PIdx<Str>) -> bool {
        self.strs.len() > idx.into()
    }

    fn put_variable(
        &mut self,
        name: String,
        values: Vec<Str>,
        pointer: Pointer,
    ) -> Result<PIdx<Str>, PIdx<Str>> {
        // First, check to see if this variable already exists.
        let key = (name, values);
        if let Some(idx) = self.str_vars.get(&key) {
            // This variable already exists!
            Err(*idx)
        } else {
            // Add the variable as a new program and return the index

            let val_idx = self.str_vals.len().into();
            let prog_idx = self.strs.len().into();
            self.str_vals.extend_from_slice(&key.1);
            let val_program =
                Variable::<Str>::new(key.0.clone(), val_idx, pointer, self.variables());
            let (pre, post) = val_program.conditions();
            let (pre, post) = (pre.clone(), post.clone());
            self.strs.push(val_program);

            // Each variable is provably unique
            let oe_key: OEKey<Str> = (key.1, Some(pointer), pre, post);
            debug_assert!(!self.str_oe.contains_key(&oe_key));
            self.str_oe.insert(oe_key, prog_idx);

            Ok(prog_idx)
        }
    }

    fn put_constant(&mut self, code: &str, value: Str) -> Result<PIdx<Str>, PIdx<Str>> {
        let values = vec![value; self.examples];
        let empty = Condition::empty(self.variables());
        let oe_key: OEKey<Str> = (values, None, empty.clone(), empty);

        if let Some(idx) = self.str_oe.get(&oe_key) {
            return Err(*idx);
        }

        let val_idx = self.str_vals.len().into();
        let prog_idx = self.strs.len().into();
        self.str_vals.extend_from_slice(&oe_key.0);
        let program = Constant::new(code.to_string(), val_idx, self.variables());
        self.strs.push(program);

        Ok(prog_idx)
    }
}
impl Store<IntArray> for Bank {
    fn get_values(&self, idx: VIdx<IntArray>) -> &[IntArray] {
        &self.int_arr_vals[idx.into()..idx + self.examples]
    }

    fn get_program(&self, idx: PIdx<IntArray>) -> &Box<dyn Program<IntArray>> {
        &self.int_arrs[idx]
    }

    fn put_program(
        &mut self,
        mut program: Box<dyn MaybeProgram<IntArray>>,
    ) -> Result<PIdx<IntArray>, PIdx<IntArray>> {
        // First, check OE
        let values = program
            .extract_values()
            .expect("Incomplete MaybeProgram was given to the store. This should not happen.");

        // TODO This *hurts* :`(
        let pre = program.pre_condition().clone();
        let post = program.post_condition().clone();

        let oe_key: OEKey<IntArray> = (values, program.pointer(), pre, post);

        if let Some(idx) = self.int_arr_oe.get(&oe_key) {
            return Err(*idx);
        }

        // insert the values
        let val_idx = self.int_arr_vals.len().into();
        self.int_arr_vals.extend(oe_key.0.clone());
        let prog_idx = self.int_arrs.len().into();

        // add to OE
        self.int_arr_oe.insert(oe_key, prog_idx);

        // add the program
        self.int_arrs.push(program.into_program(val_idx));

        Ok(prog_idx)
    }

    fn has_program(&self, idx: PIdx<IntArray>) -> bool {
        self.strs.len() > idx.into()
    }

    fn put_variable(
        &mut self,
        name: String,
        values: Vec<IntArray>,
        pointer: Pointer,
    ) -> Result<PIdx<IntArray>, PIdx<IntArray>> {
        // First, check to see if this variable already exists.
        let key = (name, values);
        if let Some(idx) = self.int_arr_vars.get(&key) {
            // This variable already exists!
            Err(*idx)
        } else {
            // Add the variable as a new program and return the index

            let val_idx = self.int_arr_vals.len().into();
            let prog_idx = self.int_arrs.len().into();
            self.int_arr_vals.extend_from_slice(&key.1);
            let val_program =
                Variable::<IntArray>::new(key.0.clone(), val_idx, pointer, self.variables());
            let (pre, post) = val_program.conditions();
            let (pre, post) = (pre.clone(), post.clone());
            self.int_arrs.push(val_program);

            // Each variable is provably unique
            let oe_key: OEKey<IntArray> = (key.1, Some(pointer), pre, post);
            debug_assert!(!self.int_arr_oe.contains_key(&oe_key));
            self.int_arr_oe.insert(oe_key, prog_idx);

            Ok(prog_idx)
        }
    }

    fn put_constant(
        &mut self,
        code: &str,
        value: IntArray,
    ) -> Result<PIdx<IntArray>, PIdx<IntArray>> {
        let values = vec![value; self.examples];
        let empty = Condition::empty(self.variables());
        let oe_key: OEKey<IntArray> = (values, None, empty.clone(), empty);

        if let Some(idx) = self.int_arr_oe.get(&oe_key) {
            return Err(*idx);
        }

        let val_idx = self.int_arr_vals.len().into();
        let prog_idx = self.int_arrs.len().into();
        self.int_arr_vals.extend_from_slice(&oe_key.0);
        let program = Constant::new(code.to_string(), val_idx, self.variables());
        self.int_arrs.push(program);

        Ok(prog_idx)
    }
}
