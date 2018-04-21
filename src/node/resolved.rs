use package::ident::Ident;
use path::Path;
use super::cause::Cause;
use super::Node;

#[derive(Debug)]
pub struct Resolved<'a, T: 'a + Node> {
    pub paths: Vec<Path<&'a T>>,
    pub cause: Cause<'a, T>
}

impl<'a, T: Node> Resolved<'a, T> {
    pub fn new(paths: Vec<Path<&'a T>>, cause: Cause<'a, T>) -> Self {
        Resolved {
            paths,
            cause
        }
    }

    pub fn failure(cause: Cause<'a, T>) -> Self {
        Self::new(vec![], cause)
    }

    pub fn is_success(&self) -> bool {
        self.paths.len() != 0
    }
}
