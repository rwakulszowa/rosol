use std::cell::RefCell;
use std::rc::Rc;

use package::ident::Ident;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawNode<Id: Ident> {
    pub id: Id,
    pub deps: Vec<Rc<RefCell<RawNode<Id>>>>
}

impl<Id: Ident> RawNode<Id> {
    pub fn new(id: Id) -> Rc<RefCell<Self>> {
        let raw = Self {
            id,
            deps: Vec::new()
        };
        Rc::new(RefCell::new(raw))
    }

    pub fn add_dependency(&mut self, other: &Rc<RefCell<Self>>) {
        self.deps.push(other.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use package::ident::SimpleUnique;

    #[test]
    fn adds_transitive_dependency() {
        let nodes: Vec<_> = 
            vec!["a", "b", "c"]
            .iter()
            .map(|id| RawNode::new(
                SimpleUnique::new(id)))
            .collect();

        let (a, b, c) = (nodes[0].clone(), nodes[1].clone(), nodes[2].clone());

        a.borrow_mut().add_dependency(&b);
        b.borrow_mut().add_dependency(&c);


        let a_deps: Vec<_> = a.borrow()
            .deps
            .iter()
            .flat_map(|n| {
                let mut ans = n.borrow().deps.clone();
                ans.push(n.clone());
                ans
            })
            .collect();

        assert!(a_deps.contains(&c));
    }
}
