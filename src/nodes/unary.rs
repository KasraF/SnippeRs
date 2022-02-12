use crate::utils::*;
use crate::*;

pub struct UnaryEnum<A: Value, T: Value> {
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
    pub fn builder(
        validator: fn(&[A]) -> bool,
        value_fn: fn(&A) -> T,
        code_fn: fn(&str) -> String,
    ) -> Box<dyn Fn(usize) -> Box<dyn Enumerator<T>>> {
        Box::new(move |size: usize| Box::new(Self::new(size, validator, value_fn, code_fn)))
    }

    pub fn new(
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

pub struct UnaryOp<A: Value, T: Value> {
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
    pub fn new(
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
