mod cause;
mod dependency;
mod resolved;
mod solvability;

use package::ident::{Ident, SimpleUnique};
use path::Path;
use self::cause::Cause;
use self::dependency::Dependency;
use self::resolved::Resolved;
use self::solvability::Solvability;

/// Node of a DFS-resolvable tree.
///
/// It is assumed that all `Node` instances are created before
/// invoking `resolve`
pub trait Node: Sized {
    type Id: Ident;

    fn resolve<'a>(&'a self, mut path: Path<&'a Simple<Self::Id>>) -> Resolved<'a, Self>;
}

#[derive(Debug)]
pub struct Simple<Id: Ident> {
    pub id: Id,
    pub dependency: Dependency
}

impl<Id: Ident> Node for Simple<Id> {
    type Id = Id;

    fn resolve<'a>(&'a self, mut path: Path<&'a Simple<Self::Id>>) -> Resolved<'a, Self> {
        path.append(self);

        let solvability = Self::solvability(
            &Self::idents(&path));

        let paths = match solvability {
            Solvability::Ok => vec![path],
            Solvability::Conflict => vec![],
            _ => unreachable!()
        };

        let cause = Cause::new(vec![]);

        Resolved {
           paths,
           cause
        }
    }
}

impl<Id: Ident> Simple<Id> {
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
            id,
            dependency: Dependency {}
        };

        let res = s.resolve(Path::new(vec![]));

        assert!(!res.paths.is_empty());
    }

    #[test]
    fn resolves_into_empty_when_duplicated() {
        let id = SimpleUnique { id: "id1" };

        let s = Simple {
            id,
            dependency: Dependency {}
        };

        let path = Path::new(vec![&s]);
        let res = s.resolve(path);

        assert!(res.paths.is_empty());
    }
}
