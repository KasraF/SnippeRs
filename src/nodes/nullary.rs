use crate::{store::Store, Program};

pub struct Variable<T> {
    name: String,
    values: Vec<T>,
}

impl<T> Variable<T> {
    pub fn new(name: String, values: Vec<T>) -> Box<Self> {
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
