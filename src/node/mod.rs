mod cause;
mod dependency;
mod resolved;
mod solvability;

trait Node {
}

#[derive(Debug)]
pub struct Simple {
}

impl Simple {
}

impl Node for Simple {
}
