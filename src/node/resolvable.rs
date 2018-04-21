use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

use package::ident::Ident;
use path::Path;
use super::Node;
use super::resolved::Resolved;

pub trait Resolvable: Sized + Clone + Eq + Hash + PartialEq {
    type Id: Ident;

    fn resolve<'a>(&'a self, path: Path<'a, Self>) -> Resolved<'a, Self>;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// Trivial implementation for demonstation / testing
/// TODO: move to examples / tests
pub struct Simple<T: Ident> {
    pub node: Box<Node<Simple<T>>>
}

impl<T: Ident> Resolvable for Simple<T> {
    type Id = T;

    fn resolve<'a>(&'a self, path: Path<'a, Self>) -> Resolved<'a, Self> {
        self.node.solve(path)
    }
}

impl<T: Ident> Simple<T> {
    pub fn new(node: &Node<Simple<T>>) -> Self {
        Simple {
            node: Box::new(node.clone())
        }
    }
}
