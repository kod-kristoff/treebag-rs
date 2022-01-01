use crate::{
    component_graph::ComponentGraph,
};
//package gui;
//
//import java.awt.*;
//import java.awt.event.*;
//import javax.swing.*;
//import java.util.*;
//import java.io.*;
//
//public class worksheet extends JFrame implements ActionListener, MouseListener {
//
///** This is the central class of TREEBAG - the worksheet.
///	* The code assumes that only one instance of the worksheet
///	* is active (created either by the <code>main</code> method
///	* or an instance of <code>tbApplet</code>).
///	*/
pub struct Worksheet {
    graph: ComponentGraph,
}
//  public static commandTabs tabs;
//  private static final long serialVersionUID = -6215999136996319169L;
//  private static String versionNumber = "1.62";
//  private static String load = "Load...";
//  private static String add = "Add...";
//  private static String save = "Save...";
//  private static String deleteAll = "clear";
//  private static String quit = "quit";
//  private static String addComp = "add component...";
//  private static String createComp = "create component...";
//  private static String[] worksheetCommands = { load, add, save, deleteAll, null, quit };
//  private static String[] componentCommands = { addComp, createComp };
//  
//  private componentGraph graph;
//  private PopupMenu popup = new PopupMenu();
//  private Point popedUpAt;
//  private worksheetPanel panel;
//  private tbApplet parent;
//  
impl Worksheet {
    pub fn new() -> Self {
        log::trace!("Worksheet::new");
        Self {
            graph: ComponentGraph::new(),
        }
    }
}
//  public worksheet(tbApplet app, String fileName) {
//    super("TREEBAG " + versionNumber + " worksheet");
//    parent = app;
//    tabs = new commandTabs(this);
//    setDefaultCloseOperation(DO_NOTHING_ON_CLOSE);
//    enableEvents(AWTEvent.WINDOW_EVENT_MASK);
//    graph = new componentGraph();
//    GridBagLayout layout = new GridBagLayout();
//    GridBagConstraints constraints = new GridBagConstraints();
//    constraints.fill = GridBagConstraints.BOTH;
//    constraints.gridwidth = GridBagConstraints.REMAINDER;
//    constraints.weightx = constraints.weighty = 1.0;
//    layout.setConstraints(graph, constraints);
//    constraints.fill = GridBagConstraints.HORIZONTAL;
//    constraints.weighty = 0;
//    panel = new worksheetPanel();
//    layout.setConstraints(panel, constraints);
//    pane().setLayout(layout);
//    pane().add(graph);
//    pane().add(panel);
//    pane().add(popup);
//    graph.addMouseListener(this);
//    initMenus();
//    pack();
//    setVisible(true);
//    if (fileName != null) graph.addConfig(true, absolutePath(fileName), this);
//    //    graph.addConfig(true, System.getProperty("user.dir")+"/worksheet", this);
//  }
impl Worksheet {
    pub fn graph_mut(&mut self) -> &mut ComponentGraph {
        &mut self.graph
    }
} 
//  private String absolutePath(String fileName) {
//	if (parent != null) return parent.absolutePath(fileName);
//	else return new File(fileName).getAbsolutePath();
//  }
//  
//  private Container pane() { return getContentPane(); }
//  
//  private void initMenus() {
//    JMenuBar bar = new JMenuBar();
//    JMenu menu = new JMenu("Worksheet", true);
//    for (int i = 0; i < worksheetCommands.length; i++) {
//      if (worksheetCommands[i] == null) {
//        menu.addSeparator();
//        popup.addSeparator();
//      }
//      else {
//        JMenuItem item = new JMenuItem(worksheetCommands[i]);
//        item.addActionListener(this);
//        menu.add(item);
//        popup.add(new MenuItem(worksheetCommands[i]));
//      }
//    }
//    bar.add(menu);
//    popup.addSeparator();
//    popup.addSeparator();
//    menu = new JMenu("Component", true);
//    for (int i = 0; i < componentCommands.length; i++) {
//      if (worksheetCommands[i] == null) {
//        menu.addSeparator();
//        popup.addSeparator();
//      }
//      else {
//        JMenuItem item = new JMenuItem(componentCommands[i]);
//        item.addActionListener(this);
//        menu.add(item);
//        popup.add(new MenuItem(componentCommands[i]));
//      }
//    }
//    bar.add(menu);
//    setJMenuBar(bar);
//    popup.addActionListener(this);
//    bar.setVisible(true);
//  }
//  
//  public void setReadMe(String text) {
//    panel.setReadMe(text);
//  }
//  
//  protected void processWindowEvent(WindowEvent e) {
//    if (e.getID() == WindowEvent.WINDOW_CLOSING && graph.confirm()) exit();
//  }
//  
//  private void exit() {
//	if (parent != null) parent.destroy();
//    else System.exit(0);
//  }
//    	
//  public void dispose() {
//    	graph.clear();
//    super.dispose();
//    if (tabs != null) {
//    	  tabs.dispose();
//    	  tabs = null;
//    }
//  }
//  
//  public void actionPerformed(ActionEvent e) {
//    String com = e.getActionCommand();
//    if (quit.equals(com) && graph.confirm()) exit();
//    else if (deleteAll.equals(com)) {
//      graph.clear();
//      panel.setReadMe(null);
//    }
//    else if (addComp.equals(com)) {
//      if (e.getSource() == popup) graph.newNode(popedUpAt);
//      else graph.newNode(new Point(graph.getSize().width/2, graph.getSize().height/2));
//    }
//    else if (createComp.equals(com)) {
//      if (e.getSource() == popup) new editor(popedUpAt,graph);
//      else new editor(new Point(graph.getSize().width/2, graph.getSize().height/2),
//                        graph);
//    }
//    else if (save.equals(com)) graph.saveConfig();
//    else if (add.equals(com) || load.equals(com)) {
//      graph.addConfig(load.equals(com), this);
//    }
//  }
//  
//  public void mouseClicked(MouseEvent e) {
//  }
//  
//  public void mousePressed(MouseEvent e) {
//    popedUpAt = e.getPoint();
//    if (e.isAltDown() || e.isMetaDown()) {
//      popup.show(graph, popedUpAt.x, popedUpAt.y);
//    }
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
//  
//  private class worksheetPanel extends JPanel implements ActionListener {
//  
//    /**
//	 * 
//	 */
//	private static final long serialVersionUID = 4784246388249456854L;
//	private JTextField progress;
//    private GridBagLayout grid = new GridBagLayout();
//    private GridBagConstraints constr = new GridBagConstraints();
//    private Button readmeButton = new Button("ReadMe");
//    private boolean readMeAvailable = false;
//    private JFrame readMeFrame = null;
//  
//    public worksheetPanel() {
//      super();
//      setLayout(grid);
//      setBorder(BorderFactory.createEtchedBorder());
//      progress = new JTextField();
//      progress.setEditable(false);
//      constr.insets = new Insets(6,6,6,6);
//      constr.weightx = 1;
//      constr.anchor = GridBagConstraints.WEST;
//      grid.setConstraints(progress, constr);
//      add(progress);
//      graph.setProgressField(progress);
//      constr.anchor = GridBagConstraints.EAST;
//      grid.setConstraints(readmeButton, constr);
//      readmeButton.addActionListener(this);
//    }
//    
//    private int cols, rows;
//    
//    private void size(String s) {
//      int tmpCols = 0;
//      rows = cols = 1;
//      for (int i = 0; i < s.length(); i++) {
//        if (s.charAt(i) != '\n') {
//          tmpCols++;
//          if (tmpCols > cols) cols = tmpCols;
//        }
//        else {
//          rows++;
//          tmpCols = 0;
//        }
//      }
//    }
//    
//    public void setReadMe(String readMe) {
//      if (readMeAvailable) {
//        remove(readmeButton);
//        readMeFrame.dispose();
//      }
//      readMeAvailable = readMe != null;
//      if (readMeAvailable) {
//        readMeFrame = new JFrame("ReadMe");
//        size(readMe);
//        JTextArea text = new JTextArea(readMe,rows+1,cols * 2 / 3);
//        text.setLineWrap(true);
//        text.setWrapStyleWord(true);
//        text.setEditable(false);
//        text.setMargin(new Insets(5,7,5,7));
//        readMeFrame.getContentPane().add(new JScrollPane(text));
//        readMeFrame.pack();
//        add(readmeButton);
//      }
//    }
//    
//    public void actionPerformed(ActionEvent e) {
//      readMeFrame.show();
//    }
//    
//    public void dispose() {
//      if (readMeAvailable) readMeFrame.dispose();
//      panel.dispose();
//    }
//    
//  }
//  
//  public class commandTabs extends JFrame {
//	  
//	/** 
//   * 
//	 */
//	private static final long serialVersionUID = 3239816098138206611L;
//	private Vector shownComponents = new Vector(5);
//	private boolean unused = true;
//	private worksheet myWorksheet;
//	  
//	public commandTabs(worksheet ws) {
//      super("Control panes");
//      myWorksheet = ws;
//      getContentPane().setLayout(new BoxLayout(getContentPane(),BoxLayout.Y_AXIS));
//	}
//	
//	public void addTab(String name, JComponent c) {
//    getContentPane().add(Box.createVerticalStrut(10));
//	  shownComponents.add(c);
//    getContentPane().add(c);
//	  if (shownComponents.size()==1 && unused) {
//         unused = false;
//         Point loc = myWorksheet.getLocationOnScreen();
//         Dimension size = myWorksheet.getSize();
//         setLocation(loc.x + size.width, loc.y);
//	  }
//    pack();
//    setVisible(true);
//    requestFocusInWindow(); // Stupid, but makes it work slightly better on Mac OS X
//	}
//	
//	public boolean isShown(Component c) {
//	  return shownComponents.contains(c);
//	}
//	
//	public void removeTab(JComponent c) {
//	  int i = shownComponents.indexOf(c);
//	  if (i>=0) {
//	    shownComponents.remove(i);
//      getContentPane().remove(2*i+1);
//      getContentPane().remove(2*i);
//	    pack();
//	    if (shownComponents.size()==0) setVisible(false);
//      else requestFocusInWindow(); // Stupid, but makes it work slightly better on Mac OS X
//	  }
//	}
//  }
//
//
//}
//
