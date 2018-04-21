use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub struct Path<T: PartialEq> {
    pub nodes: Vec<T>
}

impl<T: PartialEq> Path<T> {
    pub fn new(nodes: Vec<T>) -> Self {
        Path { nodes: nodes }
    }

    pub fn append(&mut self, node: T) {
        self.nodes.push(node);
    }

    pub fn unique(&self, el: &T) -> bool {
       self.nodes.iter().filter(|&x| x == el).count() == 1
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

    #[test]
    fn unique() {
        let mut path = Path::new(vec![1, 2]);

        assert!(path.unique(&1));

        path.append(1);
        assert!(!path.unique(&1));
    }
}
