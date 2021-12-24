//package generators;
use crate::*; 
//
//import terms.*;
use treebag_terms::Term;
//
///** A (possibly non-deterministic) device that input terms into output terms.
pub trait TreeTransducer: TreeGenerator {
//
    ///** Compute the output for a given term as input.
    ///  * The resulting term can also be queried using <code>currentTerm()</code>.
    ///  * It can be assumed that the argument term is not a <code>null</code> reference.
    fn apply(term: Term) -> Term;
//  
}
