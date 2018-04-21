use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

use package::ident::Ident;
use path::Path;
use super::resolved::Resolved;

pub trait Resolvable: Sized + Clone + Eq + Hash + PartialEq {
    type Id: Ident;

    fn resolve<'a>(&'a self, path: Path<'a, Self>) -> Resolved<'a, Self>;
}
