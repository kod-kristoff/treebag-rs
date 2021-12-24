mod tree_generator;
mod tree_grammar;
mod tree_transducer;

pub use tree_grammar::*;
pub use tree_generator::*;
pub use tree_transducer::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
