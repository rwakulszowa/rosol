extern crate rosol;

mod resolvable_impl;

use rosol::node::Node;
use rosol::node::cause::Cause;
use rosol::node::resolved::Resolved;
use rosol::package::ident::SimpleUnique;
use rosol::path::Path;

use self::resolvable_impl::Simple as SimpleResolvable;

type N = Node<SimpleResolvable<SimpleUnique>>;

#[test]
fn single_node() {
    let id = SimpleUnique { id: "id1" };

    let s: N = Node {
        id: id.clone(),
        dependency: None
    };

    let res = s.solve(Path::new(vec![]));

    let expected = Resolved::new(
        vec![Path::new(vec![&s])],
        Cause::empty());

    assert_eq!(res, expected);
}

#[test]
fn duplicate() {
    let id = SimpleUnique { id: "id1" };

    let s: N = Node {
        id: id.clone(),
        dependency: None
    };

    let path = Path::new(vec![&s]);
    let res = s.solve(path);

    let expected = Resolved::failure(
        Cause::from(&s));

    assert_eq!(res, expected);
}

#[test]
fn circular() {
    let id = SimpleUnique { id: "id1" };

    let mut circular: N = Node {
        id: id.clone(),
        dependency: None
    };

    let dep = SimpleResolvable::new(&circular);
    circular.dependency = Some(dep);

    let path = Path::new(vec![]);
    let res = circular.solve(path);

    let expected = Resolved::failure(
        Cause::empty());

    assert_eq!(res, expected);
}

#[test]
fn recursive() {
    let id_a = SimpleUnique { id: "a" };
    let id_b = SimpleUnique { id: "b" };
    let id_c = SimpleUnique { id: "c" };

    let a: N = Node {
        id: id_a.clone(),
        dependency: None
    };

    let a_dep = SimpleResolvable::new(&a);

    let b: N = Node {
        id: id_b.clone(),
        dependency: Some(a_dep)
    };

    let b_dep = SimpleResolvable::new(&b);

    let c: N = Node {
        id: id_c.clone(),
        dependency: Some(b_dep)
    };

    let path = Path::new(vec![]);
    let res = c.solve(path);

    let expected = Resolved::new(
        vec![Path::new(vec![&c, &b, &a])],
        Cause::empty());

    assert_eq!(res, expected);
}
