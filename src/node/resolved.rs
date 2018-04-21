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
}
