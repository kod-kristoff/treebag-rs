use std::collections::HashMap;

use crate::{Signature, Symbol, SymbolHandle};

//package terms;
//
//import java.util.*;
//import parsers.*;
//
/// A finite signature.
/// Having different symbols with the same name is allowed.
///
pub struct FiniteSignature {
    symbols: HashMap<SymbolHandle, u32>,
    elem_count: u32,
}

// implements signature, Cloneable {
//
//  private Hashtable symbols = new Hashtable();
//  private int elemCount = 0;
impl FiniteSignature {
    pub fn new() -> Self {
        let symbols = HashMap::new();
        let elem_count = 0;
        Self {
            symbols,
            elem_count,
        }
    }
}

impl FiniteSignature {
// /** Adds a symbol to a signature.
//  */
//  public void addSymbol(symbol s) {
//    symbols.put(s, new Integer(elemCount++));
//  }
//  
// /** Removes a symbol from a signature.
//  */
//  public void removeSymbol(symbol s) {
//    symbols.remove(s);
//  }
//  
    /// The number of symbols in this signature./
    pub fn size(&self) -> usize {
        self.symbols.len()
    }
//  
//  public int indexOf(symbol sym) {
//    return ((Integer)symbols.get(sym)).intValue();
//  }
//  
    pub fn max_index(&self) -> u32 {
        self.elem_count
    }
//  
// /** Test whether another signature is disjoint with this one. */
//  public boolean disjointWith(signature sig) {
//    Enumeration enm = symbols.keys();
//    while (enm.hasMoreElements()) {
//      if (sig.contains((symbol)enm.nextElement())) return false;
//    }
//    return true;
//  }
//  
// /** Test whether this signature is a superset of another one. */
//  public boolean contains(finiteSignature sig) {
//    Enumeration enm = sig.elements();
//    while (enm.hasMoreElements()) {
//      if (!contains((symbol)enm.nextElement())) return false;
//    }
//    return true;
//  }
//  
// /** Add all symbols of another finite signature to this one. */
//  public void unionWith(finiteSignature sig) {
//    Enumeration enm = sig.elements();
//    while (enm.hasMoreElements()) addSymbol((symbol)enm.nextElement());
//  }
//  
// /** An enumeration of all the symbols in the signature. */
//  public Enumeration elements() { return symbols.keys(); }
//  
//  public String toString() {
//    StringBuffer result = new StringBuffer();
//    result.append("{ ");
//    Enumeration elem = elements();
//    if (elem.hasMoreElements()) {
//      symbol sym = (symbol)elem.nextElement();
//      result.append(nameParser.unparse(sym.toString()) + ":" + sym.rank());
//      while (elem.hasMoreElements()) {
//        sym = (symbol)elem.nextElement();
//        result.append(", " + nameParser.unparse(sym.toString()) + ":" + sym.rank());
//      }
//    }
//    result.append(" }");
//    return result.toString();
//  }
//
// /** Parse a signature definition.
//    @exception ParseException if an error occurs
//  */
//  public void parse(ASCII_CharStream stream) throws ParseException {
//    setParser parser = new setParser(stream);
//    parser.set(new cp(new symbolParser(stream)));
//  }
} 

impl Signature for FiniteSignature {
  
    fn contains(&self, symbol: &dyn Symbol) -> bool {
        // self.symbols.contains_key(symbol)
        unimplemented!()
    }

}
//  private class cp implements componentParser {
//    private symbolParser symbols;
//    public cp(symbolParser symbols) { this.symbols = symbols; }
//    public void component() throws ParseException { addSymbol(symbols.symbol()); }
//  }
//
//  public Object clone() {
//    finiteSignature result;
//    try { result = (finiteSignature)super.clone(); }
//    catch (CloneNotSupportedException e) // cannot happen (hopefully)
//      { throw new InternalError(e.toString()); }
//    result.symbols = (Hashtable)symbols.clone();
//    return result;
//  }
//
//}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let sig = FiniteSignature::new();

        assert_eq!(sig.size(), 0);
        assert_eq!(sig.max_index(), 0);
    }
}
