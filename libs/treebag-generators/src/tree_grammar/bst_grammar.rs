// package generators;
// 
// import java.io.*;
// import java.util.*;
// import parsers.*;
// import terms.*;
// import util.*;
// 
// /** An implementation of BST Grammar
//   * (Branching Syncronized Tree Grammar).
//   * Parts of the implementation are located
//   * in BSTGrammar.java and SyncedRule.java
//   */
// public class BSTGrammar extends treeGrammar {
// 
//   //A BST grammar is translated internally to a regular
//   //tree grammar and a series of tree transducers.
//   regularTreeGrammar gram;
//   tdTransducer trans[];
// 
//   //In order to, as an extra feature, display synchronization info,
//   //two extra components are required.
//   tdTransducer lastTransWhenUsingYield;
//   private YIELDTransduction yieldTransducer;
// 
//   //This string is added as suffix to the nonterminals,
//   //when a BST is transformed so that nonterminals and
//   //terminals becomes disjoint. Note that the empty string is allowed.
//   private static String nontermSuffix = "";
// //  private static String nontermSuffix = "_nonterm";
// 
//   //Can be set to true for debugging purposes. Then output tree
//   //will have all synctrees and final tree as subtrees.
//   //The output must be directed to a free term algebra if using this.
//   private static boolean debugGiveSyncTreesAsOutputToo = false;
// 
//   //If true, the rule similar to T->T<0> will be added
//   //whenever a rule with lefthandside T is missing in a subtable.
//   //This flag is set during parsing.
//   private boolean useImplicitRule = false;
// 
//   //If set to TYPE_PLAIN, full synctrees will be created, which is
//   //very slow and gives lots of undefined trees.
//   //If set to TYPE_NORMAL many of the unreachable subtrees in the synctrees will be
//   //replaced with bottom symbols. The components rougly keeps their size but
//   //the running time in most cases is greatly improved.
//   //This flag is set during parsing.
//   //If set to TYPE_EXTENDED translation is made in the 'extended' way,
//   //creating transducers that may be exponentially large
//   //with respect to output tree size.
//   //At the other hand sync trees will go from exponential
//   //size to polynomial size in some grammars.
//   //User should try for theirself whether this option
//   //works better off or on.
//   private int translationType;// = BSTGrammarParser.TYPE_NORMAL;
// 
// //=============================================================================
// // First the 'interactive' part that deals with commands ...
// //=============================================================================
// 
//   //useYieldTransducer is set to true when synchronization info
//   //is to be shown. If a free term algenra is used, the sync info
//   //will be shown as subtrees to nonterminals when doing
//   //stepwise derivations. This flag is set by GUI.
//   private boolean useYieldTransducer = false;
// 
//   private static String enumerate = "table enumeration";
//   private static String advance = "advance";
//   private static String single = "derive stepwise";
//   private static String complete = "results only";
//   private static String step = "derivation step";
//   private static String random = "random tables";
//   private static String refine = "refine";
//   private static String back = "back";
//   private static String reset = "reset";
//   private static String[][] eCommands = {{ advance, reset }, {random}};
//   private static String[][] rCommands = {{ refine, back, reset }, {enumerate}};
//   private static String[][] sCommands = {{ step, back }, { complete }};
//   private static String[][] cCommands = {{ single }};
//   private static String showSync = "show sync info"; // changed to lowercase. FD
//   private static String hideSync = "hide sync info"; // changed to lowercase. FD
// 
//   // Two new methods introduced and used below. FD
//   private String levelCommand(int level) {
//     if (level == bstNValue - 1) return "choose new rules";
//     if (regulation == null && nondisjoint) return "choose new supertables at depth " + (level+1);
//     else return "choose new supertables at depth " + (level+2);
//   }
// 
//   private int extractLevel(String com) {
//     for (int level = 0; level < bstNValue; level++) {
//       if (levelCommand(level).equals(com)) return level;
//     }
//     return -1;
//   }
// 
//   public list commands() {
//     list result = super.commands();
//     if (gram.isEnumerate) {
//       for (int i = 0; i < eCommands.length; i++) result.append(eCommands[i]);
//       if (gram.isStepwise) for (int i = sCommands.length; i > 0;) result.tail().prepend(sCommands[--i]);
//       else for (int i = cCommands.length; i > 0;) result.tail().prepend(cCommands[--i]);
//     }
//     else  for (int i = 0; i < rCommands.length; i++) result.append(rCommands[i]);
// 
//     for(int i=0; i<bstNValue; i++) // turned trans.length into bstNValue to be more consistent. FD
//     {
//       // condition below changed by FD: Do not care if the transducer is deterministic unless it is
//       // the one choosing rules.
//       if  (i<trans.length - 1 || !trans[i].isDeterministic())
//       {
//         String[] tCommandsN = { levelCommand(i) };
//         result.append(tCommandsN);
// 
//       }
//     }
//     if(!useYieldTransducer)
//     {
//       String[] yCommand = { showSync };
//       result.append(yCommand);
//     }
//     else
//     {
//       String[] yCommand = { hideSync };
//       result.append(yCommand);
//     }
//     return result;
//   }
// 
//   public void execute(String com) {
//     recompute = true;
//     if (enumerate.equals(com)) gram.execute("enumeration");
//     else if (advance.equals(com))
//     {
//       gram.execute("advance");
// 
//       //Without this first step, the init is not generated, when using
//       //stepwise derivation. Then the first resulting BST-tree would be
//       //undefined.
//       gram.execute("single step");
//     }
//     else if (single.equals(com)) { gram.execute("derive stepwise"); gram.execute("single step"); }
//     else if (complete.equals(com)) gram.execute("results only");
//     else if (step.equals(com)) gram.execute("single step");
//     else if (random.equals(com)) {
//       gram.execute("random generation");
//       gram.execute("back");
//       checkAdvance();
//     }
//     else if (refine.equals(com)) gram.execute("refine");
//     else if (back.equals(com)) {
//       gram.execute("back");
//       checkAdvance();
//     }
//     else if (reset.equals(com)) {
//       gram.execute("reset");
//       if (!gram.isEnumerate || gram.isStepwise) checkAdvance();
//     }
//     else if(showSync.equals(com)||hideSync.equals(com))
//     {
//       useYieldTransducer = !useYieldTransducer;
//       //The last transducer have to be rerun, since we have switched
//       //what transducer that is last transducer.
//       //New random seed is used which may not be the best way to handle it.
//       int levelToRecomputeFrom = bstNValue;
//       for(int i=levelToRecomputeFrom; i<trans.length; i++)
//       {
//         trans[i].execute("new random seed");
//       }
//       lastTransWhenUsingYield.execute("new random seed");
//     }
//     else {
//       //Used specifies through GUI how many levels to recompute
//       int level = extractLevel(com);
//       if(level != -1)
//       {
//         try {
//         for(int i=level; i<trans.length; i++)
//         {
//           trans[i].execute("new random seed");
//         }
//         lastTransWhenUsingYield.execute("new random seed");
//         }catch(Exception e)
//         {
//           throw new RuntimeException("Could not execute transducers. "
//             + "Probably a bug in the BST implementation\n" + e.toString());
//         }
//       }
//       else super.execute(com);
//     }
//   }
// 
//   private void checkAdvance() {
//     term t = gram.currentTerm();
//     if (t != null && !t.topSymbol().equals(init)) {
//       if (!gram.isEnumerate) gram.execute("refine");
//       else gram.execute("single step");
//     }
//   }
// 
//   public boolean requestsExit(String com) {
//     return com.equals(advance) || com.equals(refine) || com.equals(reset);
//   }
// 
// //=============================================================================
// // Now the actual implementation of BST grammars ...
// //=============================================================================
// 
//   //Set of terminals for the input BST grammar
//   private finiteSignature bstTermSig;
// 
//   //Set of nonterminals for the input BST grammar
//   private fixedRankSignature bstNontermSig;
// 
//   //Set of synchronization symbols for the input BST grammar.
//   //This will be a set of intergers.
//   private fixedRankSignature bstSyncSymbolSig;
// 
//   //Set of nonterminals for the output regular tree grammar
//   private fixedRankSignature regTreeGrammarNontermSig =
//     new fixedRankSignature(0);
// 
//   //axiom of the input BST
//   private term axiom;
// 
//   //Dessa är klassattribut som inte fanns i ET0L men som behövs i BST:
//   private SuperTable superTables;
//   private int bstDValue; //Number of syncsymbols, ie size of bstSyncSymbolSig
// 
//   //Depth if BST grammar. This equals the nesting level of the supertables
//   //all rules are found at, as well as the number of synsymbols on each
//   //nonterminal in righthandsides.
//   private int bstNValue;
// 
//   //These three are used when a regular expression, restricting table
//   //choices, is specified in the BST-file.
//   private Vector regularRules = new Vector();
//   private Vector chainRules = new Vector();
//   private rexp regulation = null;
// 
//   //true if some terminal and nonterminal of the BST has the same name.
//   boolean nondisjoint;
// 
//   //Below are some names on new symbols added to the output grammar
//   //and transducers. Hopefully the names wont clash with userspecified
//   //names in BST-file.
//   private symbol bottom = new symbol("bot", 0);
//   private String initName = "init";
//   private symbol init = new symbol(initName, 1);
//   private String qname = "q__";
//   private String Sname = "S__";
//   private String S0name = "S0__";
//   private String newAxiomName = "NEW_AXIOM";
//   private String newAxiomTableName = "TBL_axiom_table_";
// 
//   //If this is null, axiom is symbol. Otherwise this will point to table
//   //containing axiom-rule. This is used when ensuring that axiom table only
//   //occurs directly below init in synctrees.
//   private SuperTable axiomTable = null;
// 
//   private term current = null;
//   private boolean recompute = true;
// 
//   public synchronized term currentTerm() {
//     term allSyncTrees[] = new term[trans.length];  //for debugging purposes
//     if (recompute) {
// 
//       try {
//         current = gram.currentTerm();
// 
//         if(!useYieldTransducer)
//         {
//           for(int i=0; i<trans.length; i++)
//           {
//             allSyncTrees[i] = current;
//             current = trans[i].apply(current);
//           }
//         }
//         else
//         {
//           for(int i=0; i<trans.length-1; i++)
//           {
//             allSyncTrees[i] = current;
//             current = trans[i].apply(current);
//           }
//           current = lastTransWhenUsingYield.apply(current);
//           current = yieldTransducer.apply(current);
//         }
//       }
//       catch (ExitException e) { return current = null; }
//       recompute = false;
//     }
//     if(debugGiveSyncTreesAsOutputToo)//true = debug mode
//     {
//         term oldcurrent = current;
//         symbol sym = new symbol("Debug - All trees", trans.length+1);
//         current = new term(sym);
//     	int i;
//         for(i=0; i<trans.length; i++)
//     	{
//     	  current.defineSubterm(i, allSyncTrees[i]);
//     	}
//         current.defineSubterm(i, oldcurrent);
//     }
//     return current;
//   }
// 
// /** Initialize this <code>BSTGrammar</code> by reading its definition from a stream.
//   * The syntax is defined by the class <code>BSTGrammarParser</code>.
//   * Before the grammar can be used, <code>translate</code> must be called with a
//   * filename (which is used to store the regular tree grammar and the top-down tree
//   * transducer into which the grammar is translated).
//   * @see BSTGrammarParser
//   * @exception ParseException or RuntimeException if an error occurs
//   */
//   public void parse(ASCII_CharStream stream) throws ParseException {
//     BSTGrammarParser parser = new BSTGrammarParser(stream);
//     parser.BSTGrammar();
//     bstTermSig = parser.sig;
//     bstNontermSig = parser.nonterminals;
//     bstSyncSymbolSig = parser.syncSymbols;
// 
//     this.axiom = parser.axiom;
//     regulation = parser.regulation;
// 
//     superTables = parser.tables;
//     bstDValue = parser.syncSymbols.size();
//     bstNValue = parser.depth;
// 
//     useImplicitRule = parser.implicitRules;
//     findUniqueNames();
// 
//     //If something goes wrong, try moving this below creation of intermediate tds
//     if(useImplicitRule)
//       superTables.addImplicitRules(bstNontermSig, bstDValue);
// 
//     //If axiom is a nonsinglenode tree or a single terminal, we make a new
//     //nonterminal and let it be the axiom. Then we add a new table whose only 
//     //rule is to replace this nonterminal with the old axiom.
//     if(axiom.topSymbol().rank()>0 || bstTermSig.contains(axiom.topSymbol()) )
//     {
//       //Replace axiom with new nonterminal.
//       symbol newAxiom = new synchronizedSymbol(newAxiomName, 0);
//       SyncedRule axiomRule = new SyncedRule(newAxiom, axiom);
//       bstNontermSig.addSymbol(newAxiom);
//       axiom = new term(newAxiom);
//       //Add table whose only rule has as lhs the new nonterminal, 
//       //ans as rhs the old axiom.
//       if(bstNValue==0)
//       {
//         //special case for n=0
//         superTables.addRule(axiomRule);
//       }
//       else//n!=0
//       {
//         axiomTable = new SuperTable();
//         axiomTable.setName(newAxiomTableName);
//         axiomTable.addRule(axiomRule);
//         //The hierarchy of new subtables gets names differing only by 
//         //a number in the end.
//         for(int i=1; i<bstNValue; i++)
//         {
//           SuperTable t = new SuperTable();
//           t.setName(newAxiomTableName+"_"+i);
//           t.addSubTable(axiomTable);
//           axiomTable = t;
//         }
//         superTables.addSubTable(axiomTable);
//       }//n!=0
//       if(regulation != null)
//       {
//       	int indexOfAxiomTable = superTables.getNumTables()-1;
//         regulation = rexp.conc(rexp.num(indexOfAxiomTable+1),regulation);
//       }
//     }
// 
//     //Check and handle nondisjoint terminal and nonterminal sets.
//     try{
// 
//       //First we create the set containing the terminals for which there 
//       //are nonterminals with the same name.
//       finiteSignature bothNontermAndTermSymbols = new finiteSignature();
//       nondisjoint = false;
//       Enumeration enum2 = bstTermSig.elements();
//       while(enum2.hasMoreElements())
//       {
//         symbol terminal = (symbol)enum2.nextElement();
//         if(bstNontermSig.contains(new symbol(terminal.toString(), 0)))
//         {
//           nondisjoint = true;
//           bothNontermAndTermSymbols.addSymbol(terminal);
//         }
//       }
//       /*
//       if(writeDebugFile)
//       {
//         out.write((nondisjoint?"\r\nNondisjoint term/nonterm. Changing!\r\n":"\r\nDisjoint term/nonterm. Not changing...\r\n"));
//       }
//       */
//       if(nondisjoint)
//       {
//         if(bstNValue==0)
//         {
//           throw new ParseException("Currently no support for zero depth "
//            + "grammar with nondisjoin nonterminals and terminals.");
//         }
//         SuperTable oldTopTable = superTables;
// 
//         if(regulation == null)
//         {
//           int padSymbol = 0;
//           superTables = increaseDepthWithOneByPuttingAllInSame(
//                           superTables, padSymbol);
//           bstNValue++;
//         }
//         else
//         {
//           throw new ParseException("Currently no support for both regulation "
//              + "and nondisjoint nonterminal and terminal sets is implemented");
//         }
// 
//         //Create the new table that only can be used in last step to remove
//         //all syncinfo from nonterminals, making them terminals
//         {
//           {
//             SuperTable removeSyncinfoTable = new SuperTable();
//             String removeSyncTableName = "TBL_REMOVE_SYNCINFO";
//             removeSyncinfoTable.setName(removeSyncTableName);
// 
//             Enumeration enum3 = bothNontermAndTermSymbols.elements();
//             while(enum3.hasMoreElements())
//             {
//               symbol s = (symbol)enum3.nextElement();
//               /*
//                * if one wants to replace nonterminal with new nonterminal
//                */
//               symbol s_ = s;
//               if(nontermSuffix.equals("") == false)
//               {
//                 s_ = new synchronizedSymbol(s.toString()+nontermSuffix, 1);
//                 bstNontermSig.removeSymbol(s);
//                 bstNontermSig.addSymbol(s_);
//                 oldTopTable.relabelSymbol(s, s_); //replace nonterminal in all rules
//                 if(axiom.topSymbol().equals(s))
//                   axiom.relabel(s_);
//               }
//               removeSyncinfoTable.addRule(new SyncedRule(s_, new term(s)));
//             }
//             for(int i=1; i<bstNValue; i++)
//             {
//               SuperTable t = new SuperTable();
//               t.setName(removeSyncTableName+"_"+i);
//               t.addSubTable(removeSyncinfoTable);
//               removeSyncinfoTable = t;
//             }
//             superTables.addSubTable(removeSyncinfoTable);
//           }
//         }
//         if(regulation != null)
//         {
//           int indexOfSyncTable = superTables.getNumTables()-1;
//           regulation = rexp.conc(regulation, rexp.num(indexOfSyncTable+1));
//         }
//         else
//         {
//           if(axiomTable!=null)  //if axiom table is used
//           {
//             //Then lift out axiom table from level 2 to level 1
//             //Now there are 3 tables inside the top table:
//             // - table with all original BST tables
//             // - table with rules to remove syncinfo, making nonterms termsl,
//             //   only usable in last step.
//             // - table with axiom table, only usable in first step.
//             oldTopTable.removeSubTable(axiomTable);
//             SuperTable axiomTableParent = new SuperTable();
//             axiomTableParent.setName(newAxiomTableName+"_"+bstNValue);
//             axiomTableParent.addSubTable(axiomTable);
//             superTables.addSubTable(axiomTableParent);
//             axiomTable = axiomTableParent;
//           }
//         }
// 
//       }
//     }catch(Exception e2)
//     {
//       e2.printStackTrace();
//       throw new RuntimeException(
//         "Error in implementation. Unhandled exption during parsing.");
//     }
// 
//     translationType = parser.translationType;
//     //parser.TYPE_PLAIN or parser.TYPE_NORMAL or parser.TYPE_EXTENDED
//     
//     //execution continues after the GUI has called translate() ...
// 
//   }//method parse(ASCII_CharStream stream)
// 
//   /** Increases depth of BST by one. A new supertable at level 0 is created, and
//    *  the old level-0-table, that should be sent as parameter, is placed inside this
//    *  table. All sync-symbol-tuples in all rules are then increased by one symbol,
//    *  so that the tuples length agrees with the new BST depth.
//    */
//   private SuperTable increaseDepthWithOneByPuttingAllInSame(SuperTable oldTopTable, int padSyncSymbol)
//   {
//     SuperTable newTopTable = new SuperTable();
//     newTopTable.setName(oldTopTable+"_NEW");
//     newTopTable.addSubTable(oldTopTable);
// 
//     //recursive method that adds a 0 to all syncsymbol tuples in all rules.
//     oldTopTable.increaseLengthOfSyncSymbolTuplesWithOne(padSyncSymbol);
//     return newTopTable;
//   }
// 
//   private void findUniqueNames()
//   {
//     //find a name for q0 that does not interfer with any user defined BST-symbols
//     for(int i=0; i<10000/*so program cant hang*/; i++)
//     {
//       qname = "q" + i;  //tries q0,q1,q2 etc
//       //0 since nonterminals have rank 0
//       if(bstNontermSig.contains(new symbol(qname,0)))
//         continue;
//       //1 since, only terminals with rank 1 can cause name clash with states
//       if(bstTermSig.contains(new symbol(qname,1)))
//         continue;
//       break;  //no name clash
//     }
//     //same for S0name
//     for(int i=0; i<10000/*so program cant hang*/; i++)
//     {
//       S0name = "S" + i;  //tries S0,S1,S2 etc
//       //should be enough to check terminal set, since states has rank 1 and nonterms rank 0
//       if(bstTermSig.contains(new symbol(S0name,1)))
//         continue;
//       break;  //no name clash
//     }
//     //same for Sname
//     for(int i=0; i<10000/*so program cant hang*/; i++)
//     {
//       Sname = "Start" + i;  //tries Start0,Start1,Start2 etc
//       //should be enough to check terminal set, since states has rank 1 and nonterms rank 0
//       if(bstTermSig.contains(new symbol(Sname,1)))
//         continue;
//       break;  //no name clash
//     }
//     for(int i=0; i<10000/*so program cant hang*/; i++)
//     {
//        if(i!=0)
//           newAxiomName = newAxiomName + i;
//       //0 since nonterminals have rank 0
//       if(bstNontermSig.contains(new symbol(newAxiomName,0)))
//         continue;
//       if(bstTermSig.contains(new symbol(newAxiomName,0)))
//         continue;//break;
//       break;  //no name clash
//     }
//   }
// 
//   /** The method that manages translation of the parsed BST grammar
//     * to regular tree grammar and transducers which are written to
//     * files. The code for translating the particular components are
//     * in methods called by this method.
//     * After creating these files they are read and parsed.
//     * @param fileName All outputfiles will have this filename plus an
//     * extension consisting of a dot followed by a letter or number.
//     */
//   public void translate(String fileName) throws ParseException {
//     //Regular tree grammar is written to file with extension ".1"
//     File file1 = new File(fileName + ".1");
//     boolean generated = false;
//     try{
//       //Special case for n=0.
//       //Then we should have one tree grammar and no transducers.
//       if(bstNValue==0){
//         new RegularTreeGrammarGenerator().generateSolitaryTreeGrammar(openOutFile(file1));
//       }
//       else {
//         //Extended mode is currently not implemented for the regular
//         //tree grammar, only for tree transducers.
//         boolean useNormalMode = true;
//         if(translationType == BSTGrammarParser.TYPE_PLAIN)
//         {
//           useNormalMode = false;
//         }
// 
//         //If table choices on top level are restricted by regular
//         //expression the regular tree grammar is build another way.
//         if (regulation == null) {
//           //restricting regular expression not used
//           new RegularTreeGrammarGenerator().generateDefaultGrammar(
//               openOutFile(file1), useNormalMode);
//         }
//         else
//         {
//           //restricting regular expression is used
//           new RegularTreeGrammarGenerator().generateGrammar(
//               openOutFile(file1), useNormalMode, regulation);
//         }
// 
//         //Generate the n-1 intermediate td tree transducers
//         IntermediateTransducerGenerator generator = new IntermediateTransducerGenerator();
//         for(int i=1; i<bstNValue; i++)
//         {
//           File file2 = new File(fileName + "."+(i+1));
//           if(translationType == BSTGrammarParser.TYPE_PLAIN)
//           {
//             generator.generateIntermediateTransducerUsingPlainTranslation(
//               openOutFile(file2), i);
//           }
//           if(translationType == BSTGrammarParser.TYPE_NORMAL)
//           {
//             generator.generateIntermediateTransducerUsingNormalTranslation(
//               openOutFile(file2), i);
//           }
//           if(translationType == BSTGrammarParser.TYPE_EXTENDED)
//           {
//             generator.generateIntermediateTransducerUsingExtendedTranslation(
//               openOutFile(file2), i);
//           }
//         }
// 
//         //Generate the final transducer.
//         File file3 = new File(fileName + "." + (1+bstNValue) );
//         new FinalTransducerGenerator().generateFinalTransducer(openOutFile(file3), false);
// 
//         //Generate an alternative final transducer, that adds
//         //the full synchroization strings to the leaves out the output.
//         new FinalTransducerGenerator().generateFinalTransducer(openOutFile(new File(fileName + ".Y")), true);
// 
//       }
//       generated = true;
//     }
//     catch (IOException e) {
//       for (int i=0; i<bstNValue+1; i++)
//       {
//         File file = new File(fileName + "."+(i+1));
//         if (!file.exists()) throw new ParseException("Could not write to output file:\n" + e.getMessage());
//         if (!(new File(fileName + ".Y")).exists()) throw new ParseException("Could not write to output file:\n" + e.getMessage());
//       }
//     }
//     catch (java.security.AccessControlException e) {
//       for (int i=0; i<bstNValue+1; i++)
//       {
//         File file = new File(fileName + "."+(i+1));
//         if (!file.exists()) throw new ParseException("Could not write to output file:\n" + e.getMessage());
//         if (!(new File(fileName + ".Y")).exists()) throw new ParseException("Could not write to output file:\n" + e.getMessage());
//       }
//     }
//     catch (Exception e)
//     {
//       throw new ParseException("Error. \n" + e.getMessage());
//     }
// 
//     try {
//       objectParser parser = new objectParser(new ASCII_CharStream(new FileInputStream(file1),1,1));
//       gram = (regularTreeGrammar)parser.parse();
// 
//       trans = new tdTransducer[bstNValue];
// 
//       for(int i=0; i<bstNValue; i++)
//       {
//         String ithFileName = fileName + "." + (i+2);
//         parser = new objectParser(new ASCII_CharStream(new FileInputStream(ithFileName),1,1));
//         trans[i] = (tdTransducer)parser.parse();
//       }
//       parser = new objectParser(new ASCII_CharStream(new FileInputStream(fileName+".Y"),1,1));
//       lastTransWhenUsingYield = (tdTransducer)parser.parse();
//       yieldTransducer = new YIELDTransduction();
//     }
//     catch (ParseException e) {
//       if (generated) {
//         throw new ParseException("Could not parse generated file:\n" +
//           e.getMessage() +
//           "\n[This should not happen. There must be one of those " +
//           "small insects in the implementation 8-( ]");
//       }
//       else {
//         throw new ParseException("Could not parse previously existing file:\n" +
//           e.getMessage() +
//           "\nPlease make sure that the file is writable to enable " +
//           "generation of new file");
//       }
//     }
//     catch (FileNotFoundException e) {
//       throw new ParseException("Could not open file:\n" + e.getMessage());
//     }
//   }
// 
//   private FileWriter openOutFile(File file) throws IOException {
//     if (file.exists()) file.delete();
//     FileWriter result = new FileWriter(file);
//     return result;
//   }
// 
// //=============================================================================
// // Generation of the regular tree grammar
// //=============================================================================
//   class RegularTreeGrammarGenerator{
//     //Generate regular tree grammar for the case in which regular expression
//     //is not used.
//     private void generateDefaultGrammar(FileWriter out,
//         boolean useNormalMode) throws IOException {
//       int d = bstDValue;
//       regTreeGrammarNontermSig.addSymbol(Sname);
//       regTreeGrammarNontermSig.addSymbol(S0name);
// 
//       out.write("generators.stubbornRegularTreeGrammar:  ");
//       out.write("  (\n");
// 
//       //Nonterminals of the regular tree grammar, ie {S,S0}
//       out.write("    " + regTreeGrammarNontermSig + ",\n");
// 
//       //Output signature of the reg.tree.grammar.
//       //Contains all table names on highest level.
//       finiteSignature t = new finiteSignature();
//   
//       t.addSymbol(bottom);
//       t.addSymbol(init);
// 
//       for(int i=0; i<superTables.getNumTables(); i++)
//       {
//          SuperTable st = superTables.getSubTable(i);
//          t.addSymbol(new symbol(st.getName(),d));
//       }
//       out.write("    " + t + ",\n");
//       out.write("    {\n");
//   
//       tableEnumerationRules(out,useNormalMode,Sname,S0name);
//   
//       out.write("    },\n");
//       out.write("    "+Sname+"\n");
//       out.write("  )");
//       out.close();
//     }
// 
//     /** Finds subtable with a certain name under the given table.
//       * The method is only looking one level below.
//       */
//     private SuperTable findTableWithName(SuperTable topTable, String name)
//     {
//       for(int i=0; i<topTable.getNumTables(); i++)
//       {
//         SuperTable st = topTable.getSubTable(i);
//         if(st.getName().equals(name))
//         {
//         	return st;
//         }
//       }
//       return null;
//     }
// 
//     /** Checking if a certain subtree in a table's rules' righthandsides
//       * can ever be reached.
//       * The actual functionality is lotaced in the SuperTable class.
//       */
//     private boolean isSynctreeSubtreeUseful(SuperTable table,
//       int subtreeNumber, int levelToCheck, int dValue)
//     {
//       Vector usedBranches = table.getUsedSyncBranchNumbersForSyncLevel(dValue, levelToCheck);
//       return usedBranches.contains(new Integer(subtreeNumber));
//     }
// 
//     //Metoden som skapar trädgrammatiken som ger första synkroniseringsträdet
//     private void tableEnumerationRules(FileWriter out, boolean useNormalMode, 
//         String Sname, String S0name/*, String SAxiomname*/) throws IOException {
// 
//       int d = bstDValue;
// 
//       String initRule;
//       if(axiomTable==null)
//       {
//         initRule = Sname+" -> " + initName + "[" + S0name + "]";
//         out.write("      " + initRule );
//         out.write(",\n      ");
//         //If axiom-table is used, dont create this init rule. Instead create a different init rule below.
//       }
// 
//       for (int t=0; t < superTables.getNumTables(); t++)
//       {
//         SuperTable st = superTables.getSubTable(t);
// 
//         //We use getUsedSyncBranchNumbers to chose where to put bottom symbols.
//         //Most important with this is that putting bottom symbols instead of branches
//         //is that it often reduces times complexity from exponential to polynomial.
//         //It has no effect on the output language, since those branches would not have
//         //been used anyway.
//         //This way terminating tables will automatically become leaves in the sync. tree (or rather not really leaves,
//         //but if we delete all bottom-symbols they would become leaves).
//         //However, it is not sufficient for the case when terminals and nonterminals are not disjoint sets,
//         //and that special case is handled later in code by adding more rules that creates leaves.
//         Vector usedBranches = st.getUsedSyncBranchNumbersForSyncLevel(d,1);  //one since sync1
// 
//         //build string of d words that can be bottom or nonterminal depending on sync-strings.
//         //Example: d=2 and <01><00> are the only strings in any subtable, subsubtable etc, of this table.
//         //but since this is only sync1, we only look at first digit, ie both are 0.
//         //only 0 means only first of the two branches are useful. so string will be "S0__,bot"
//         //All terminating tables will result in "bot,bot", since no sync.symbols at all can be found.
//         //If useNormalMode==false it will instead write "S0__,S0__"
//         String s = "";
//         for(int j=0; j<d; j++)
//         {
//           if(useNormalMode==false || usedBranches.contains(new Integer(j)))
//           {
//             s = s + S0name;
//           }
//           else
//           {
//             s = s + bottom;
//           }
//           if(j<d-1)
//           {
//             s = s + ",";
//           }
//         }
//   
//         //Create the rule S -> init[axiomtable[x1,x1,...]]
//         //Actually axiom does not NEED to be handled this way, it's just
//         //for efficiency reasons. Axiom could be handled just like any other
//         //table, but that would give many undefined outputs.
//         if(axiomTable!=null && st==axiomTable)
//         {
//           out.write( Sname+" -> " + initName + "[" + st.getName() + "[" + s + "]]" );
//         }
//         else
//         {
//         	out.write(S0name + " -> " + st.getName() + "[" + s + "] weight " + st.getWeight() );
//         }
//   
//         //What if no table contains only rules with no nonterminals in rhs?
//         //Then the rules listed above will make the sync tree never terminate, because
//         //the code above thinks that there are no terminal tables.
//         //What we need is alternative rules that puts only bottom-symbols beneath such
//         //tables. This way terminal-and-nonterminal-tables can occur both as nodes and leaves
//         //in sync. trees, whereas nonterminal-only-tables can only occur as nodes.
//         //Still, this could created undefined results, if some subtables are terminating
//         //and some are not (isTerminable is true if some subtable is terminating)
//         //It could be fixed by adding similar handling to all transducers,
//         //alternatively making it users responsibility to always supply one supertable
//         //with only terminating rules.
//         if(usedBranches.size()>0) //if ==0, such a bot,bot-rule has been created already
//         {
//         	  //this is not needed when optimization is off, but it can reduce number of
//         	  //undefined derived trees i think.
//   
//         	  if(st.isTerminable(bstTermSig, bstNontermSig))
//         	  {
//                 out.write(",\n      ");
//                 s = "";
//                 for(int j=0; j<d; j++)
//                 {
//                   {
//                     s = s + bottom;
//                   }
//                   if(j<d-1)
//                   {
//                     s = s + ",";
//                   }
//                 }
// 
//                 //Axiom table gets same special case as above. Still it affects
//                 //efficiency, not generated langauge.
//                 if(axiomTable!=null && st==axiomTable)
//                 {
//                   out.write( Sname+" -> " + initName + "[" + st.getName() + "[" + s + "]]" );
//                 }
//                 else
//                 {
//   //              	out.write(S0name + " -> " + st.getName() + "[" + s + "] weight " + st.getWeight() );
//                 	out.write(S0name + " -> " + st.getName() + "[" + s + "] weight 0" ); // Changed weight. FD
//                 }
//         	  } // if st.isTerminable
//         }//if usedBranches.size()>0
// 
//         if (t+1 < superTables.getNumTables()) out.write(",\n");
//         else out.write("\n");
//   
//       }
//   
//       //if we dont use optimization. 
//       //this is the only way the first sync tree can be terminated
//       if(useNormalMode==false)
//       {
//         out.write(",\n      " + S0name + " -> " + bottom + " weight 0.2" );
//       }
//       out.write("\n");
//     }
// 
//     /** Generate tree grammar when regular expression is used.
//       * This method and the methods it calls, are except for a few lines,
//       * identical to the corresponding methods in ET0L.
//       */
//     private void generateGrammar(FileWriter out, boolean
//         useNormalMode, rexp r) throws IOException {
//   
//       finiteSignature tableSig = new finiteSignature();
//       for(int i=0; i<superTables.getNumTables(); i++)
//       {
//          SuperTable st = superTables.getSubTable(i);
//          tableSig.addSymbol(new symbol(st.getName(),bstDValue));
//       }
//   
//       tableSig.addSymbol(init);
//       tableSig.addSymbol(bottom);
//   
//       int max = generateRules(r, 0);
//       for (int i = chainRules.size(); i <= max; i++) chainRules.addElement(new Vector());
//       for (int i = regularRules.size(); i <= max; i++) regularRules.addElement(new Vector());
//       addRegularRule(max, bottom + " weight 0");
//       applyClosure();
//       regTreeGrammarNontermSig.addSymbol("S");
//       for (int i = 0; i <= max; i++) regTreeGrammarNontermSig.addSymbol("S" + i);
//       max = removeUnreachable(max);
//       out.write("generators.stubbornRegularTreeGrammar:\n");
//       out.write("  (\n");
//       out.write("    " + regTreeGrammarNontermSig + ",\n");
//       out.write("    " + tableSig + ",\n");
//       out.write("    {\n");
//   //    out.write("      S -> " + init + "[" + grammarSymbol(0) + "],\n");
//       out.write("      S -> " + init + "[" + "S0" + "],\n");
//       for (int i = 0; i < regularRules.size(); i++) {
//         Vector v = (Vector)regularRules.elementAt(i);
//         for (int j = 0; j < v.size(); j++) {
//   //        out.write("      S" + i + " -> " + v.elementAt(j));
//           out.write("      S" + i + " -> ");
//           String table = "";
//           String nonterm = "";
//           try{
//             StringTokenizer tok = new StringTokenizer(v.elementAt(j).toString(),"[]");
//             table = tok.nextToken();
//             nonterm = tok.nextToken();
//           }catch(Exception e)
//           {
//             nonterm = null;
//           }
//             int tabNum = 0;
//             try{
//               tabNum = Integer.parseInt(table.substring(1));
//               table = superTables.getSubTable(tabNum-1).getName();
//               out.write(table);
//             }catch(Exception e)
//             {
//               out.write(table);
//             }
//           if(nonterm!=null)
//           {
//             boolean terminating = true;  // added by FD
//             out.write("[");
//             for(int k=0; k<bstDValue; k++)
//             {
//               SuperTable theTable = findTableWithName(superTables, table);
//               if(theTable==null)
//               {
//                 throw new RuntimeException("This should not happen. Could not find table " + table);
//               }
//               if(isSynctreeSubtreeUseful(theTable, k, 1, bstDValue)) {
//                 out.write( nonterm );
//                 terminating = false;
//               }
//               else
//                 out.write(bottom.toString());
// 
//               if(k+1<bstDValue)
//                 out.write(",");
//             }
//             out.write("]");
//             if (terminating) out.write(" weight 0");
//           }
// 
//           if (i < max || j+1 < v.size()) out.write(",\n"); else out.write("\n");
//         }
//       }
//       out.write("    },\n");
//       out.write("    S\n");
//       out.write("  )");
//       out.close();
//     }
//   
//     boolean loopFlag = false;
// 
//     private int generateRules(rexp r, int n) {
//       switch (r.type) {
//         case rexp.CONC:
//           boolean oldLoopFlag = loopFlag;
//           loopFlag = r.subexp[1].containsLoop();
//           int m = generateRules(r.subexp[0], n);
//           loopFlag = oldLoopFlag;
//           return generateRules(r.subexp[1], m);
//         case rexp.UNION:
//           oldLoopFlag = loopFlag;
//           int m1 = generateRules(r.subexp[0], n+1);
//           boolean newLoopFlag = loopFlag;
//           loopFlag = oldLoopFlag;
//           int m2 = generateRules(r.subexp[1], m1 + 1);
//           loopFlag = loopFlag || newLoopFlag;
//           addChainRule(n, n+1);
//           addChainRule(n, m1+1);
//           addChainRule(m1, m2);
//           return m2;
//         case rexp.PLUS:
//           loopFlag = true;
//           m = generateRules(r.subexp[0], n);
//           addChainRule(m, m + 1);
//           addChainRule(m, n);
//           return m+1;
//         case rexp.STAR:
//           loopFlag = true;
//         case rexp.OPT:
//           m = generateRules(r.subexp[0], n+1);
//           addChainRule(n, m + 1);
//           addChainRule(m, m + 1);
//           addChainRule(n, n + 1);
//           if (r.type == rexp.STAR) addChainRule(m, n + 1);
//           return m+1;
//         default:
//           if (loopFlag) addRegularRule(n, "t" + r.type + "[S" + (n+1) + "]");
//           else addRegularRule(n, "t" + r.type + "[S" + (n+1) + "] weight 0");
//           return n+1;
//       }
//     }
// 
//     private void addChainRule(int lhs, int rhs) {
//       for (int i = chainRules.size(); i <= lhs; i++) chainRules.addElement(new Vector());
//       ((Vector)chainRules.elementAt(lhs)).addElement(new Integer(rhs));
//     }
// 
//     private void addRegularRule(int lhs, String rhs) {
//       for (int i = regularRules.size(); i <= lhs; i++) regularRules.addElement(new Vector());
//       ((Vector)regularRules.elementAt(lhs)).addElement(rhs);
//     }
// 
//     private void applyClosure() {
//       closure();
//       Vector result = new Vector(regularRules.size());
//       for (int i = 0; i < chainRules.size(); i++) {
//         Vector v = new Vector();
//         result.addElement(v);
//         Vector cr = (Vector)chainRules.elementAt(i);
//         for (int j = 0; j < cr.size(); j++) {
//           int k = ((Integer)cr.elementAt(j)).intValue();
//           v.addAll((Vector)regularRules.elementAt(k));
//         }
//       }
//       regularRules = result;
//     }
//   
//     private void closure() {
//       for (int i = 0; i < chainRules.size(); i++) {
//         Vector cr = (Vector)chainRules.elementAt(i);
//         cr.addElement(new Integer(i));
//       }
//       boolean changed;
//       do {
//         changed = false;
//         for (int i = 0; i < chainRules.size(); i++) {
//           Vector cr = (Vector)chainRules.elementAt(i);
//           for (int j = 0; j < cr.size(); j++) {
//             int k = ((Integer)cr.elementAt(j)).intValue();
//             Vector cr2 = (Vector)chainRules.elementAt(k);
//             for (int l = 0; l < cr2.size(); l++) {
//               if (!cr.contains(cr2.elementAt(l))) {
//                 cr.addElement(cr2.elementAt(l));
//                 changed = true;
//               }
//             }
//           }
//         }
//       } while (changed);
//     }
//   
//     private int removeUnreachable(int max) {
//       Vector rhsNonterminals = new Vector(max+1);
//       for (int i = 0; i <= max; i++) {
//         Vector v = (Vector)regularRules.elementAt(i);
//         Vector w = new Vector();
//         rhsNonterminals.add(w);
//         for (int j = 0; j < v.size(); j++) {
//           String rhs = (String)v.elementAt(j);
//           int k = rhs.indexOf('S');
//           if (k >= 0) {
//             try {
//               w.add(new Integer(rhs.substring(k+1, rhs.indexOf(']'))));
//             } catch (NumberFormatException e) { throw new InternalError(); }
//           }
//         }
//       }
//       BitSet reachable = new BitSet(max+1);
//       reachable.set(0);
//       boolean changed = true;
//       while (changed) {
//         changed = false;
//         for (int i = 0; i <= max; i++) {
//           if (reachable.get(i)) {
//             Vector v = (Vector)rhsNonterminals.elementAt(i);
//             for (int j = 0; j < v.size(); j++) {
//               int index = ((Integer)v.elementAt(j)).intValue();
//               if (!reachable.get(index)) {
//                 reachable.set(index);
//                 changed = true;
//               }
//             }
//           }
//         }
//       }
//       for (int i = max; i > 0; i--) {
//         if (!reachable.get(i)) {
//           if (i == max) max--;
//           regTreeGrammarNontermSig.removeSymbol(new symbol("S" + i, 0));
//           regularRules.setElementAt(new Vector(0), i);
//         }
//       }
//       return max;
//     }
// 
//   /** Special case for n=0
//     * BST0 = REGT so we only produce one regular tree grammar and no transducers.
//     */
//     private void generateSolitaryTreeGrammar(FileWriter out) throws IOException {
//       out.write("generators.stubbornRegularTreeGrammar:\n");
//       out.write("  (\n");
//       out.write("    " + bstNontermSig + ",\n");
//       out.write("    " + bstTermSig + ",\n");
//       out.write("    {\n");
//   
//       for (int u=0; u < superTables.getNumRules(); u++) {
//         SyncedRule r = superTables.getRule(u);
//   
//         //all sync info gets removed from rhe rhs
//         String s = "    " + r.getLhs() + " -> " + r.getRhs();
//         out.write(s);
//         if (u+1 < superTables.getNumRules())
//           out.write(",\n");
//         else
//           out.write("\n");
//       }
//       out.write("    },\n");
//       out.write("    " + axiom + "\n");
//       out.write("    )\n");
//       out.write("  }\n");
//       out.close();
//     }//method generateSolitaryTreeGrammar
// 
//   }//class RegularTreeGrammarGenerator
// 
// //=============================================================================
// // Generation of the intermediate top down tree trandcuers
// //=============================================================================
//   class IntermediateTransducerGenerator{
// 
//   //The methods for creating all tree transducers except the last one,
//   //ie transducers having (according to articles definitions) input \Sigma_n
//   //and output \Sigma_{n+1} where n is the parameter transducerNumber
//   //which means the number of this transducer in the chain, ie transducerNumber 
//   //is 1 first time this is called during translation.
//   //The first three functions, of which all functionality is in the third,
//   //takes care of the plain and normal translation types.
//   //The rest of the functions take care of the much more complicated
//   //extended translation alternative.
// 
//     protected void generateIntermediateTransducerUsingNormalTranslation(
//         FileWriter out, int transducerNumber) throws IOException {
//       generateTransducer(out, transducerNumber, true);
//     }
//     protected void generateIntermediateTransducerUsingPlainTranslation(
//         FileWriter out, int transducerNumber) throws IOException {
//       generateTransducer(out, transducerNumber, false);
//     }
//     /** The plain and normal translations are so similar algorithmically
//       * so they are placed in the same method, with a flag deciding
//       * which one is to be used
//       */
//     private void generateTransducer(FileWriter out, int transducerNumber, 
//         boolean usingNormalMode) throws IOException {
//   
//       int currentN = transducerNumber;
//   
//       Vector allTablesOnDepthN = superTables.getTablesAtDepth(currentN);
//       Vector allTablesOnDepthNplus1 = superTables.getTablesAtDepth(currentN+1);
//   
//       out.write("generators.tdTransducer:\n");
//       out.write("  (\n");
// 
//       int d = bstDValue;
//   
//       int dToThePowerOfN = 1;
//       for(int ii=0; ii<currentN; ii++)
//       {
//         dToThePowerOfN *= d;
//       }
//   
//       finiteSignature tdInput = new finiteSignature();
//       tdInput.addSymbol(bottom);
//       tdInput.addSymbol(init);
//   
//       //needed for nonterminal derivations.
//       Enumeration enum0 = regTreeGrammarNontermSig.elements();
//       while (enum0.hasMoreElements()) {
//         tdInput.addSymbol(new symbol(enum0.nextElement().toString(),0));
//       }
//   
//       for(int i=0; i<allTablesOnDepthN.size(); i++)
//       {
//         SuperTable st = (SuperTable)allTablesOnDepthN.elementAt(i);
//         tdInput.addSymbol(new symbol(st.getName(),dToThePowerOfN));
//       }
//       out.write("    " + tdInput + ",\n");
//   
//       finiteSignature tdOutput = new finiteSignature();
//       tdOutput.addSymbol(bottom);
//       tdOutput.addSymbol(init);
//   
//       //needed for nonterminal derivations.
//       Enumeration enum1 = regTreeGrammarNontermSig.elements();
//       while (enum1.hasMoreElements()) {
//         tdOutput.addSymbol(new symbol(enum1.nextElement().toString(),0));
//       }
// 
//       for(int i=0; i<allTablesOnDepthNplus1.size(); i++)
//       {
//         SuperTable st = (SuperTable)allTablesOnDepthNplus1.elementAt(i);
//         tdOutput.addSymbol(new symbol(st.getName(),dToThePowerOfN*d));
//       }
// 
//       out.write("    " + tdOutput + ",\n");
//   
//   //    if(!useExtendedTranslation)
//       {
//         out.write("    " + "{"+qname+"}" + ",\n");        //en td för sync.trees behöver bara en icketerminal
//         out.write("    " + "{\n");
//   
//         out.write("    " + "  "+qname+"[init[x1]] -> init["+qname+"[x1]],\n");
// 
//         boolean qBotToBotRuleNeeded = false;
//   
//         for (int t=0; t < allTablesOnDepthN.size(); t++) {
//           SuperTable st = (SuperTable)allTablesOnDepthN.elementAt(t);//superTables.getSubTable(t);
//   
//           for (int u=0; u < st.getNumTables(); u++) {
//             SuperTable stSub = st.getSubTable(u);
//             String s = "";
//             String s2 = "";
// 
//             Vector usedBranches = stSub.getUsedSyncBranchNumbersForSyncLevel(d,currentN+1);
//   
//             //In example grammar, here the string x1,x2 is created
//             for(int j=0; j<dToThePowerOfN; j++)
//             {
//               s2 = s2 + "x" + (j+1);
//               if(j+1<dToThePowerOfN)
//                 s2 = s2 + ",";
//             }
//   
//             if(translationType == BSTGrammarParser.TYPE_PLAIN)
//               qBotToBotRuleNeeded = true;
//             else //ie normal translation
//             {
//               //According to the construction in the thesis report
//               //this should always be set to true, but it apprears to only be
//               //a matter of efficiency.
//               if(usedBranches.size()>0 && stSub.isTerminable(bstTermSig, bstNontermSig))
//                 qBotToBotRuleNeeded = true;
//             }
// 
//             //In example grammar, here the string q[x1],q[x1],q[x2],q[x2] would created if not optimized
//             //Now for example q[x1],q[x1],bot,bot might be created instead, depending on sync-strings in the current handled table.
//             for(int j=0; j<dToThePowerOfN*d; j++)
//             {
//               if(usingNormalMode==false || usedBranches.contains(new Integer(j)))
//               {
//                 s = s + qname + "[x" + (1+(int)(j/d)) + "]";//stSub.getName()
//               }
//               else
//               {
//                 s = s + bottom;
//               }
//               if(j<(dToThePowerOfN*d)-1)
//               {
//                 s = s + ",";
//               }
//             }
//             String lhsVariables = s2;
//             String rhsVariables = s;
//             out.write("      " + qname + "[" + st.getName() + "[" + lhsVariables + "]] -> " +
//                       stSub.getName() + "[" + rhsVariables + "] weight " + stSub.getWeight() );
//   
//             if (u+1 < st.getNumTables()) out.write(",\n");
//             else out.write("\n");
//           }
//           if (t+1 < allTablesOnDepthN.size()) out.write(",\n");
//           else out.write("\n");
// 
//         }
//         out.write("    ,\n");
//   
//         if(qBotToBotRuleNeeded)
//         {
//           //the rule used when the td hits the leaves of the sync tree
//           out.write("    " + qname + "[" + bottom + "] -> " + bottom + ",\n");
//         }
//   
//         //needed for nonterminal derivations. could look like: q0[S0__] -> S0__
//         Enumeration enum_ = regTreeGrammarNontermSig.elements();
//         while (enum_.hasMoreElements()) {
//           String s = enum_.nextElement().toString();
//           out.write("    " + qname + "[" + s + "] -> " + s + ",\n");
//         }
//   
//         out.write("    " + "  "+qname+"[init[x1]] -> init["+qname+"[x1]]\n");
//       }
//   
//       out.write("    },\n");
//       out.write("    " + qname + "\n");
//       out.write("  )");
//       out.close();
//     }//method generateTransducer
//   
// 
// 
//     /** The powerSet method should return a Vector object
//      * whose elements are exactly the subsets of v, with no repetitions.
//      * More precisely, if v is a Vector object, possibly with
//      * repetitions, the result powerSet(v) is a Vector object u
//      * without repetitions whose elements represent all the subsets of
//      * v. */
//     public Vector powerSet(finiteSignature sig)
//     {
//         Enumeration viter = sig.elements();
//         Vector result = new Vector();
//         if (!viter.hasMoreElements())
//             {
//                 // v is empty, so return its power
//                 // set, i.e., the set with the empty set
//                 // as its only element:
//                 result.add( new finiteSignature() );
//                 return result;
//             }
// 
//         // Pick an element x of v and let
//         // p be the power set of v - {x}.
//         // Then p is the set of subsets of v
//         // not containing x.
//         Object x = viter.nextElement();
//         finiteSignature vminusx = new finiteSignature();
//         while (viter.hasMoreElements())
//         {
//             vminusx.addSymbol( (symbol)viter.nextElement() );
//         }
//         Vector p = powerSet(vminusx);
// 
//         // Build the power set of v
//         // by adding, for each s in p,
//         // s u {x}
//         Iterator piter = p.iterator();
//         while (piter.hasNext())
//         {
//             finiteSignature s = (finiteSignature) piter.next();
//             // make a copy of s and add x to it
//             Object t = s.clone();//Sets1.uniquify(s);
//             s.addSymbol((symbol)x);
//             // add both s and t to the answer
//             result.add(s);
//             result.add(t);
//         }
//         return result;
//     }
// 
//     public Vector findSmartSubsetForStateSignature(Vector hardCoded, String axiomSymbol, Vector allTablesOnDepthN, Vector allTablesOnDepthNplus1, int currentN)
//     {
//       ///// try to replace powerset of S with smart subset of powerset of S
//         int d = bstDValue;
//         int dToThePowerOfN = 1;
//         for(int ii=0; ii<currentN; ii++)
//         {
//           dToThePowerOfN *= d;
//         }
//       Enumeration enum_3 = hardCoded.elements();
//       fixedRankSignature tempSig = new fixedRankSignature(0);  //used to keep track of duplicates
//       Vector smartSubset = new Vector();
//   
//       //make sure that one set containing only axiom symbol is there
//       {
//         fixedRankSignature temp = new fixedRankSignature(0);
//         temp.addSymbol(axiomSymbol);
//         tempSig.addSymbol(qname+"__"+sigToState(temp));
//         smartSubset.addElement(temp);
//       }
//   
//       while (enum_3.hasMoreElements())
//       {
//         finiteSignature currentNonTerms= (finiteSignature)enum_3.nextElement();
// 
//         for (int t=0; t < allTablesOnDepthN.size(); t++) {
//           SuperTable st = (SuperTable)allTablesOnDepthN.elementAt(t);
//   
//           for (int u=0; u < st.getNumTables(); u++) {
//             SuperTable stSub = st.getSubTable(u);
//   
//               Vector usedBranches = stSub.getUsedSyncBranchNumbersForSyncLevel(d,currentN+1);
//   
//               //In example grammar, here the string q[x1],q[x1],q[x2],q[x2] would created if not optimized
//               //Now for example q_S[x1],q_C[x1],bot,bot might be created instead
//               for(int j=0; j<dToThePowerOfN*d; j++)
//               {
//                 if(usedBranches.contains(new Integer(j)))
//                 {
//                   fixedRankSignature possibleDerivedNonterms = stSub.nonterminalslnRhsThatUseGivenSyncNumber(currentNonTerms, j, d, currentN);
//                   String str = qname+"__"+sigToState(possibleDerivedNonterms);
//   
//                   if(!tempSig.contains(new symbol(str,0)))
//                   {
//                     tempSig.addSymbol(qname+"__"+sigToState(possibleDerivedNonterms));
//                     smartSubset.addElement(possibleDerivedNonterms);
//                   }
//                 }
//               }
//           }//for each subtable
//         }
//       }//end of while loop
//       return smartSubset;
//       //////// end replace powerset
//     }
// 
//     protected class SymbolComparator implements Comparator
//     {
//       public int compare(Object o1, Object o2)
//       {
//         return o1.toString().compareTo(o2.toString());
//       }
//     }
//     //Help function for generateTransducer
//     //Creates a state-name corresponding to a set of nonterminals
//     //Example: On input: {E,C} the functtion returns: "_E_C"
//     //         On input: {}    the functtion returns: ""
//     private String sigToState(finiteSignature sig)
//     {
//       String ret = new String();
//       Vector v = new Vector();
//       Enumeration enum_2 = sig.elements();
//       while (enum_2.hasMoreElements()) {
//         v.addElement(enum_2.nextElement());
//       }
//   
//       //We must sort v, otherwise {E,C} and {C,E} would become distinct symbols.
//       //Sorting is done in increasing alphabetic order
//       java.util.Collections.sort(v, new SymbolComparator());
// 
//       for(int i=0; i<v.size(); i++)
//       {
//         ret = ret + "_" + v.elementAt(i);
//       }
//       return ret;
//     }
//   
//     protected void generateIntermediateTransducerUsingExtendedTranslation(
//         FileWriter out, int transducerNumber) throws IOException {
//   
//       int currentN = transducerNumber;
//   
//       Vector allTablesOnDepthN = superTables.getTablesAtDepth(currentN);
//       Vector allTablesOnDepthNplus1 = superTables.getTablesAtDepth(currentN+1);
//   
//       out.write("generators.tdTransducer:\n");
//       out.write("  (\n");
//   
//       int d = bstDValue;
//   
//       int dToThePowerOfN = 1;
//       for(int ii=0; ii<currentN; ii++)
//       {
//         dToThePowerOfN *= d;
//       }
//   
//       finiteSignature tdInput = new finiteSignature();
//       tdInput.addSymbol(bottom);
//       tdInput.addSymbol(init);
//   
//       //needed for nonterminal derivations.
//       Enumeration enum0 = regTreeGrammarNontermSig.elements();
//       while (enum0.hasMoreElements()) {
//         tdInput.addSymbol(new symbol(enum0.nextElement().toString(),0));
//       }
//   
//       for(int i=0; i<allTablesOnDepthN.size(); i++)
//       {
//         SuperTable st = (SuperTable)allTablesOnDepthN.elementAt(i);
//         tdInput.addSymbol(new symbol(st.getName(),dToThePowerOfN));
//       }
//       out.write("    " + tdInput + ",\n");
// 
//       finiteSignature tdOutput = new finiteSignature();
//       tdOutput.addSymbol(bottom);
//       tdOutput.addSymbol(init);
//   
//       //needed for nonterminal derivations.
//       Enumeration enum1 = regTreeGrammarNontermSig.elements();
//       while (enum1.hasMoreElements()) {
//         tdOutput.addSymbol(new symbol(enum1.nextElement().toString(),0));
//       }
//   
//       for(int i=0; i<allTablesOnDepthNplus1.size(); i++)
//       {
//         SuperTable st = (SuperTable)allTablesOnDepthNplus1.elementAt(i);
//         tdOutput.addSymbol(new symbol(st.getName(),dToThePowerOfN*d));
//       }
//   
//       out.write("    " + tdOutput + ",\n");
// 
//       {
//         //if useExtendedTranslation
//         fixedRankSignature smartStateSig = new fixedRankSignature(1);
//         fixedRankSignature secondStateSig = new fixedRankSignature(1);
//         try{
//           Vector fullPowerSet;
//           fullPowerSet = powerSet(bstNontermSig);
//           Vector smartSubset = new Vector();
// 
//           smartSubset = findSmartSubsetForStateSignature(fullPowerSet,
//             axiom.toString(), allTablesOnDepthN, allTablesOnDepthNplus1, currentN);
//   
//             /*
//             //No case has been found where it is an improvement to call twice.
//             {
//             	Vector secondSet = new Vector();
//             	int numstates = 0;
//             	while(true)
//             	{
//                 secondSet = findSmartSubsetForStateSignature(secondSet,
//                   axiom.toString(), allTablesOnDepthN, allTablesOnDepthNplus1, currentN);
//                 if(secondSet.size() > numstates)
//                 {
//                   numstates = secondSet.size();
//                   continue;  //continue as long as state set increases
//                 }
//                 smartSubset = secondSet;
//                 break;
//             	}
//             }
//             */
//   
//             //This would not be needed if q0 was replaced by {bstStart} as
//             //start-state in the td. A possible change to do.
//             if(!smartSubset.contains(null))
//             {
//               smartSubset.addElement(null);  //the "dont know" signature
//               //it might be redundant with the signature containing all nonterminals
//               //so if we want to improve efficiency that could be taken care of,
//               //for example by deleting that signature.
//             }
//   
//             Enumeration enum_2 = smartSubset.elements();
//             while (enum_2.hasMoreElements()) {
//               finiteSignature currentNonTerms = (finiteSignature)enum_2.nextElement();
//               String stateName;
//               if(currentNonTerms==null /*|| currentNonTerms.size()==0*/) //use the second?
//                 stateName = qname;
//               else
//                 stateName = qname+"__"+sigToState(currentNonTerms);//.toString();
//               smartStateSig.addSymbol(stateName);
//             }
//             Enumeration enum_4 = smartSubset.elements();
//             while (enum_4.hasMoreElements()) {
//               finiteSignature currentNonTerms = (finiteSignature)enum_4.nextElement();
//               String stateName;
//               if(currentNonTerms==null /*|| currentNonTerms.size()==0*/) //use the second?
//                 stateName = qname;
//               else
//                 stateName = qname+"__"+sigToState(currentNonTerms);//.toString();
//               secondStateSig.addSymbol(stateName);
//             }
//             out.write("    " + smartStateSig + ",\n");
//       /*
//             //some debug info
//       //      out.write("    %Debuginfo, FULL POWERSET:" + fullPowerSet + "\n");
//             out.write("    %Debuginfo, FULL POWERSET had " + fullPowerSet.size() + " elements.\n");
//             out.write("    %Debuginfo, USED SUBSET OF POWERSET:\r\n    %" + smartStateSig + "\n");
//             out.write("    %Debuginfo, second state set, check if it's smaller than used set, "+
//                 "if so it would be better to call the subset computation twice or more:\n    %"+
//                 secondStateSig + "\n");
//       */
//             out.write("    " + "{\n");
// 
//             //Loop for all sets of possible sets of nonterminals
//             Enumeration enum_ = smartSubset.elements();
//             while (enum_.hasMoreElements())
//             {
//               finiteSignature currentNonTerms= (finiteSignature)enum_.nextElement();
//               if(currentNonTerms!=null)
//                 out.write("    %Rules for when the possible BST-nonterminals are:" + currentNonTerms + "\n");
//               else
//                 out.write("    %Rules for when the possible BST-nonterminals are unknown:\n");
//               String stateName;
//               if(currentNonTerms==null)
//                 stateName = qname;
//               else
//                 stateName = qname+"__"+sigToState(currentNonTerms);
//   
//               for (int t=0; t < allTablesOnDepthN.size(); t++) {
//                 SuperTable st = (SuperTable)allTablesOnDepthN.elementAt(t);
//   
//                 for (int u=0; u < st.getNumTables(); u++) {
//                   SuperTable stSub = st.getSubTable(u);
//   
//                     String s = "";
//                     String s2 = "";
//   
//                     Vector usedBranches = stSub.getUsedSyncBranchNumbersForSyncLevel(d,currentN+1);
//   
//                     //In example grammar, here the string x1,x2 is created
//                     for(int j=0; j<dToThePowerOfN; j++)
//                     {
//                       s2 = s2 + "x" + (j+1);
//                       if(j+1<dToThePowerOfN)
//                         s2 = s2 + ",";
//                     }
//                     //In example grammar, here the string q[x1],q[x1],q[x2],q[x2] would created if not optimized
//                     //Now for example q_S[x1],q_C[x1],bot,bot might be created instead
//                     for(int j=0; j<dToThePowerOfN*d; j++)
//                     {
//                       String stateNameForRHS;
//                       if(usedBranches.contains(new Integer(j)))
//                       {
//                         fixedRankSignature possibleDerivedNonterms = stSub.nonterminalslnRhsThatUseGivenSyncNumber(currentNonTerms, j, d, currentN);
//                         String str = qname+"__"+sigToState(possibleDerivedNonterms);
//                         if(possibleDerivedNonterms.size()==0)
//                         {
//                           s = s + bottom;
//                         }
//                         else
//                         {
//                           if(smartStateSig.contains(new symbol(str,1)))
//                           {
//                             stateNameForRHS = str;
//                           }
//                           else
//                           {
//                             stateNameForRHS = qname;
//                           }
//                           s = s + stateNameForRHS + "[x" + (1+(int)(j/d)) + "]";
//                         }
//                       }
//                       else
//                       {
//                         s = s + bottom;
//                       }
//                       if(j<(dToThePowerOfN*d)-1)
//                       {
//                         s = s + ",";
//                       }
//                     }
//                     String lhsVariables = s2;
//                     String rhsVariables = s;
//                     out.write("      " + stateName + "[" + st.getName() + "[" + lhsVariables + "]] -> " +
//                               stSub.getName() + "[" + rhsVariables + "] weight " + stSub.getWeight() + ",");
// 
//                     out.write("\n");
//                 }//for each subtable
//               }
// 
//               out.write("    " + stateName + "[" + bottom + "] -> " + bottom + ",\n");
// 
//               //needed for nonterminal derivations. could look like: q0[S0__] -> S0__
//               Enumeration en = regTreeGrammarNontermSig.elements();
//               while (en.hasMoreElements()) {
//                 String s = en.nextElement().toString();
//                 out.write("    " + stateName + "[" + s + "] -> " + s + ",\n");
//               }
//             }//end loop for all sets of possible sets of nonterminals
// 
//         }catch(Exception e)
//         {
//           //Catches and display any exception.
//           throw new RuntimeException(
//             "Error in implementation. Unhandled exception.\n"
//             + e.getClass().getName() + " line " + e.getStackTrace()[0] + "  ");
//         }
// 
//         fixedRankSignature signatureWithOnlyAxiom = new fixedRankSignature(1);
//         signatureWithOnlyAxiom.addSymbol(axiom.toString());
//         String str = qname+"__"+sigToState(signatureWithOnlyAxiom);
//         String stateNameForRHS;
//         if(smartStateSig.contains(new symbol(str,1)))
//         {
//           stateNameForRHS = str;
//         }
//         else
//         {
//           stateNameForRHS = qname;
//         }
//         out.write("    " + "  "+qname+"[init[x1]] -> init["+stateNameForRHS+"[x1]]\n");
//       }//if useExtendedTranslation
// 
//       out.write("    },\n");
//       out.write("    " + qname + "\n");
//       out.write("  )");
//       out.close();
//     }//method generateTransducer
//   }//Class IntermediateTransducerGenerator
// 
// //=============================================================================
// // Generation of the final down down tranducer
// //=============================================================================
//   class FinalTransducerGenerator{
// 
//   /******************* Generation of final transducer ********************/
// 
//     private void generateFinalTransducer(FileWriter out, boolean useYieldTransducer) throws IOException {
//       int n = bstNValue;
//       Vector allTablesOnDepthN = superTables.getTablesAtDepth(n);
// 
//       symbol syncInfoTopSymbol = new symbol("syncinfo", n+1);
//       symbol syncInfoBottomSymbol = new symbol("-",0);
//       out.write("generators.tdTransducer:\n");
//       out.write("  (\n");
// 
//       int d = bstDValue;
// 
//       int dToThePowerOfN = 1;
//       for(int ii=0; ii<n; ii++)
//       {
//         dToThePowerOfN *= d;
//       }
// 
//       finiteSignature tdInput = new finiteSignature();
//       tdInput.addSymbol(bottom);
//       tdInput.addSymbol(init);
//   
//       //Take copy of original terminal signature, so that we dont destroy
//       //it when we need to add things to it later.
//       finiteSignature termSig2 = new finiteSignature();
//       {
//         Enumeration enum_ = bstTermSig.elements();
//         while (enum_.hasMoreElements()) {
//           termSig2.addSymbol((symbol)enum_.nextElement());
//         }
//       }
//   
//       //Take copy of original nonterminal signature too.
//       fixedRankSignature nontermSig2 = new fixedRankSignature(1);
//       {
//         Enumeration enum_ = bstNontermSig.elements();
//         while (enum_.hasMoreElements()) {
//           nontermSig2.addSymbol(((symbol)enum_.nextElement()).toString());
//         }
//       }
//   
//       //needed for nonterminal derivations
//       Enumeration enum2 = regTreeGrammarNontermSig.elements();
//       while (enum2.hasMoreElements()) {
//         tdInput.addSymbol((symbol)enum2.nextElement());
//       }
//   
//       for(int i=0; i<allTablesOnDepthN.size(); i++)
//       {
//         SuperTable st = (SuperTable)allTablesOnDepthN.elementAt(i);
//         tdInput.addSymbol(new symbol(st.getName(),dToThePowerOfN));
//       }
//       out.write("    " + tdInput + ",\n");
//   
//       if(!useYieldTransducer)
//       {
//         termSig2.unionWith(bstNontermSig);  //needed for nonterminal derivations
//       }
//       else
//       {
//         //need to change rank so that nonterms can have syncinfo as subtrees
//         Enumeration enum_ = bstNontermSig.elements();
//         while (enum_.hasMoreElements()) {
//   //        termSig2.addSymbol(new symbol(enum_.nextElement().toString(), n));        //did not work with example pandq
//   //so i no longer change rank
//           termSig2.addSymbol(new symbol(enum_.nextElement().toString(), 0));
//         }
//         termSig2.addSymbol(syncInfoTopSymbol);
//       }
//   
//       boolean debug=false;//for debugging purposes
//       if(debug)
//       {
//         termSig2.addSymbol(new symbol("error_no_init",0));
//       }
//   
//       if(useYieldTransducer)
//       {
//         //termSig2.unionWith(bstSyncSymbolSig);      //need to change rank to 1
//         Enumeration enum_ = bstSyncSymbolSig.elements();
//         while (enum_.hasMoreElements()) {
//           termSig2.addSymbol(new symbol(enum_.nextElement().toString(), 1));
//         }
//         termSig2.addSymbol(syncInfoBottomSymbol);
//         termSig2.addSymbol(new symbol("subst", n+1));
//         for(int i=0; i<n; i++)
//         {
//           termSig2.addSymbol(new symbol("proj-"+(i+1), 0));
//         }
//       }
//       out.write("    " + termSig2 + ",\n");  //output signature for the td
//       nontermSig2.addSymbol(qname);
//   
//       out.write("    " + nontermSig2 + ",\n");
//       out.write("    " + "{\n");
//   
//       Vector origSyncSymbolNames = new Vector();
//       if(useYieldTransducer)
//       {
//         Vector tmp = new Vector();
//         Enumeration enum_ = bstSyncSymbolSig.elements();
//         while (enum_.hasMoreElements()) {
//           tmp.addElement(enum_.nextElement().toString());
//         }
//         for(int j=tmp.size()-1; j>=0; j--)
//         {
//           //this seems to be required to put them in the right order
//           origSyncSymbolNames.addElement(tmp.elementAt(j));
//         }
//         //A0[T00[x1]] -> subst[f[g[A[x1]],A[x1]], 0[-], 0[-]],
//         String s = qname + "[" + init + "[x1]] -> subst[" + axiom + "[x1] " ;
//         for(int j=0; j<n; j++)
//         {
//           s = s + ", -";
//         }
//         s = s + "]";
//         out.write("      " + s +"\n    ");
//       }
//       else
//       {
//         //since always axiom now is symbol and not tree we can do as below
//         //for example this could become: q0[init[x1]] -> S[x1]
//         String s = qname + "[" + init + "[x1]] -> " + axiom + "[x1]";
//         out.write("      " + s +"\n    ");
//       }
//   
//       for (int t=0; t < allTablesOnDepthN.size(); t++) {
//   
//         SuperTable st = (SuperTable)allTablesOnDepthN.elementAt(t);//superTables.getSubTable(t);
//   
//         for (int u=0; u < st.getNumRules(); u++) {
//           SyncedRule r = st.getRule(u);
//   
//           out.write(",\n    ");
//   
//           String s;
//           if(useYieldTransducer)
//           {
//             //A[T00[x1]] -> subst[f[g[A[x1]],A[x1]], 0[proj-1], 0[proj-2]],
//             s = r.getFinalTDRuleWithYield(st, dToThePowerOfN, d, origSyncSymbolNames);
//           }
//           else
//           {
//             s = r.getFinalTDRule(st, dToThePowerOfN, d);
//           }
//   
//           out.write("  " + s + " weight " + r.getWeight() );
//         }
//         out.write("\n      ");
//       }//for all tables on depth n
//   
//       //needed for nonterminal derivations
//       Enumeration enum_ = bstNontermSig.elements();
//       while (enum_.hasMoreElements()) {
//         //could look like: S[S0__] -> S,
//         out.write("\n      ");
//         String s = enum_.nextElement().toString();  //nonterminal name
//         if(!useYieldTransducer)
//         {
//           Enumeration en = regTreeGrammarNontermSig.elements();
//           while (en.hasMoreElements()) {
//             out.write(",\n      " + s + "[" + en.nextElement() + "] -> " + s );
//           }
//         }
//         else
//         {
//           String s2 = "";
//           for(int i=0; i<n; i++)
//           {
//             s2 = s2 + "proj-" + (i+1);
//             if(i+1<n)
//               s2 = s2 + ",";
//           }
//           Enumeration en = regTreeGrammarNontermSig.elements();
//           while (en.hasMoreElements()) {
//             out.write(",\n      " + s + "[" + en.nextElement() + "] -> " + 
//  	 	syncInfoTopSymbol + "[" + s + ", "+s2 + "]");
//           }
//         }
//       }
// 
//       if(debug)
//       {
//         try{
//         String s = regTreeGrammarNontermSig.elements().nextElement().toString();
//         out.write( ",\n   "+qname+"[" + s + "] -> error_no_init \n");
//         }catch(Exception e2){}//should not happen
//       }
// 
//       out.write("},\n");
//       out.write("    "+qname+"\n");
//       out.write("  )");
//       out.close();
//     }//method generateFinalTransducer
//   }//class FinalTransducerGenerator
// 
// }//class BSTGrammar
// 
// 
