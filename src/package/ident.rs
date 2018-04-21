use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Ident: Clone + Debug + Eq + Hash + PartialEq {
    fn are_conflicting(instances: &Vec<Self>) -> bool;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SimpleUnique {
    pub id: &'static str
}

impl SimpleUnique {
    pub fn new(id: &'static str) -> Self {
        SimpleUnique { id: id }
    }
}

impl Ident for SimpleUnique {
    fn are_conflicting(instances: &Vec<Self>) -> bool {
        let mut found = HashSet::new();

        !instances
            .into_iter()
            .all(move |x| found.insert(x))  // will return False if alrady present
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_conflicts() {
        let a = SimpleUnique::new("A");
        let b = SimpleUnique::new("B");

        assert!(!SimpleUnique::are_conflicting(&vec![a.clone(), b.clone()]));
        assert!(SimpleUnique::are_conflicting(&vec![a.clone(), b.clone(), a.clone()]));
    }
}
