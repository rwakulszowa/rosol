extern crate rosol;

mod resolvable_impl;

use rosol::node::Node;
use rosol::node::cause::Cause;
use rosol::node::resolved::Resolved;
use rosol::package::ident::SimpleUnique;
use rosol::path::Path;

use self::resolvable_impl as resolvable;

type N = Node<resolvable::Simple<SimpleUnique>>;
type NAny = Node<resolvable::Any<SimpleUnique>>;

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
    let id_a = SimpleUnique { id: "a" };
    let id_b = SimpleUnique { id: "b" };
    let id_c = SimpleUnique { id: "c" };

    let a: N = Node {
        id: id_a.clone(),
        dependency: None
    };

    let a_dep = resolvable::Simple::new(&a);

    let b: N = Node {
        id: id_b.clone(),
        dependency: Some(a_dep)
    };

    let b_dep = resolvable::Simple::new(&b);

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

#[test]
fn with_any() {
    // a -> (b | c -> a) => a -> b  // TODO: magic syntax / macro
    let id_a = SimpleUnique { id: "a" };
    let id_b = SimpleUnique { id: "b" };
    let id_c = SimpleUnique { id: "c" };

    let mut a: NAny = Node {
        id: id_a.clone(),
        dependency: None
    };

    let b: NAny = Node {
        id: id_b.clone(),
        dependency: None
    };

    let c: NAny = Node {
        id: id_c.clone(),
        dependency: Some(
            resolvable::Any::new(vec![&a]))
    };

    a.dependency = Some(
        resolvable::Any::new(vec![&b, &c]));

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
