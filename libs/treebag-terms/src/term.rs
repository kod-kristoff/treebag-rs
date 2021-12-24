use crate::*;
use std::rc::Rc;
//
//import parsers.*;
//
///** A term over ranked symbols.
//  * @see symbol
//  */
pub struct Term {
    top: Rc<dyn Symbol>,
    subterm: Vec<Term>,
}


//public class term implements Cloneable {
//
//  private symbol top = null;
//  private term[] subterm = null;
//  
// /** Constructs an uninitialized <code>term</code> object (for use in
//  * connection with the <code>parse()</code> method).
//  */
//  public term() {
//    subterm = new term[0];
//  }
impl Term {
// /** Constructs a term consisting of a single root symbol (and still
//  * undefined subterms).
//  */
//  public term(symbol root) {
//    top     = root;
//    subterm = new term[root.rank()];
//  }
//  
// /** Replace the root symbol of the term with another one.
//  * Old subterms beyond the rank of the new symbol get lost; the others are
//  * retained. The new symbol must not be <code>null</code>!
//  */
//  public void relabel(symbol s) {
//    top = s;
//    term[] old = subterm;
//    subterm = new term[s.rank()];
//    System.arraycopy(old, 0, subterm, 0, Math.min(old.length, subterm.length));
//  }
//
// /** Set the i-th subterm of this term.
//  * @param i the number of the subterm to be set (numbering starts
//  *        with 0)
//  * @param sub the subterm
//  */
//  public void defineSubterm(int i, term sub) {
//    subterm[i] = sub;
//  }
//
    ///** Returns the outer most symbol of the term. */
    pub fn top_symbol(&self) -> Rc<dyn Symbol> {
        Rc::clone(&self.top)
    }
//  
// /** Returns the i-th subterm of the term.
//  * @param i the number of the requested subterm (0 denoting the first)
//  */
//  public term subterm(int i) {
//    return subterm[i];
//  }
//  
// /** Returns the depth of the term (= length of the longest path from the
//  * root to a leaf).
//  */
//  public int depth() {
//    if (top.rank() == 0) return 0;
//    int result = 0;
//    for (int i = 0; i < top.rank; i++) result = (int)Math.max(result, subterm[i].depth());
//    return result + 1;
//  }
//  
// /** Try to match this term against another one, i.e., to find a substitution for
//  * the variables in this term that turns it into the argument term.
//  * Upon success an array is returned that contains the subterms of the
//  * argument term that correspond to the variables in this term. Otherwise,
//  * <code>null</code> is returned.
//  */
//  public term[] match (term against) {
//    term[] result = new term[highestVariable()+1];
//    if (match(against, result)) return result;
//    else return null;
//  }
//  
//  private boolean match(term against, term[] storeIn) {
//    if (top instanceof variable) {
//      storeIn[((variable)top).index()] = against;
//      return true;
//    }
//    else {
//      if (!top.equals(against.top)) return false;
//      for (int i = 0; i < top.rank(); i++) {
//        if (!subterm[i].match(against.subterm[i], storeIn)) return false;
//      }
//      return true;
//    }
//  }
//
// /** Compute the highest index of a variable in this term.
//  * If it does not contain any variable, -1 is returned.
//  */
//  public int highestVariable() {
//    if (top instanceof variable) return ((variable)top).index();
//    int result = -1;
//    for (int i = 0; i < top.rank(); i++) {
//      result = Math.max(result, subterm[i].highestVariable());
//    }
//    return result;
//  }
//  
// /** Substitute terms for the variables in this term.
//  * This copies the involved terms.
//  */
//  public term substitute(term[] substitution) {
//    if (top instanceof variable) {
//      if (substitution.length > ((variable)top).index) {
//        return (term)substitution[((variable)top).index].clone();
//      }
//      else return new term(top);
//    }
//    else {
//      term result = new term(top);
//      for (int i = 0; i < top.rank(); i++) {
//        result.subterm[i] = subterm[i].substitute(substitution);
//      }
//      return result;
//    }
//  }
//    
//  public String toString() {
//    return termParser.toString(this);
//  }
//  
//  public void parse(ASCII_CharStream stream) throws ParseException {
//    termParser parser = new termParser(stream);
//    term t = parser.term();
//    top = t.top;
//    subterm = t.subterm;
//  }
//  
//  public Object clone() {
//    term result;
//    try { result = (term)super.clone(); }
//    catch (CloneNotSupportedException e) // cannot happen (hopefully)
//      { throw new InternalError(e.toString()); }
//    result.subterm = (term[])subterm.clone();
//    for (int i = 0; i < subterm.length; i++) {
//      if (subterm[i] != null) result.subterm[i] = (term)subterm[i].clone();
//    }
//    return result;
//  }
//  
}

// pub type TermHandle = std::rc::Rc<& dyn Term>;
