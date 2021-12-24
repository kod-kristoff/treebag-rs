mod signature;
pub mod signatures;
mod symbol;
mod term;

pub use signature::*;
pub use symbol::*;
pub use term::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
