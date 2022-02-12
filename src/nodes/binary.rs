use crate::utils::*;
use crate::*;

pub struct BinEnum<L: Value, R: Value, T: Value> {
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
    pub fn builder(
        validator: BinValidatorFn<L, R>,
        value_fn: BinValueFn<L, R, T>,
        code_fn: BinCodeFn,
    ) -> Box<dyn Fn(usize) -> Box<dyn Enumerator<T>>> {
        Box::new(move |size: usize| Box::new(Self::new(size, validator, value_fn, code_fn)))
    }

    pub fn new(
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

pub struct BinOp<L: Value, R: Value, T: Value> {
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
    pub fn new(
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
