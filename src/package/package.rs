use std::cmp::Eq;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::hash::Hash;

use node::Node;
use node::resolvable::Resolvable;
use package::dependency::Dependency;
use package::ident::Ident;

pub trait Package: Clone + Debug + Eq + Hash + PartialEq {
    type Id: Ident;

    fn id(&self) -> Self::Id;
    fn dependencies(&self) -> Vec<Dependency<Self::Id>>;
    fn to_node<R: Resolvable>(&self) -> Node<R>;  // TODO: pass a graph / id + dep_ids
}
