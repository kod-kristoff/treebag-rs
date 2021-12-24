package terms;

/** A ranked symbol.
  */
public class symbol {

  protected String name;
  protected int    rank;
  
/** A symbol whose name and rank are given as arguments.
  * @param n the name of the symbol
  * @param rk its rank
  */
  public symbol(String n, int rk) {
    name = n.intern();
    rank = rk;
  }
  
/** A symbol whose instance variables are not yet inititalised. */
  protected symbol() {} 
  
/** Returns the name of the symbol.
  * The names of symbols can be compared using the == operator because this method
  * returns the canonical representation of the string obtained by an application of the method
  * <code>intern()</code>.
  */
  public String toString() {
    return name;
  }

/** Returns the rank of the symbol.
  */
  public int rank() {
    return rank;
  }

/** Two symbols are equal iff their names and ranks coincide.
  */
  public boolean equals(Object other) {
    if (other instanceof symbol) {
      return name==((symbol)other).toString() && rank==((symbol)other).rank();
    }
    else return false;
  }

  public int hashCode() {
    return name.hashCode();
  }
  
}
package terms;

/** A ranked symbol.
  */
public class synchronizedSymbol extends symbol {

  private int[] syncInfo;
  
/** A symbol of rank 0 with synchronization information.
  * @param n the name of the symbol
  * @param syncSize the number of synchronization numbers
  */
  public synchronizedSymbol(String n, int syncSize) {
    super(n,0);
    syncInfo = new int[syncSize];
  }
  
  public void setSync(int index, int sync) {
    syncInfo[index] = sync;
  }
  
  public int[] getSync() {
    return (int[])syncInfo.clone();
  }
  
  public void setSync(int syncInfo[]) {
    this.syncInfo = syncInfo;
  } 
}
package terms;

public class variable extends symbol {

  int index = 0;

  public variable(String name) {
    super(name, 0);
    try {
      index = Integer.parseInt(name.substring(1));
    } catch (NumberFormatException e)
    { throw new InternalError(); }
  }
  
  public variable(int index) {
    super("x" + new Integer(index).toString(), 0);
    this.index = index;
  }
  
  public static boolean isVariable(String name) {
    if (name.length() < 2) return false;
    if (name.charAt(0) != 'x') return false;
    if (name.charAt(1) == '0' && name.length() > 2) return false;
    for (int i = 1; i < name.length(); i++) {
      if (name.charAt(i) < '0' || name.charAt(i) > '9') return false;
    }
    return true;
  }
  
  public int index() { return index; }
  
}
package terms;

/**
 * A copy of variable.java, but symbol names must be
 * y1, y2, ... instead of x1, x2, ...
 */

public class parameter extends symbol {

    int index = 0;

    public parameter(String name) {
        super(name, 0);
        try {
            index = Integer.parseInt(name.substring(1));
        } catch (NumberFormatException e)
        { throw new InternalError(); }
    }

    public parameter(int index) {
        super("y" + new Integer(index).toString(), 0);
        this.index = index;
    }
    
    public static boolean isParameter(String name) {
        if (name.length() < 2) return false;
        if (name.charAt(0) != 'y') return false;
        if (name.charAt(1) == '0' && name.length() > 2) return false;
        for (int i = 1; i < name.length(); i++) {
            if (name.charAt(i) < '0' || name.charAt(i) > '9') return false;
        }
        return true;
    }

    public int index() { return index; }
}

