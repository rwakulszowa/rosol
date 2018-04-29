extern crate petgraph;

mod cache;
pub mod node;
pub mod package;
pub mod path;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
