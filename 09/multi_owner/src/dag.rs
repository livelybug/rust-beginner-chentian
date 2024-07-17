use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

#[derive(Debug)]
struct Node {
  id: usize,
  downstream: Option<Rc<Node>>
}

impl Node {
  pub fn new(id: usize) -> Self {
    Self {
      id,
      downstream: None,
    }
  }

  pub fn update_downstream(&mut self, downstream: Rc<Node>) {
    self.downstream = Some(downstream);
  }

  pub fn get_downstream(&self) -> Option<Rc<Node>> {
    self.downstream.as_ref().map(|v| v.clone())
  }
}

fn main() {
  let mut node1 = Node::new(1);
  let mut node2 = Node::new(2);
  let mut node3 = Node::new(3);
  let node4 = Node::new(4);

  println!("Hello, dag!");

  node3.update_downstream(Rc::new(node4));
  node1.update_downstream(Rc::new(node3));
  node2.update_downstream(node1.get_downstream().unwrap());
  println!("node1: {:?}, node2: {:?}", node1, node2);

  let node5 = Node::new(5);
  let node3 = node1.get_downstream().unwrap();
  // node3.update_downstream(Rc::new(node5));  // Need RefCell

  println!("node1: {:?}, node2: {:?}", node1, node2);

  let arr = vec![1]; 
  println!("{:?}", arr);
  std::thread::spawn(move || {
      println!("{:?} in thread", arr); 
    }
  );

  let str = "hello";
  println!("{:?}", &str);
  let str1 = Arc::new(str);
  let str2 = Arc::clone(&str1);
  std::thread::spawn(move || {
    println!("{:?} in thread", str2); 
  });
  thread::sleep(Duration::from_secs(1));
}