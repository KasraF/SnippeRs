use super::*;
use crate::utils::{Int, Val};

type BinOp<L, R, O> = &'static dyn Fn(&[L], &[R]) -> Vec<O>;
type BinCheck<L, R> = &'static dyn Fn(&[L], &[R]) -> bool;
type BinCode = &'static dyn Fn(&str, &str) -> String;

pub struct BinEnum<'s, L: Val, R: Val, O: Val> {
    op: BinOp<L, R, O>,
    check: BinCheck<L, R>,
    code: BinCode,
    store: &'s Store<'s>,
    lhs_idx: Index<L>,
    rhs_idx: Index<R>,
}

impl<'s, L: Val, R: Val, O: Val> BinEnum<'s, L, R, O> {
    pub fn new(op: BinOp<L, R, O>, check: BinCheck<L, R>, code: BinCode, store: &'s Store) -> Self {
        let lhs_idx = Index::new(0);
        let rhs_idx = Index::new(0);
        Self {
            op,
            check,
            store,
            code,
            lhs_idx,
            rhs_idx,
        }
    }
}

impl<'s, L: Val, R: Val, O: Val> NodeEnum<O> for BinEnum<'s, L, R, O> {
    fn next(&mut self) -> Box<dyn MaybeNode<O>> {
        // See if we're done
        if !self.store.has(self.lhs_idx) {
            debug_assert!(
                !self.store.has(self.rhs_idx),
                "Didn't update the indices properly: Lhs ({}) was invalid, but rhs ({}) is fine.",
                self.lhs_idx,
                self.rhs_idx
            );
            return None;
        }

        debug_assert!(
            self.store.has(self.rhs_idx),
            "Didn't update the indices properly: Lhs ({}) is valid, but Rhs ({}) is not.",
            self.lhs_idx,
            self.rhs_idx
        );

        // Build a new node
        let lhs = self.store.values(self.lhs_idx);
        let rhs = self.store.values(self.rhs_idx);
        let values = self.op(lhs, rhs);
        // TODO Also need to check that the inputs are valid
        let rs = BinNodeMaybe::new(self.lhs_idx, self.rhs_idx, self.code, values);

        // TODO Increment the indices

        Some(rs)
    }
}

pub struct BinNodeMaybe<L: Val, R: Val, O: Val> {
    lhs: Index<L>,
    rhs: Index<R>,
    code_fn: BinCode,
    values: Vec<O>,
}

impl<L: Val, R: Val, O: Val> BinNodeMaybe<L, R, O> {
    fn new(lhs: Index<L>, rhs: Index<R>, code_fn: BinCode, values: Vec<O>) -> Self {
        Self {
            lhs,
            rhs,
            code_fn,
            values,
        }
    }

    pub fn into_node(self, store: &mut Store) -> BinNode<L, R, O> {
        let values = store.insert(&self.values);
        BinNode {
            lhs: self.lhs,
            rhs: self.rhs,
            code_fn: self.code_fn,
            values,
        }
    }
}

pub struct BinNode<L: Val, R: Val, O: Val> {
    lhs: Index<L>,
    rhs: Index<R>,
    code_fn: BinCode,
    values: Index<O>,
}

impl<L: Val, R: Val, O: Val> Node<O> for BinNode<L, R, O> {
    fn code(&self, store: &Store) -> String {
        let lhs = store.program(self.lhs);
        let rhs = store.program(self.rhs);
        self.code_fn(lhs.code(store), rhs.code(store))
    }

    fn values<'a>(&self, store: &'a Store) -> &'a [O] {
        store.values(self.values)
    }
}

fn true_check<L, R>(lhs: L, rhs: R) -> bool {
    true
}

fn add_op(lhs: Int, rhs: Int) -> Int {
    return lhs + rhs;
}

fn add_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

fn add_node<'s>(store: &Store) -> Box<BinEnum<'s, Int, Int, Int>> {
    BinEnum::new(add_op, &true_check, &add_code, store)
}
