use std::cmp::PartialEq;
use node::Node;
use package::ident::Ident;

#[derive(Debug, PartialEq)]
pub struct Path<'a, T: 'a + Ident> {
    pub nodes: Vec<&'a Node<T>>
}

impl<'a, T: 'a + Ident> Path<'a, T> {
    pub fn new(nodes: Vec<&'a Node<T>>) -> Self {
        Path { nodes: nodes }
    }

    pub fn append(mut self, node: &'a Node<T>) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn unique(&self, el: &Node<T>) -> bool {
       self.nodes.iter().filter(|&&x| x == el).count() == 1
    }

    pub fn idents(&self) -> Vec<T> {
        self.nodes.iter().map(|x| x.id.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use package::ident::SimpleUnique;

    pub fn vec_equal<T: Eq>(a: Vec<T>, b: Vec<T>) -> bool {
        (a.len() == b.len()) &&
        a.iter()
            .zip(b.iter())
            .all(|(a, b)| a == b)
    }

    #[test]
    fn appends() {
        let path = Path::new(vec![]);

        let id_a = SimpleUnique { id: "a" };

        let a = Node {
            id: id_a.clone(),
            dependency: None
        };

        let path = path.append(&a);

        assert!(
            vec_equal(
                path.nodes,
                vec![&a]));
    }

    #[test]
    fn unique() {
        let id_a = SimpleUnique { id: "a" };

        let a = Node {
            id: id_a.clone(),
            dependency: None
        };

        let mut path = Path::new(vec![&a]);

        assert!(path.unique(&a));

        path = path.append(&a);
        assert!(!path.unique(&a));
    }

    #[test]
    fn idents() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };

        let a = Node {
            id: id_a.clone(),
            dependency: None
        };

        let b = Node {
            id: id_b.clone(),
            dependency: None
        };

        let path = Path::new(vec![&a, &b]);

        assert_eq!(
            path.idents(),
            vec![id_a, id_b]);
    }
}
