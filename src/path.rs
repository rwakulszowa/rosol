use node::resolvable::Resolvable;
use node::Node;
use package::ident::Ident;

#[derive(Clone, Debug, PartialEq)]
pub struct Path<'a, T: 'a + Resolvable> {
    pub nodes: Vec<&'a Node<T>>
}

impl<'a, T: 'a + Resolvable> Path<'a, T> {
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

    pub fn idents(&self) -> Vec<T::Id> {
        self.nodes.iter().map(|x| x.id.clone()).collect()
    }

    pub fn conflict(&self) -> bool {
        T::Id::are_conflicting(&self.idents())
    }
}

#[cfg(test)]
mod tests {
    use node::resolved::Resolved;
    use package::ident::SimpleUnique;
    use super::*;

    type N = Node<MockResolvable>;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    struct MockResolvable {}

    impl Resolvable for MockResolvable {
        type Id = SimpleUnique;

        fn resolve<'a>(&'a self, path: Path<'a, Self>) -> Resolved<'a, Self> {
            Resolved::success(path)
        }
    }

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

        let a: N = Node {
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

        let a: N = Node {
            id: id_a.clone(),
            dependency: None
        };

        let path = Path::new(vec![&a]);

        assert!(path.unique(&a));

        let path = path.append(&a);
        assert!(!path.unique(&a));
    }

    #[test]
    fn idents() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };

        let a: N = Node {
            id: id_a.clone(),
            dependency: None
        };

        let b: N = Node {
            id: id_b.clone(),
            dependency: None
        };

        let path = Path::new(vec![&a, &b]);

        assert_eq!(
            path.idents(),
            vec![id_a, id_b]);
    }

    #[test]
    fn conflict() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };

        let a: N = Node {
            id: id_a.clone(),
            dependency: None
        };

        let b: N = Node {
            id: id_b.clone(),
            dependency: None
        };

        let path = Path::new(vec![&a, &b]);
        assert!(!path.conflict());

        let path = path.append(&a);
        assert!(path.conflict());
    }
}
