use std::collections::HashMap;
use petgraph::Graph;

use node::Node;
use node::resolvable::Resolvable;
use package::package::Package;

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

    pub fn build_graph<R: Resolvable>(&self, root_id: &P::Id) -> Graph<&P, ()> {
        let mut graph = Graph::<_, _>::new();
        let root = self.get(root_id);
        graph.add_node(root);
        // TODO
        graph
    }

    fn get(&self, id: &P::Id) -> &P {
        self.packages.get(id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use node::resolved::Resolved;
    use path::Path;
    use super::*;
    use package::dependency::Dependency;
    use package::ident::SimpleUnique;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    struct MockResolvable {}

    impl Resolvable for MockResolvable {
        type Id = SimpleUnique;

        fn resolve<'a>(&'a self, path: Path<'a, Self>) -> Resolved<'a, Self> {
            unimplemented!();
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
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
    fn builds_graph() {

        let ids: Vec<_> =
            vec!["a", "b", "c", "d"]
            .iter()
            .map(|id| SimpleUnique { id })
            .collect();

        let root = ids[0].clone();

        let pkgs: Vec<_> = ids
            .into_iter()
            .map(|id| MockPackage { id, dependencies: vec![] })
            .collect();

        let repo = Repository::new(pkgs);

        let graph = repo.build_graph::<MockResolvable>(&root);
    }
}
