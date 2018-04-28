extern crate rosol;

mod resolvable_impl;

use rosol::node::Node;
use rosol::node::cause::Cause;
use rosol::node::resolved::Resolved;
use rosol::package::ident::SimpleUnique;
use rosol::path::Path;

use self::resolvable_impl as resolvable;

#[test]
fn single_node() {
    type R = resolvable::Simple;

    let id = SimpleUnique { id: "id1" };

    let s: Node<R> = Node {
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
    type R = resolvable::Simple;

    let id = SimpleUnique { id: "id1" };

    let s: Node<R> = Node {
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
    type R = resolvable::Simple;

    let id = SimpleUnique { id: "id1" };

    let mut circular: Node<R> = Node {
        id: id.clone(),
        dependency: None
    };

    let dep = resolvable::Simple::new(&circular);
    circular.dependency = Some(dep);

    let path = Path::new(vec![]);
    let res = circular.solve(path);

    let expected = Resolved::failure(
        Cause::empty());

    assert_eq!(res, expected);
}

#[test]
fn recursive() {
    // c -> b -> a => c -> b -> a
    type R = resolvable::Simple;

    let nodes: Vec<Node<R>> =
        vec!["a", "b", "c"]
        .iter()
        .map(|id| SimpleUnique { id })
        .map(|id| Node { id, dependency: None })
        .collect();

    let (a, mut b, mut c) = (nodes[0].clone(), nodes[1].clone(), nodes[2].clone());

    b.dependency = Some(R::new(&a));
    c.dependency = Some(R::new(&b));

    let path = Path::new(vec![]);
    let res = c.solve(path);

    let expected = Resolved::new(
        vec![Path::new(vec![&c, &b, &a])],
        Cause::empty());

    assert_eq!(res, expected);
}

#[test]
fn with_any() {
    // a -> (b | c -> a) => a -> b

    type R = resolvable::Any;

    let nodes: Vec<Node<R>> =
        vec!["a", "b", "c"]
        .iter()
        .map(|id| SimpleUnique { id })
        .map(|id| Node { id, dependency: None })
        .collect();

    let (mut a, b, mut c) = (nodes[0].clone(), nodes[1].clone(), nodes[2].clone());

    // NOTE: the dependency should be a RefCell, but this works well enough for a test
    // `c.dependency` is not exactly the same as `a` here (it is cloned before setting
    // `a.dependency`)
    c.dependency = Some(
        R::new(vec![&a]));

    a.dependency = Some(
        R::new(vec![&b, &c]));

    let path = Path::new(vec![]);
    let res = a.solve(path);

    let expected = Resolved::new(
        vec![Path::new(vec![&a, &b])],
        Cause::empty());

    assert_eq!(res, expected);
}

#[test]
fn with_orand() {
    // a -> (b | c -> d) => a -> c -> d
    type R = resolvable::OrAnd;
    type And = resolvable::AndDependency;
    type Or = resolvable::OrDependency;

    let nodes: Vec<Node<R>> =
        vec!["a", "b", "c", "d"]
        .iter()
        .map(|id| SimpleUnique { id })
        .map(|id| Node { id, dependency: None })
        .collect();

    let (mut a, mut b, mut c, d) = (nodes[0].clone(), nodes[1].clone(), nodes[2].clone(), nodes[3].clone());

    // Set the dependencies starting from the bottom
    // d -> None

    // c -> d
    c.dependency = Some(
        R::new(
            Or::single(
                And::single(d.clone()))));

    // b -> c & d
    b.dependency = Some(
        R::new(
            Or::single(
                And::new(
                    vec![c.clone(), d.clone()]))));

    // a -> b | c
    a.dependency = Some(
        R::new(
            Or::new(
                vec![
                    And::single(b.clone()),
                    And::single(c.clone())])));


    let path = Path::new(vec![]);
    let res = a.solve(path);

    let expected = Resolved::new(
        vec![Path::new(vec![&a, &c, &d])],
        Cause::empty());

    assert_eq!(res, expected);
}
