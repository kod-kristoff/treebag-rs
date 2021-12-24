use crate::{Symbol, SymbolHandle};

/// A signature, i.e., a set of symbols.
pub trait Signature {

    /// Find out whether this signature contains a given symbol.
    fn contains(&self, symbol: &dyn Symbol) -> bool;
  
    // /// Yields the string representation of the signature.
    // fn to_string(&self) -> String;
  
    // Parse the signature from a stream, like in <code>parsable</code>.
    // @see parsable
    // @exception ParseException if an error occurs
    // pub fn parse(stream: ) throws ParseException;

}
