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
trait Node<'a, Id: Ident + 'a> {
    fn resolve(&'a self, path: Path<&'a Simple<'a, Id>>) -> Resolved<'a, Id>;
}

#[derive(Debug)]
pub struct Simple<'a, Id: Ident + 'a> {
    pub id: &'a Id,
    pub dependency: Dependency
}

impl<'a, Id: Ident + 'a> Node<'a, Id> for Simple<'a, Id> {
    fn resolve(&'a self, mut path: Path<&'a Simple<'a, Id>>) -> Resolved<'a, Id> {
        path.append(&self);

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

impl<'a, Id: Ident + 'a> Simple<'a, Id> {
    fn solvability(idents: &Vec<&Id>) -> Solvability {
        let conflict = Id::are_conflicting(idents);

        if conflict {
            Solvability::Conflict
        } else {
            Solvability::Ok
        }
    }

    fn idents<'b>(path: &'b Path<&'b Self>) -> Vec<&'b Id> {
        path.nodes.iter().map(|n| n.id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves() {
        let id = SimpleUnique { id: "id1" };

        let s = Simple {
            id: &id,
            dependency: Dependency {}
        };

        let res = s.resolve(Path::new(vec![]));

        assert!(!res.paths.is_empty());
    }

    #[test]
    fn resolves_into_empty_when_duplicated() {
        let id = SimpleUnique { id: "id1" };

        let s = Simple {
            id: &id,
            dependency: Dependency {}
        };

        let path = Path::new(vec![&s]);
        let res = s.resolve(path);

        assert!(res.paths.is_empty());
    }
}
