mod cause;
mod dependency;
mod resolved;
mod solvability;

use package::ident::{Ident, SimpleUnique};
use path::Path;
use self::cause::Cause;
use self::dependency::Dependency;
use self::resolved::Resolved;

trait Node<'a, T: Ident + 'a> {
    fn resolve(&self, path: Path) -> Resolved<'a, T>;
}

#[derive(Debug)]
pub struct Simple<'a, T: Ident + 'a> {
    pub id: &'a T,
    pub dependency: Dependency
}

impl<'a, T: Ident + 'a> Node<'a, T> for Simple<'a, T> {
    fn resolve(&self, path: Path) -> Resolved<'a, T> {
        Resolved {
           paths: vec![],
           cause: Cause::new(vec![])
        }
    }
}

impl<'a, T: Ident + 'a> Simple<'a, T> {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let id = SimpleUnique {};

        let s = Simple {
            id: &id,
            dependency: Dependency {}
        };

        let res = s.resolve(Path {});

        assert!(res.paths.is_empty());
    }
}
