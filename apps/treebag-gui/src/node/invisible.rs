//package gui;
//
//import java.awt.*;
//import java.awt.event.*;
//import gui.nodeTypes.*;
//
///** An invisible <code>node</code> which is able to follow the mouse.
///
pub struct InvisibleNode {}
//public class invisibleNode extends node implements MouseListener,
//                                               MouseMotionListener {
//  
//  /**
//	 * 
//	 */
//  private static final long serialVersionUID = 1234662582113166399L;
//  private edge dangling = null;
//  private static Color colourOK; { colourOK = Color.green.darker(); }
//  private static Color colourKO; { colourKO = Color.red; }
//
impl InvisibleNode {
    pub fn new() -> Self {
        log::trace!("InvisibleNode::new");
        InvisibleNode {}
    }
}
//  public void activateAtWith(int x, int y, edge dangling) {
//    setPosition(x,y);
//    this.dangling = dangling;
//    addMouseListener(this);
//    addMouseMotionListener(this);
//    setChanged();
//    notifyObservers();
//  }
//  
//  public void mouseClicked(MouseEvent e) {
//    removeMouseMotionListener(this);
//    removeMouseListener(this);
//    parent().setTarget(position);
//    dangling = null;
//  }
//  
//  public void mousePressed(MouseEvent e) {
//  }
//  
//  public void mouseReleased(MouseEvent e) {
//  }
//  
//  public void mouseEntered(MouseEvent e) {
//  }
//  
//  public void mouseExited(MouseEvent e) {
//  }
//  
//  public void mouseDragged(MouseEvent e) {
//  }
//  
//  public void mouseMoved(MouseEvent e) {
//    setPosition(position.x + e.getPoint().x, position.y + e.getPoint().y);
//  }
//  
//  private void setPosition(int x, int y) {
//    Dimension d = getParent().getSize();
//    x = Math.min(x, d.width);
//    y = Math.min(y, d.height);
//    position.x = Math.max(0, x);
//    position.y = Math.max(0, y);
//    setBounds(position.x, position.y, 0, 0);
//    setChanged();
//    notifyObservers();
//  }
//  
//  public boolean contains(int x, int y) {
//    return dangling != null;
//  }
//  
//  public boolean contains(Point p) {
//    return true;
//  }
//  
//  public Point getBorderPoint(Point seenFrom) {
//    if (dangling == null) return position;
//    worksheetNode n = parent().getNodeAt(position.x, position.y);
//    if (n != null && n.acceptsSource((worksheetNode)dangling.source)) {
//      dangling.setColour(colourOK);
//      return n.getBorderPoint(seenFrom);
//    }
//    dangling.setColour(colourKO);
//    return position;
//  }
//  
//}
//
