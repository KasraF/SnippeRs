use std::marker::PhantomData;

use crate::{
    ctx::{Context, Contexts, Ctx, Ctxs, Var, VarIter, VarMap, VariableMap},
    store::{ProgramStore, Store},
    utils::{Index, Val},
};

use super::{MaybeNode, Node, NodeEnum};

pub fn variable_node_enum<'s, T: Val>(store: &'s Store) -> Box<dyn NodeEnum<T> + 's>
where
    Context: Ctx<T>,
    Store: ProgramStore<T>,
    VariableMap: VarMap<T>,
{
    Box::new(VariableNodeEnum::new(store))
}

pub struct VariableNodeEnum<'s, T: Val> {
    vars: VarIter<T>,
    store: &'s Store,
    _phantom_data: PhantomData<T>,
}

impl<'s, T: Val> VariableNodeEnum<'s, T>
where
    VariableMap: VarMap<T>,
{
    pub fn new(store: &'s Store) -> Self {
        Self {
            vars: store.var_map.iter(),
            store,
            _phantom_data: PhantomData,
        }
    }
}

impl<'s, T: Val> Iterator for VariableNodeEnum<'s, T>
where
    Store: ProgramStore<T>,
    VariableMap: VarMap<T>,
    Contexts: Ctxs<T>,
{
    type Item = Box<dyn MaybeNode<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(var) = self.vars.next() {
            let val = self.store.ctxs.get(var);
            Some(Box::new(MaybeVariableNode { var, val }))
        } else {
            None
        }
    }
}

struct MaybeVariableNode<T: Val> {
    var: Var<T>,
    val: Vec<T>,
}

impl<T: Val> MaybeNode<T> for MaybeVariableNode<T>
where
    Store: ProgramStore<T>,
    VariableMap: VarMap<T>,
{
    fn values<'a>(&'a self) -> &'a [T] {
        &self.val
    }

    fn to_node(self: Box<Self>, node_index: Index<T>) -> (Box<dyn Node<T>>, Vec<T>) {
        (
            Box::new(VariableNode {
                var: self.var,
                val: node_index,
            }),
            self.val,
        )
    }
}

struct VariableNode<T: Val> {
    var: Var<T>,
    val: Index<T>,
}

impl<T: Val> Node<T> for VariableNode<T>
where
    Store: ProgramStore<T>,
    VariableMap: VarMap<T>,
{
    fn code(&self, store: &Store) -> String {
        store.var_map.get(self.var).to_string()
    }

    fn values<'a>(&self, store: &'a Store) -> &'a [T] {
        store.values(self.val)
    }
}
