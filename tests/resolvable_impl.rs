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
