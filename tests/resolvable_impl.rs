pub extern crate rosol;

use rosol::node::cause::Cause;
use rosol::node::Node;
use rosol::node::resolvable::Resolvable;
use rosol::node::resolved::Resolved;
use rosol::node::solvability::Solvability;
use rosol::package::ident::{Ident, SimpleUnique};
use rosol::path::Path;
use rosol::utils;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// Trivial implementation for demonstation / testing
pub struct Simple<T: Ident> {  // TODO: make them all non-generic (for simplicity)
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// Node represenation of boolean equations
pub struct OrAnd {
    pub or_dep: OrDependency
}

impl Resolvable for OrAnd {
    type Id = SimpleUnique;

    fn resolve<'a>(&'a self, path: Path<'a, Self>) -> Resolved<'a, Self> {
        self.or_dep.resolve_step(path)
    }
}

impl OrAnd {
    pub fn new(or_dep: OrDependency) -> Self {
        OrAnd { or_dep }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OrDependency {
    pub and_deps: Vec<AndDependency>
}

impl OrDependency {
    pub fn new(and_deps: Vec<AndDependency>) -> Self {
        OrDependency { and_deps }
    }

    pub fn single(and_dep: AndDependency) -> Self {
        Self::new(vec![and_dep])
    }

    pub fn resolve_step<'a>(&'a self, path: Path<'a, OrAnd>) -> Resolved<'a, OrAnd> {
        let results = self.and_deps
            .iter()
            .map(|dep| dep.resolve_step(path.clone()))
            .collect();

        Resolved::merge(results)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AndDependency {
    pub deps: Vec<Node<OrAnd>>
}

impl AndDependency {
    pub fn new(deps: Vec<Node<OrAnd>>) -> Self {
        AndDependency { deps }
    }

    pub fn single(dep: Node<OrAnd>) -> Self {
        Self::new(vec![dep])
    }

    pub fn resolve_step<'a>(&'a self, path: Path<'a, OrAnd>) -> Resolved<'a, OrAnd> {
        let subresults = self.deps
            .iter()
            .map(|node| node.solve(path.clone()))
            .collect();

        let cause = Self::added_cause(&subresults);

        let no_failures = subresults
            .iter()
            .all(|res| res.is_success());

        if no_failures {
            let paths_per_child = subresults
                .into_iter()
                .map(|res| res.paths);

            let megapaths =
                utils::selections(paths_per_child.collect())
                .into_iter()
                .map(|paths| Self::megapath(path.clone(), paths));

            Resolved::new(
                megapaths
                    .filter(|path| match Node::solvability(&path) {
                        Solvability::Ok => true,
                        _ => false
                    })
                    .collect(),
                cause)
        } else {
            // If any dependency failed, the node is unresolvable
            Resolved::failure(cause)
        }
    }

    fn added_cause<'a>(causes: &Vec<Resolved<'a, OrAnd>>) -> Cause<'a, OrAnd> {
       causes
           .iter()
           .fold(
               Cause::empty(),
               |acc, res| acc.merge(res.cause.clone()))
    }

    fn megapath<'a>(prefix: Path<'a, OrAnd>, paths: Vec<Path<'a, OrAnd>>) -> Path<'a, OrAnd> {
        let mut suffixes = paths
            .iter()
            .map(|path| path.suffix(&prefix))
            .collect();

        let mut parts = vec![prefix];
        parts.append(&mut suffixes);

        Path::chain(parts)
    }
}
