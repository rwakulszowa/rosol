use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use package::dependency::Dependency;
use package::package::Package;
use package::rawnode::RawNode;

#[derive(Debug)]
pub struct Repository<P: Package> {
    packages: HashMap<P::Id, P>
}

impl<P: Package> Repository<P> {
    pub fn new<I>(packages: I) -> Self
    where I: IntoIterator<Item = P> {
        let packages = packages
            .into_iter()
            .map(|p| (p.id(), p))
            .collect();

        Repository {
            packages
        }
    }

    pub fn add(&mut self, pkg: P) {
        self.packages.insert(pkg.id(), pkg);
    }

    pub fn build_graph(&self, root_id: &P::Id) -> Rc<RefCell<RawNode<P::Id>>> {
        let mut nodes = HashMap::new();

        let root = self.get(root_id).unwrap();
        let mut pending = vec![root];

        // Aggregate all used packages
        while !pending.is_empty() {
            let pkg = pending.pop().unwrap();

            // Check if visited to avoid infinite recursion
            if !nodes.contains_key(&pkg) {
                let raw = RawNode::new(pkg.id().clone());
                nodes.insert(pkg, raw);

                pending.extend(
                    pkg
                        .dependencies()
                        .into_iter()
                        .flat_map(|dep| self.dependency_matches(&dep))
                    );
            }
        }

        // Add edges
        let edges = nodes
            .keys()
            .flat_map(|source| {
                source
                    .dependencies()
                    .into_iter()
                    .flat_map(|dep| self.dependency_matches(&dep))
                    .map(move |dep| (source, dep))
            });

        for (source, target) in edges {
            let a = nodes.get(source).unwrap();
            let b = nodes.get(target).unwrap();
            a.borrow_mut().add_dependency(b);
        }
        
        nodes.get(&root).unwrap().clone()
    }

    fn get(&self, id: &P::Id) -> Option<&P> {
        self.packages.get(id)
    }

    fn dependency_matches(&self, dep: &Dependency<P::Id>) -> Vec<&P> {
        dep
            .ids()
            .iter()
            .filter_map(|id| self.get(id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use node::Node;
    use node::resolvable::Resolvable;
    use package::ident::SimpleUnique;
    use super::*;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    struct MockPackage {
        id: SimpleUnique,
        dependencies: Vec<Dependency<SimpleUnique>>
    }

    impl Package for MockPackage {
        type Id = SimpleUnique;

        fn id(&self) -> Self::Id {
            self.id.clone()
        }

        fn dependencies(&self) -> Vec<Dependency<Self::Id>> {
            self.dependencies.clone().into_iter().collect()
        }

        fn to_node<R: Resolvable>(&self) -> Node<R> {
            unimplemented!();
        }
    }

    #[test]
    fn finds_dependency_matches() {

        let ids: Vec<_> =
            vec!["a", "b", "c", "d"]
            .iter()
            .map(|id| SimpleUnique { id })
            .collect();

        let repo_ids = ids[..3].to_vec();
        let dep_ids = ids[2..].to_vec();
        let overlapping = ids[2..3].to_vec();

        let dep = Dependency::Choice(dep_ids);

        let repo_pkgs: Vec<_> = repo_ids
            .into_iter()
            .map(|id| MockPackage { id, dependencies: vec![] })
            .collect();

        let overlapping_pkgs: Vec<_> = overlapping
            .into_iter()
            .map(|id| MockPackage { id, dependencies: vec![] })
            .collect();

        let repo = Repository::new(repo_pkgs);

        assert_eq!(
            repo.dependency_matches(&dep),
            overlapping_pkgs.iter().collect::<Vec<_>>());
    }

    #[test]
    fn builds_graph() {

        let ids: Vec<_> =
            vec!["a", "b", "c"]
            .iter()
            .map(|id| SimpleUnique { id })
            .collect();

        let (a, b, c) = (ids[0].clone(), ids[1].clone(), ids[2].clone());
        let root = a.clone();

        let a_ = MockPackage {
            id: a.clone(),
            dependencies: vec![
                Dependency::Choice(vec![b.clone()]),
                Dependency::Choice(vec![c.clone()])]
        };

        let b_ = MockPackage {
            id: b.clone(),
            dependencies: vec![
                Dependency::Choice(vec![c.clone()])]
        };

        let c_ = MockPackage {
            id: c.clone(),
            dependencies: vec![]
        };

        let raws: Vec<_> = ids
            .iter()
            .map(|id| RawNode::new(id.clone()))
            .collect();

        let (a_r, b_r, c_r) = (raws[0].clone(), raws[1].clone(), raws[2].clone());
        a_r.borrow_mut().add_dependency(&b_r);
        a_r.borrow_mut().add_dependency(&c_r);
        b_r.borrow_mut().add_dependency(&c_r);

        let repo = Repository::new(vec![a_, b_, c_]);
        let root_node = repo.build_graph(&root);

        assert_eq!(
            root_node,
            a_r);
    }
}
