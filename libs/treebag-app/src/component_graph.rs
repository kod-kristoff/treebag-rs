use std::rc::Rc;

use crate::WorksheetNode;

//package gui;
//
//import java.io.*;
//import java.lang.reflect.*;
//import java.awt.*;
//import java.awt.event.*;
//import javax.swing.*;
//import java.util.*;
//import gui.nodeTypes.*;
//import parsers.*;
//import util.*;
//
//public class componentGraph extends JPanel implements MouseListener {
//
///**
///	 * 
///
pub struct ComponentGraph {
//  private static final long serialVersionUID = 3046505008622990455L;
    nodes: Vec<Rc<dyn WorksheetNode>>,
//  private list edges = new list();
//  private int defaultWidth = 500;
//  private int defaultHeight = 500;
//  private Dimension preferredSize = new Dimension(defaultWidth, defaultHeight);
//  private invisibleNode invisible = new invisibleNode();
//  private edge newEdge = null;
//  private int activeCount = 0;
//  private JTextField progressField = null;
}

impl ComponentGraph {
    pub fn new() -> Self {
        log::trace!("ComponentGraph::new");
        Self {
            nodes: Vec::new(),
        }
    }
}
//  public componentGraph() {
//    super();
//    setPreferredSize(preferredSize);
//    setBorder(BorderFactory.createRaisedBevelBorder());
//    setLayout(null);
//    setBackground(Color.white);
//    addMouseListener(this);
//    add(invisible);
//  }
//  
//  public synchronized boolean confirm() {
//    list l = nodes;
//    while (true) {
//      if (l.isEmpty()) return true;
//      if (((worksheetNode)l.head()).isUnsaved()) break;
//      l = l.tail();
//    }
//    int choice = JOptionPane.showConfirmDialog(null,
//      "Unsaved file(s). Proceed anyway?",
//      "Unsaved file(s)!",
//      JOptionPane.YES_NO_OPTION,
//      JOptionPane.QUESTION_MESSAGE);
//    return choice == 0;
//  }
//  
//  public synchronized void setProgressField(JTextField t) {
//    progressField = t;
//    updateProgress();
//  }
//  
//  public synchronized void addToActiveCount(int c) {
//    activeCount += c;
//    if (progressField != null && (activeCount == 0 || activeCount == c))
//    {
//      updateProgress();
//    }
//  }
//  
//  private synchronized void updateProgress() {
//    if (activeCount == 0) {
//      progressField.setBackground(null);
//      progressField.setText("ready");
//    }
//    else if (activeCount > 0) {
//      progressField.setBackground(Color.orange);
//      progressField.setText("busy");
//    }
//    else throw new InternalError("Negative number of busy components");
//    progressField.setSize(progressField.getPreferredSize());
//  }
//
impl ComponentGraph {
    pub fn add_node(&mut self, n: Rc<dyn WorksheetNode>) {

//  public void addNode(worksheetNode n, Point position) {
        self.nodes.push(n);
//    super.add(n);
//    n.setPosition(position);
//    addComponentListener(n);
//    invalidate();
//    repaint();
    }
}
//  
//  public void deleteNode(worksheetNode n) {
//    list l = edges;
//    while (!l.isEmpty()) {
//      edge e = (edge)l.head();
//      if (n == e.source || n == e.target) deleteEdge(e);
//      else l = l.tail();
//    }
//    l = nodes;
//    while (n != l.head()) l = l.tail();
//    l.removeFirst();
//    super.remove(n);
//    invalidate();
//    repaint();
//  }
//  
//  public void clear() {
//    if (confirm()) deleteAllNodes();
//  }
//  
//  private void deleteAllNodes() {
//    while (!nodes.isEmpty()) {
//      worksheetNode n = (worksheetNode)nodes.head();
//      n.delete(true);
//    }
//  }
//  
//  public worksheetNode getNodeAt(int x, int y) {
//    list l = nodes;
//    while (!l.isEmpty()) {
//      worksheetNode n = (worksheetNode)l.head();
//      if (n.contains(x-n.getLocation().x, y-n.getLocation().y)) return n;
//      l = l.tail();
//    }
//    return null;
//  }
//  public worksheetNode newNode(Point pos, String file) {
//    worksheetNode n = null;
//    if (file != null) {
//      n = parseNode(file);
//      if (n != null) addNode(n, pos);
//    }
//    return n;
//  }
//  
//  public void newNode(Point pos) {
//    newNode(pos,fileChooser.selectFile("Load TREEBAG component", FileDialog.LOAD));
//  }
//  
//  private worksheetNode parseNode(String fileName) {
//    try {
//      addToActiveCount(1);
//      parsable parsed = null;
//      includer stream = null;
//      boolean ok = false;
//      try {
//        stream = new includer(new File(fileName));
//        objectParser parser = new objectParser(new ASCII_CharStream(stream,1,1));
//        parsed = parser.parse();
//        ok = true;
//      } catch (ParseException e) {
//        errorDisplay.show("Parsing of `" + fileName + "' failed:\n" + e.getMessage());
//      } catch (TokenMgrError e) {
//        errorDisplay.show("Parsing of `" + fileName + "' failed:\n" + e.getMessage());
//      } catch (includer.IncludeFileNotFoundException e) {
//        errorDisplay.show("Parsing of `" + fileName + "' failed:\n" + e.getMessage());
//      }
//      finally { if (stream != null) try { stream.close(); } catch (IOException e) {} }
//      if (!ok) return null;
//      if (parsed.getName() == null) {
//        String name = fileName.substring(fileName.lastIndexOf(File.separatorChar)+1);
//        parsed.setName("file " + name);
//      }
//      Class cl = parsed.getClass();
//      Object[] args = new Object[2];
//      args[0] = parsed; args[1] = fileName;
//      worksheetNode result = null;
//      do {
//        try {
//          String className = cl.getName() + "Node";
//          className = className.substring(className.lastIndexOf('.')+1);
//          Class nodeClass = Class.forName("gui.nodeTypes." + className);
//          Constructor constr = nodeClass.getConstructors()[0];
//          result = (worksheetNode)constr.newInstance(args);
//        }
//        catch ( ClassNotFoundException e )  {
//          cl = cl.getSuperclass();
//          if (cl != null) continue;
//          else result = new worksheetNode(parsed, fileName);
//        }
//        catch ( IllegalAccessException e )  { throw new InternalError(e.toString()); }
//        catch ( InvocationTargetException e )  { throw new InternalError(e.getTargetException().toString()); }
//        catch ( InstantiationException e )  { throw new InternalError(e.toString()); }
//        try { result.initialize(); }
//        catch (ParseException e) {
//          errorDisplay.show(e.getMessage());
//          return null;
//        }
//        result.start();
//        return result;
//      } while (true);
//    }
//    finally { addToActiveCount(-1); }
//  }
//  
//  public void deleteEdge(edge e) {
//    e.target.inputEdgeDeleted(e);
//    list l = edges;
//    while (e != l.head()) l = l.tail();
//    l.removeFirst();
//    e.source.deleteMotionObserver(e);
//    e.target.deleteMotionObserver(e);
//    ((worksheetNode)e.source).deleteObservingEdge(e);
//    super.remove(e);
//    invalidate();
//    repaint();
//  }
//  
//  public void mouseClicked(MouseEvent e) {
//    Component c = getComponentAt(e.getPoint());
//    if (c != null && c instanceof edge) {
//      node source = ((edge)c).source;
//      deleteEdge((edge)c);
//      createDanglingEdge(source, e.getPoint());
//    }
//    else if (e.getClickCount() > 1) newNode(e.getPoint());
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
//  public void createDanglingEdge(node source, Point target) {
//    add(newEdge = new edge(source, invisible));
//    invisible.activateAtWith(target.x, target.y, newEdge);
//  }
//  
//  public void setTarget(Point p) {
//    worksheetNode source = (worksheetNode)newEdge.source;
//    remove(newEdge);
//    newEdge = null;
//    invalidate();
//    worksheetNode target = getNodeAt(p.x, p.y);
//    addEdge(source, target);
//    repaint();
//  }
//  
//  private void addEdge(worksheetNode source, worksheetNode target) {
//    if (target != null && target.acceptsSource(source)) {
//      edge e = new edge(source, target);
//      source.addObservingEdge(e);
//      edges.append(e);
//      super.add(e);
//      invalidate();
//      repaint();
//      target.inputEdgeEstablished(e);
//    }
//  }
//  
//  public boolean existsPath(worksheetNode from, worksheetNode to) {
//    if (from == to) return true;
//    list frontier = new list();
//    frontier.append(from);
//    list unused = (list)edges.clone();
//    while (!frontier.isEmpty()) {
//      node n = (node)frontier.head();
//      frontier.removeFirst();
//      list l = unused;
//      while (!l.isEmpty()) {
//        edge e = (edge)l.head();
//        if (e.source == n) {
//          if (e.target == to) return true;
//          l.removeFirst();
//          frontier.append(e.target);
//        }
//        else l = l.tail();
//      }
//    }
//    return false;
//  }
//  
//  public void saveConfig() {
//    String fileName = fileChooser.selectFile("Save worksheet configuration", FileDialog.SAVE);
//    if (fileName != null) {
//      try { 
//        File file = new File(fileName);
//        if (file.exists()) {
//          File old = new File(fileName+".bak");
//          if (old.exists()) old.delete();
//          if (!file.renameTo(old)) {
//            errorDisplay.show("Coudn't rename old file " + fileName + " to " +
//                                           fileName + ".bak;\noperation cancelled");
//            return;
//          }
//        }
//        saveConfig(new RandomAccessFile(file,"rw"));
//      }
//      catch (IOException e) {
//        errorDisplay.show("Could not write to file " + fileName + ":\n" + e);
//      }
//    }
//  }
//  
//  private void saveConfig(RandomAccessFile out) throws IOException {
//    out.writeBytes("Worksheet configuration\n");
//    if (!(getSize().width==defaultWidth && getSize().height==defaultHeight)) {
//      out.writeBytes("\nWorksheet size is (" + getSize().width + ", " + getSize().height + ")\n");
//    }
//    list l; int i;
//    for (l = nodes, i = 0; !l.isEmpty(); l = l.tail(), i++) {
//      worksheetNode n = (worksheetNode)l.head();
//      int ups = 0;
//      String common = fileChooser.getDirectory();
//      while (!n.fileName.startsWith(common)) {
//        common = common.substring(0, common.lastIndexOf(File.separator, common.length()-2)+1);
//        ups++;
//      }
//      String f = n.fileName.substring(common.length());
//      out.writeBytes("\nComponent " + i + " is ");
//      if (ups > 0 && common.length() > 0) out.writeBytes(ups + " up ");
//      out.writeBytes("\"" + f + "\" at (" + n.position.x + "," + n.position.y + ") ");
//      for (int j = 0; j < n.openWindows(); j++) out.writeByte('*');
//    }
//    l = edges;
//    if (!l.isEmpty()) {
//      out.writeBytes("\n\nEdges are\n");
//      do {
//        edge e = (edge)l.head();
//        out.writeBytes("\n  " + String.valueOf(nodeNumber(e.source)) +
//                                " -> " + String.valueOf(nodeNumber(e.target)));
//      } while ( !(l = l.tail()).isEmpty() );
//    }
//    out.writeBytes("\n\nend\n");
//    out.close();
//  }
//  
//  private int nodeNumber(node n) {
//    list l; int i;
//    for (i = 0, l = nodes; true; i++, l=l.tail()) {
//      if (l.isEmpty()) throw new InternalError();
//      if (l.head() == n) return i;
//    }
//  }
//  
//  public void addConfig(boolean isLoad, worksheet ws) {
//    if (isLoad && !confirm()) return;
//    String fileName;
//    if (isLoad) fileName = fileChooser.selectFile("Load worksheet configuration", FileDialog.LOAD);
//    else fileName = fileChooser.selectFile("Add worksheet configuration", FileDialog.LOAD);
//    if (fileName != null) addConfig(isLoad, fileName, ws);
//  }
//  
//  public void addConfig(boolean isLoad, String fileName, worksheet ws) {
//	fileChooser.setDirectory(new File(fileName).getParent());
//    configParser parser = null;
//    boolean ok = false;
//    includer stream = null;
//    try {
//      stream = new includer(new File(fileName));
//      parser = new configParser(new configParserTokenManager(new ASCII_CharStream(stream,1,1)));
//      parser.parse(fileName);
//      ok = true;
//    }
//    catch (includer.IncludeFileNotFoundException e) {
//      errorDisplay.show("Parsing of file " + fileName + " failed:\n" + e.getMessage());
//    }
//    catch (ParseException e) {
//      errorDisplay.show("Parsing of `" + fileName + "' failed:\n" + e.getMessage());
//    }
//    catch (TokenMgrError e) {
//      errorDisplay.show("Parsing of `" + fileName + "' failed:\n" + e.getMessage());
//    }
//    finally { if (stream != null) try { stream.close(); } catch (IOException e) {} }
//    if (ok) {
//      if (isLoad) deleteAllNodes();
//      if (isLoad && parser.xDim >= 0) preferredSize = new Dimension(parser.xDim, parser.yDim);
//      else preferredSize = getSize();
//      setPreferredSize(preferredSize);
//      ws.setReadMe(parser.readMe);
//      ws.pack();
//      Vector createdNodes = new Vector();
//      insertNodes(parser.nodes, createdNodes);
//      insertEdges(parser.edges,createdNodes);
//      executeCommands(parser.nodes, createdNodes);
//      openNodes(parser.nodes, createdNodes);
//    }
//  }
//    
//  private void insertNodes(Vector infoVector, Vector created) {
//    for (int i = 0; i < infoVector.size(); i++) {
//      configParser.nodeInfo info = (configParser.nodeInfo)infoVector.elementAt(i);
//      worksheetNode n = parseNode(info.fileName);
//      if (n != null) addNode(n, info.position);
//      created.addElement(n);
//    }
//  }
//  
//  private void insertEdges(Vector edges, Vector nodes) {
//    for (int i = 0; i < edges.size(); i++) {
//      configParser.edgeInfo info = (configParser.edgeInfo)edges.elementAt(i);
//      worksheetNode source = (worksheetNode)nodes.elementAt(info.source);
//      worksheetNode target = (worksheetNode)nodes.elementAt(info.target);
//      if (source != null && target != null) addEdge(source, target);
//    }
//  }
//  
//  private void executeCommands(Vector infoVector, Vector nodes) {
//    for (int i = 0; i < infoVector.size(); i++) {
//      worksheetNode node = (worksheetNode)nodes.elementAt(i);
//      if (node==null) continue;
//      configParser.nodeInfo info = (configParser.nodeInfo)infoVector.elementAt(i);
//      for (int j = 0; j < info.execute.size(); j += 2) {
//        int multiplicity = ((Integer)info.execute.elementAt(j)).intValue();
//        String command = (String)info.execute.elementAt(j+1);
//        while (multiplicity-- > 0) node.transferCommand(command, worksheetNode.USER_COMMAND);
//      }
//    }
//  }
//  
//  private void openNodes(Vector infoVector, Vector nodes) {
//    for (int i = 0; i < infoVector.size(); i++) {
//      worksheetNode node = (worksheetNode)nodes.elementAt(i);
//      if (node==null) continue;
//      configParser.nodeInfo info = (configParser.nodeInfo)infoVector.elementAt(i);
//      for (int j = 0; j < info.openWindows; j++) {
//        node.open();
//      }
//    }
//  }
//  
//}
