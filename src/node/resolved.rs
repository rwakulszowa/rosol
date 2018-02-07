use package::ident::Ident;
use path::Path;
use super::cause::Cause;

#[derive(Debug)]
pub struct Resolved<'a, T: 'a + Ident> {
    pub paths: Vec<Path>,
    pub cause: Cause<'a, T>
}

impl<'a, T: 'a + Ident> Resolved<'a, T> {
}
