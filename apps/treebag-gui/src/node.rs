mod invisible;

pub use self::invisible::*;
//
//import java.awt.*;
//import java.util.*;
//import util.*;
pub trait Node {
    fn position(&self) -> Point;
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}


//public abstract class node extends Container {
//
//  public Point position = new Point();
//  
//  private observable obs = new observable();
//
//  public abstract Point getBorderPoint(Point seenFrom);
//  
//  protected componentGraph parent() {
//    return (componentGraph)getParent();
//  }
//
//  public void observeMotionBy(Observer o) {
//    obs.addObserver(o);
//  }
//  
//  public void deleteMotionObserver(Observer o) {
//    obs.deleteObserver(o);
//  }
//  
//  protected void setChanged() { obs.setChanged(); }
//
//  protected void notifyObservers() { obs.notifyObservers(edge.MOVEMENT); }
//  
//  public void inputEdgeEstablished(edge e) {}
//  
//  public void inputEdgeDeleted(edge e) {}
//  
//}
