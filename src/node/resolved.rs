use package::ident::Ident;
use path::Path;
use super::cause::Cause;
use super::Simple as SNode;

#[derive(Debug)]
pub struct Resolved<'a, T: 'a + Ident> {
    pub paths: Vec<Path<SNode<'a, T>>>,
    pub cause: Cause<'a, T>
}

impl<'a, T: 'a + Ident> Resolved<'a, T> {
}
