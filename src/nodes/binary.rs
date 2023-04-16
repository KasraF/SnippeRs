use super::*;
use crate::utils::{Int, Val};

type BinOp<L, R, O> = &'static dyn Fn(&[L], &[R]) -> Vec<O>;
type BinCheck<L, R> = &'static dyn Fn(&[L], &[R]) -> bool;
type BinCode = &'static dyn Fn(&str, &str) -> String;

pub struct BinEnum<'s, L: Val, R: Val, O: Val> {
    op: BinOp<L, R, O>,
    check: BinCheck<L, R>,
    code: BinCode,
    store: &'s Store,
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

impl<'s, L: Val, R: Val, O: Val> Iterator for BinEnum<'s, L, R, O>
where
    Store: ProgramStore<L>,
    Store: ProgramStore<R>,
    Store: ProgramStore<O>,
{
    type Item = Box<dyn MaybeNode<O>>;

    fn next(&mut self) -> Option<Self::Item> {
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
        let values = (self.op)(lhs, rhs);

        // TODO Also need to check that the inputs are valid
        let rs = BinNodeMaybe::new(self.lhs_idx, self.rhs_idx, self.code, values);

        // TODO Increment the indices

        Some(Box::new(rs))
    }
}

pub struct BinNodeMaybe<L: Val, R: Val, O: Val> {
    lhs: Index<L>,
    rhs: Index<R>,
    code_fn: BinCode,
    values: Vec<O>,
}

impl<'s, L: Val, R: Val, O: Val> BinNodeMaybe<L, R, O>
where
    L: Val,
    R: Val,
    O: Val,
    Store: ProgramStore<L>,
    Store: ProgramStore<R>,
    Store: ProgramStore<O>,
{
    fn new(lhs: Index<L>, rhs: Index<R>, code_fn: BinCode, values: Vec<O>) -> Self {
        Self {
            lhs,
            rhs,
            code_fn,
            values,
        }
    }
}

impl<L, R, O> MaybeNode<O> for BinNodeMaybe<L, R, O>
where
    L: Val,
    R: Val,
    O: Val,
    Store: ProgramStore<L>,
    Store: ProgramStore<R>,
    Store: ProgramStore<O>,
{
    fn values<'a>(&'a self) -> &'a [O] {
        self.values.as_slice()
    }

    fn to_node(self: Box<Self>, node_index: Index<O>) -> (Box<dyn Node<O>>, Vec<O>) {
        let node = BinNode {
            lhs: self.lhs,
            rhs: self.rhs,
            code_fn: self.code_fn,
            values: node_index,
        };
        (Box::new(node), self.values)
    }
}

pub struct BinNode<L: Val, R: Val, O: Val> {
    lhs: Index<L>,
    rhs: Index<R>,
    code_fn: BinCode,
    values: Index<O>,
}

impl<'s, L, R, O> Node<O> for BinNode<L, R, O>
where
    L: Val,
    R: Val,
    O: Val,
    Store: ProgramStore<L>,
    Store: ProgramStore<R>,
    Store: ProgramStore<O>,
{
    fn code(&self, store: &Store) -> String {
        let lhs = store.program(self.lhs);
        let rhs = store.program(self.rhs);
        (self.code_fn)(lhs.code(store).as_ref(), rhs.code(store).as_ref())
    }

    fn values<'a>(&self, store: &'a Store) -> &'a [O] {
        store.values(self.values)
    }
}

fn true_check<L, R>(lhs: L, rhs: R) -> bool {
    true
}

fn add_op(lhs: &[Int], rhs: &[Int]) -> Vec<Int> {
    debug_assert!(
        lhs.len() == rhs.len(),
        "Arrays of different lengths given to add_op: {:?} vs. {:?}",
        lhs,
        rhs
    );
    lhs.iter().zip(rhs.iter()).map(|(l, r)| l + r).collect()
}

fn add_code(lhs: &str, rhs: &str) -> String {
    format!("{lhs} + {rhs}")
}

// fn add_node<'s>(store: &Store) -> Box<BinEnum<'s, Int, Int, Int>> {
//     Box::new(BinEnum::new(&add_op, &true_check, &add_code, store))
// }
