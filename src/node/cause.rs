use super::Node;
use package::ident::Ident;

#[derive(Debug)]
pub struct Cause<'a, T: 'a + Node> {
    pub nodes: Vec<&'a T>
}

impl<'a, T: 'a + Node> Cause<'a, T> {
    pub fn new(nodes: Vec<&'a T>) -> Self {
        Cause {
            nodes: nodes
        }
    }
}
