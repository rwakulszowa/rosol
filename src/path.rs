use package::ident::Ident;

#[derive(Debug)]
pub struct Path<T> {
    pub nodes: Vec<T>
}

impl<T> Path<T> {
    pub fn new(nodes: Vec<T>) -> Self {
        Path { nodes: nodes }
    }

    pub fn append(&mut self, node: T) {
        self.nodes.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn vec_equal<T: Eq>(a: Vec<T>, b: Vec<T>) -> bool {
        (a.len() == b.len()) &&
        a.iter()
            .zip(b.iter())
            .all(|(a, b)| a == b)
    }

    #[test]
    fn appends() {
        let mut path = Path::new(vec![]);
        path.append(1);

        assert!(
            vec_equal(
                vec![1],
                path.nodes));
    }
}
