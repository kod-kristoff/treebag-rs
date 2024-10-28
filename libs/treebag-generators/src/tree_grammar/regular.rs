// package generators;
// 
// import java.util.*;
// import terms.*;
// import parsers.*;
// import util.*;
// 
// /** An implementation of regular tree grammars.
//   * After instantiation (using the <code>parse</code> method) an object of this class 
//   * can be used to enumerate the language generated by the defining regular tree
//   * grammar, or to generate terms in a random fashion.
//   * For this, the following commands can be used:
//   * <ul>
//   * <li> <code>random generation<code> and <code>enumeration<code> switch from
//   *        enumeration mode to generation mode, and vice versa,
//   * <li> in enumeration mode, <code>advance</code> leads to the next term in the enumeration,
//   * <li> in generation mode, <code>refine</code> applies randomly chosen rules to all the
//   *        nonterminals in the term,
//   * <li> <code>reset</code> returns to the first term in enumeration mode, whereas
//   *        in generation mode it returns to the initial nonterminal.
//   * </ul>
//   * In addition, one can choose between two ``sub-modes'' in enumeration mode. By default,
//   * only the generated terms are shown. The command <code>show derivation</code> turns
//   * to a mode where the current term is generated stepwise, providing the commands
//   * <ul>
//   * <li> <code>single step</code>, which results in a single step
//   *        (applied to the left-most nonterminal if it exists) and
//   * <li> <code>parallel step</code>, which results in a parallel step.
//   * </ul>
//   * The command <code>show results only</code> turns to the default enumeration mode, again.
//   * </ul>
//   */
// public class regularTreeGrammar extends treeEnumerator {
// 
// //====================================================================
// // First the 'interactive' part that deals with commands ...
// //====================================================================
//   private static String single = "derive stepwise";
//   private static String complete = "results only";
//   private static String singleStep = "single step";
//   private static String parallelStep = "parallel step";
//   private static String back = "back";
//   private static String[][] sCommands = {{ singleStep, parallelStep, back }, { complete }};
//   private static String[][] cCommands = {{ single }};
//   private Vector stepsDone = new Vector();
//   
//   public boolean isStepwise = false;
//   
// /** Yields the (names of the) commands described above. */
//   public list commands() {
//     list result = super.commands();
//     if (isEnumerate) {
//       if (isStepwise) for (int i = sCommands.length; i > 0;) result.tail().prepend(sCommands[--i]);
//       else for (int i = cCommands.length; i > 0;) result.tail().prepend(cCommands[--i]);
//     }
//     return result;
//   }
//   
//   public void execute(String com) {
//     if (single.equals(com)) {
//       isStepwise = true;
//       stepsDone.removeAllElements();
//       special(derTree);
//     }
//     else if (complete.equals(com)) {
//       isStepwise = false;
//       special(derTree);
//     }
//     else if (singleStep.equals(com)) {
//       singleStep();
//       stepsDone.addElement(singleStep);
//     }
//     else if (parallelStep.equals(com)) {
//       parallelStep();
//       stepsDone.addElement(parallelStep);
//     }
//     else if (back.equals(com) && isStepwise && isEnumerate) {
//       if (stepsDone.size() > 0) {
//         stepsDone.removeElementAt(stepsDone.size() - 1);
//         special(derTree);
//         for (int i = 0; i < stepsDone.size(); i++) {
//           if (stepsDone.elementAt(i) == singleStep) singleStep();
//           else parallelStep();
//         }
//       }
//     }
//     else {
//       super.execute(com);
//       derTree = super.currentTerm();
//       if (isEnumerate &&
//           (advance.equals(com) || reset.equals(com) || enumerate.equals(com))) {
//         special(derTree);
//         stepsDone.removeAllElements();
//       }
//     }
//     translate();
//   }  
// 
// //====================================================================
// // Now the actual implementation of regular tree grammars ...
// //====================================================================
// 
//   private Hashtable meaning;
//   private Hashtable sort;
//   private term currTerm; 
//   private term derTree;
//   
//   public term currentTerm() { return currTerm; }
//   
//   private void translate() {
//     if (derTree == null) currTerm = null;
//     else currTerm = translate(derTree);
//   }
//   
//   private term translate(term derTree) {
//     symbol top = derTree.topSymbol();
//     if (top instanceof specialSymbol) {
//       return new term((symbol)sort.get(top.toString()));
//     }
//     term rhs = (term)meaning.get(top.toString());
//     if (rhs == null) return derTree;
//     term[] sub = new term[top.rank()];
//     for (int i = 0; i < sub.length; i++) sub[i] = translate(derTree.subterm(i));
//     return compose(rhs, sub);
//   }
//   
//   private term compose(term t, term[] sub) {
//     symbol top = t.topSymbol();
//     if (top instanceof variable) return sub[((variable)top).index()];
//     term result = new term(top);
//     for (int i = 0; i < top.rank(); i++) result.defineSubterm(i, compose(t.subterm(i), sub));
//     return result;
//   }
//   
//   private void special(term t) {
//     if (t != null) {
//       symbol top = t.topSymbol();
//       if (isStepwise) t.relabel(new specialSymbol(top));
//       else if (top instanceof specialSymbol) t.relabel(new symbol(top.toString(), top.rank()));
//       for (int i = 0; i < top.rank(); i++) special(t.subterm(i));
//     }
//   }
//   
//   private boolean ready;
//   
//   private void singleStep() {
//     if (derTree == null) return;
//     ready = false;
//     singleStep(derTree);
//   }
//   
//   private void singleStep(term dt) {
//     symbol top = dt.topSymbol();
//     if (top instanceof specialSymbol) {
//       ready = true;
//       dt.relabel(new symbol(top.toString(), top.rank()));
//     }
//     else for (int i = 0; i < top.rank(); i++) {
//       singleStep(dt.subterm(i));
//       if (ready) break;
//     }
//   }
//   
//   private void parallelStep() { if (derTree != null) parallelStep(derTree); }
//   
//   private void parallelStep(term dt) {
//     symbol top = dt.topSymbol();
//     if (top instanceof specialSymbol) dt.relabel(new symbol(top.toString(), top.rank()));
//     else for (int i = 0; i < top.rank(); i++) parallelStep(dt.subterm(i));
//   }
//   
// /** Initialize this <code>regularTreeGrammar</code> by reading its definition from a stream.
//   * The syntax is defined by the class <code>regularTreeGrammarParser</code>.
//   * @see regularTreeGrammarParser
//   * @exception ParseException if an error occurs
//   */
//   public void parse(ASCII_CharStream stream) throws ParseException {
//     regularTreeGrammarParser parser = new regularTreeGrammarParser(stream);
//     parser.regularTreeGrammar(this);
//     meaning = parser.meaning;
//     sort = parser.sort;
//     execute(advance);
//   }
//   
//   private static class specialSymbol extends symbol {
//     public specialSymbol(symbol s) { super(s.toString(), s.rank()); }
//   }
//   
// }
// 