pub mod resolvable;
mod cause;
mod resolved;
mod solvability;

use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

use package::ident::{Ident, SimpleUnique};
use path::Path;
use self::cause::Cause;
use self::resolved::Resolved;
use self::resolvable::Resolvable;
use self::solvability::Solvability;

/// Node of a DFS-traversable tree.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Node<R: Resolvable> {
    pub id: R::Id,
    pub dependency: Option<R>
}

impl <R: Resolvable> Node<R> {
    pub fn solve<'a>(&'a self, path: Path<'a, R>) -> Resolved<'a, R> {
        let path = path.append(self);

        let solvability = self.solvability(&path);

        match solvability {
            Solvability::Ok => {
                match self.dependency {
                    Some(ref dependency) => {
                        let subresult = dependency.resolve(path);
                        Resolved::new(
                            subresult.paths,
                            subresult.cause.above(self))
                    },
                    None => Resolved::success(path)
                }
            },
            Solvability::Conflict => {
                let cause = match path.unique(&self) {
                    true => Cause::empty(),
                    false => Cause::from(self)
                };
                Resolved::failure(cause)
            },
            _ => unreachable!()
        }
    }

    fn solvability(&self, path: &Path<R>) -> Solvability {
        if path.conflict() {
            Solvability::Conflict
        } else {
            Solvability::Ok
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::resolvable::Simple;

    type N = Node<Simple<SimpleUnique>>;

    #[test]
    fn solves() {
        let id = SimpleUnique { id: "id1" };

        let s: N = Node {
            id: id.clone(),
            dependency: None
        };

        let res = s.solve(Path::new(vec![]));

        let expected = Resolved::new(
            vec![Path::new(vec![&s])],
            Cause::empty());

        assert_eq!(res, expected);
    }

    #[test]
    fn resolves_into_empty_when_duplicated() {
        let id = SimpleUnique { id: "id1" };

        let s: N = Node {
            id: id.clone(),
            dependency: None
        };

        let path = Path::new(vec![&s]);
        let res = s.solve(path);

        let expected = Resolved::failure(
            Cause::from(&s));

        assert_eq!(res, expected);
    }

    #[test]
    fn resolves_with_dependencies() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };

        let a: N = Node {
            id: id_a.clone(),
            dependency: None
        };

        let a_dep = Simple::new(&a);

        let b: N = Node {
            id: id_b.clone(),
            dependency: Some(a_dep)
        };

        let path = Path::new(vec![]);
        let res = b.solve(path);

        let expected = Resolved::new(
            vec![Path::new(vec![&b, &a])],
            Cause::empty());

        assert_eq!(res, expected);
    }

    #[test]
    fn cleans_internal_causes() {
        let id = SimpleUnique { id: "id1" };

        let mut circular: N = Node {
            id: id.clone(),
            dependency: None
        };

        let dep = Simple::new(&circular);
        circular.dependency = Some(dep);

        let path = Path::new(vec![]);
        let res = circular.solve(path);

        let expected = Resolved::failure(
            Cause::empty());

        assert_eq!(res, expected);
    }
}
