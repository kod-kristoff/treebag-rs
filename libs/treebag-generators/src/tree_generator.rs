//package generators;
//
//import terms.*;
use std::rc::Rc;
use treebag_terms::TermHandle;
//import parsers.*;
//import gui.*;
//import util.*;
//
///** A (possibly non-deterministic) device that produces terms as output.
///  * Subclasses should implement some commands by which the behaviour can
///  * be controlled interactively.
/// */
pub trait TreeGenerator {
    //public abstract class treeGenerator extends parsable implements reactive {
    //
    ///** Returns the last term generated. */
    fn current_term(&self) -> TermHandle;
    //
    // /** The default implementation of the method <code>commands</code>
    //  * yields the empty list.
    //  */
    //  public list commands() { return new list(); }
    //
    // /** The default implementation of the method <code>execute</code>
    //  * does nothing.
    //  */
    //  public void execute(String command) { }
    //
    // /** The default <code>requestsExit</code> method returns <code>false</code>.
    //  */
    //  public boolean requestsExit(String command) { return false; }
    //
    //
}
