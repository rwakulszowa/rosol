pub extern crate rosol;

use rosol::node::Node;
use rosol::node::resolvable::Resolvable;
use rosol::node::resolved::Resolved;
use rosol::package::ident::Ident;
use rosol::path::Path;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// Trivial implementation for demonstation / testing
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// Node with alternative dependencies
pub struct Any<T: Ident> {
    pub deps: Vec<Node<Any<T>>>
}

impl<T: Ident> Resolvable for Any<T> {
    type Id = T;

    fn resolve<'a>(&'a self, path: Path<'a, Self>) -> Resolved<'a, Self> {
        let results = self.deps
            .iter()
            .map(|node| node.solve(path.clone()))
            .collect();

        Resolved::merge(results)
    }
}

impl<T: Ident> Any<T> {
    pub fn new(nodes: Vec<&Node<Any<T>>>) -> Self {
        let deps = nodes
            .into_iter()
            .map(|n| n.clone())
            .collect();

        Any {
            deps
        }
    }
}
