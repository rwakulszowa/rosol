mod cause;
mod dependency;
mod resolved;
mod solvability;

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use package::ident::{Ident, SimpleUnique};
use path::Path;
use self::cause::Cause;
use self::resolved::Resolved;
use self::solvability::Solvability;

/// Node of a DFS-resolvable tree.
///
/// It is assumed that all `Node` instances are created before
/// invoking `resolve`
pub trait Node: Sized {
    type Id: Ident + Eq + Hash;

    fn resolve<'a>(&'a self, mut path: Path<&'a Simple<Self::Id>>, nodes: &'a HashMap<Self::Id, Self>) -> Resolved<'a, Self>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Simple<Id: Ident + Eq + Hash> {
    pub id: Id,
    pub dependency: Option<Id>
}

impl<Id: Ident + Eq + Hash> Node for Simple<Id> {
    type Id = Id;

    fn resolve<'a>(&'a self, mut path: Path<&'a Simple<Self::Id>>, nodes: &'a HashMap<Self::Id, Self>) -> Resolved<'a, Self> {
        path.append(&self);

        let solvability = Self::solvability(
            &Self::idents(&path));

        match solvability {
            Solvability::Ok => {
                let dependency = self.dependency.as_ref().and_then(
                    |id| nodes.get(&id));

                match dependency {
                    Some(dependency) => {
                        let subresult = dependency.resolve(path, &nodes);

                        let cause = match subresult.is_success() {
                            true => Cause::empty(),
                            false => Cause::from(self)
                        };

                        Resolved::new(subresult.paths, cause)
                    },
                    None => Resolved::new(vec![path], Cause::empty())
                }
            },
            Solvability::Conflict =>
                Resolved::failure(Cause::from(self)),
            _ => unreachable!()
        }
    }
}

impl<Id: Ident + Eq + Hash> Simple<Id> {
    fn solvability(idents: &Vec<&Id>) -> Solvability {
        let conflict = Id::are_conflicting(idents);

        if conflict {
            Solvability::Conflict
        } else {
            Solvability::Ok
        }
    }

    fn idents<'b>(path: &Path<&'b Self>) -> Vec<&'b Id> {
        path.nodes.iter().map(|n| &n.id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves() {
        let id = SimpleUnique { id: "id1" };

        let s = Simple {
            id: id.clone(),
            dependency: None
        };

        let mut nodes = HashMap::new();
        nodes.insert(id.clone(), s.clone());

        let res = s.resolve(
            Path::new(vec![]),
            &nodes
        );

        assert!(!res.paths.is_empty());
    }

    #[test]
    fn resolves_into_empty_when_duplicated() {
        let id = SimpleUnique { id: "id1" };

        let s = Simple {
            id: id.clone(),
            dependency: None
        };

        let mut nodes = HashMap::new();
        nodes.insert(id.clone(), s.clone());

        let path = Path::new(vec![&s]);
        let res = s.resolve(path, &nodes);

        assert!(res.paths.is_empty());
    }

    #[test]
    fn resolves_with_dependencies() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };

        let a = Simple {
            id: id_a.clone(),
            dependency: None
        };

        let b = Simple {
            id: id_b.clone(),
            dependency: Some(id_a.clone())
        };

        let mut nodes = HashMap::new();
        nodes.insert(id_a.clone(), a.clone());
        nodes.insert(id_b.clone(), b.clone());

        let path = Path::new(vec![]);
        let res = b.resolve(path, &nodes);

        assert_eq!(
            res.paths,
            vec![Path::new(vec![&b, &a])]);
    }
}
