use path::Path;
use super::cause::Cause;
use super::resolvable::Resolvable;

#[derive(Debug, PartialEq)]
pub struct Resolved<'a, T: 'a + Resolvable> {
    pub paths: Vec<Path<'a, T>>,
    pub cause: Cause<'a, T>
}

impl<'a, T: 'a + Resolvable> Resolved<'a, T> {
    pub fn new(paths: Vec<Path<'a, T>>, cause: Cause<'a, T>) -> Self {
        Resolved {
            paths,
            cause
        }
    }

    pub fn success(path: Path<'a, T>) -> Self {
        Self::new(vec![path], Cause::empty())
    }

    pub fn failure(cause: Cause<'a, T>) -> Self {
        Self::new(vec![], cause)
    }

    pub fn is_success(&self) -> bool {
        self.paths.len() != 0
    }

    pub fn merge(elements: Vec<Resolved<'a, T>>) -> Self {
        elements
            .into_iter()
            .fold(
                Resolved::new(vec![], Cause::empty()),
                Self::merge_two)
    }

    fn merge_two(left: Self, right: Self) -> Self {
        let Self { mut paths, cause } = left;
        let Self { paths: rpaths, cause: rcause } = right;
        paths.extend(rpaths);
        Self::new(paths, cause.merge(rcause))
    }
}
