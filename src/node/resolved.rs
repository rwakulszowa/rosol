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
}
