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

    pub fn chain(elements: Vec<Self>) -> Self {
        Self::new(
            elements
                .into_iter()
                .flat_map(|path| path.nodes)
                .collect())
    }

    pub fn suffix(&self, prefix: &Self) -> Self {
        let long = &self.nodes;
        let short = &prefix.nodes;

        assert!({
            let long_prefix: Vec<&&Node<T>> = long
                .iter()
                .take(short.len())
                .collect();
            let short_: Vec<&&Node<T>> = short
                .iter()
                .collect();
            long_prefix == short_
        });

        Self::new(
            long
                .into_iter()
                .skip(short.len())
                .map(|&n| n)
                .collect())
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
    fn chains() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };
        let id_c = SimpleUnique { id: "c" };

        let a: N = Node {
            id: id_a.clone(),
            dependency: None
        };
        let b: N = Node {
            id: id_b.clone(),
            dependency: None
        };
        let c: N = Node {
            id: id_c.clone(),
            dependency: None
        };

        let path_a = Path::new(vec![&a]);
        let path_bc = Path::new(vec![&b, &c]);
        let path_abc = Path::new(vec![&a, &b, &c]);

        assert_eq!(
            Path::chain(vec![path_a, path_bc]),
            path_abc);
    }

    #[test]
    fn suffix() {
        let id_a = SimpleUnique { id: "a" };
        let id_b = SimpleUnique { id: "b" };
        let id_c = SimpleUnique { id: "c" };

        let a: N = Node {
            id: id_a.clone(),
            dependency: None
        };
        let b: N = Node {
            id: id_b.clone(),
            dependency: None
        };
        let c: N = Node {
            id: id_c.clone(),
            dependency: None
        };

        let path_a = Path::new(vec![&a]);
        let path_bc = Path::new(vec![&b, &c]);
        let path_abc = Path::new(vec![&a, &b, &c]);

        assert_eq!(
            path_abc.suffix(&path_a),
            path_bc);
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
