use std::collections::HashSet;

use super::resolvable::Resolvable;
use super::Node;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cause<'a, T: 'a + Resolvable> {
    pub nodes: HashSet<&'a Node<T>>
}

impl<'a, T: 'a + Resolvable> Cause<'a, T> {
    pub fn new(nodes: HashSet<&'a Node<T>>) -> Self {
        Cause { nodes }
    }

    pub fn empty() -> Self {
        Self::new(HashSet::new())
    }

    pub fn from(node: &'a Node<T>) -> Self {
        let mut nodes = HashSet::new();
        nodes.insert(node);
        Self::new(nodes)
    }

    pub fn add(mut self, node: &'a Node<T>) -> Self {
        if !self.has(node) {
            self.nodes.insert(node);
        }
        self
    }

    pub fn above(mut self, node: &'a Node<T>) -> Self {
        if self.has(node) {
            self.nodes.remove(node);
        }
        self
    }

    fn has(&self, node: &Node<T>) -> bool {
        self.nodes.contains(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use package::ident::SimpleUnique;
    use node::resolvable::Simple;

    #[test]
    fn adds() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };

        let a = Node {
            id: id_a.clone(),
            dependency: None
        };

        let a_dep = Simple::new(&a);

        let b = Node {
            id: id_b.clone(),
            dependency: Some(a_dep)
        };

        let mut nodes = HashSet::new();

        nodes.insert(&a);
        let cause_a = Cause::new(nodes.clone());

        nodes.insert(&b);
        let cause_ab = Cause::new(nodes.clone());

        assert_eq!(
            cause_a.clone().add(&a),
            cause_a);

        assert_eq!(
            cause_a.clone().add(&b),
            cause_ab);
    }

    #[test]
    fn infers_above() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };

        let a = Node {
            id: id_a.clone(),
            dependency: None
        };

        let a_dep = Simple::new(&a);

        let b = Node {
            id: id_b.clone(),
            dependency: Some(a_dep)
        };

        let mut nodes = HashSet::new();

        let cause_empty = Cause::new(nodes.clone());

        nodes.insert(&a);
        let cause_a = Cause::new(nodes.clone());

        nodes.insert(&b);
        let cause_ab = Cause::new(nodes.clone());

        assert_eq!(
            cause_a.clone().above(&a),
            cause_empty);

        assert_eq!(
            cause_ab.clone().above(&b),
            cause_a);
    }
}
