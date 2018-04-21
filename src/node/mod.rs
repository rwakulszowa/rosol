pub mod cause;
pub mod resolvable;
pub mod resolved;
pub mod solvability;

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

        let solvability = Self::solvability(&path);
        println!("Node:solve {:?} ", self.id);

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

    pub fn solvability(path: &Path<R>) -> Solvability {
        if path.conflict() {
            Solvability::Conflict
        } else {
            Solvability::Ok
        }
    }
}
