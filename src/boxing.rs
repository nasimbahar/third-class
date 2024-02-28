use std::fmt;

pub fn starting_point(){
  //  BasicExample();
    SecondExample();
   
}

fn BasicExample() {
    let boxed_integer = Box::new(10);
    println!("Boxed integer: {}", boxed_integer);

  
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LinkedList::Node(value, next) => {
                write!(f, "{} -> ", value)?;
                next.fmt(f) // Recursively format the next node
            },
            LinkedList::Nil => write!(f, "Nil"),
        }
    }
}
enum LinkedList {
    Node(i32, Box<LinkedList>),
    Nil,
}

use LinkedList::{Node, Nil};

fn SecondExample() {
    let list = LinkedList::Node(1, Box::new(LinkedList::Node(2, Box::new(LinkedList::Node(3, Box::new(LinkedList::Nil))))));
    // This creates a linked list: 1 -> 2 -> 3 -> Nil
    println!("{}", list);
   
}
