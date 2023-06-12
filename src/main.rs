// Creating a linked list queue
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone,  Debug)]
struct Node{
    value: i32,
    next: Option<Rc<RefCell<Node>>>
}
impl Node{
    pub fn new(value: i32)-> Rc<RefCell<Node>>{
        return Rc::new(RefCell::new(Node{
            value: value,
            next: None
        }))
    }
}
#[derive(Debug)]
struct LinkedList{
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub len: i64
}
impl LinkedList{
    pub fn create_empty()-> LinkedList{
        LinkedList{head: None, tail:None, len: 0}
    }
    pub fn append(&mut self, value: i32 ){
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(value) => value.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone())
        };
        self.len +=1;
        self.tail = Some(new_node);
    }
    pub fn pop(&mut self)-> Option<i32>{
        self.len -=1;
        self.head.take().map(|head|{
            if let Some(next) = head.borrow_mut().next.take(){
                self.head = Some(next);
            }
            else{
                self.tail.take();
            }
            Rc::try_unwrap(head).ok().expect("you fucked up").into_inner().value
        })
    }
}

fn main() {
    println!("Hello, world!");
    let mut linkedlist = LinkedList::create_empty();
    linkedlist.append(64_i32);

    println!("{:?}", linkedlist);
    linkedlist.pop();
    println!("{:?}", linkedlist);

}
