use super::Simple as SNode;
use package::ident::Ident;

#[derive(Debug)]
pub struct Cause<'a, T: 'a + Ident> {
    pub nodes: Vec<&'a SNode<'a, T>>
}

impl<'a, T: 'a + Ident> Cause<'a, T> {
    pub fn new(nodes: Vec<&'a SNode<'a, T>>) -> Self {
        Cause {
            nodes: nodes
        }
    }
}
