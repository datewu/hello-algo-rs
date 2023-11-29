//  cargo run --bin chapter3-link-list
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    // fn new(value: i32) -> Self {
    //     Node { value, next: None }
    // }

    fn insert(&mut self, value: i32) {
        let new_node = Node {
            value,
            next: self.next.take(),
        };
        self.next = Some(Rc::new(RefCell::new(new_node)));
    }

    fn remove_next(&mut self) {
        let next = self.next.clone();
        match next {
            None => (),
            Some(next) => {
                let next = next.borrow();
                self.next = next.next.clone();
            }
        }
    }

    fn access(&self, index: i32) -> Option<Rc<RefCell<Node>>> {
        if index < 1 {
            // return self.next.clone();
            return Some(Rc::new(RefCell::new(Node {
                value: self.value,
                next: self.next.clone(),
            })));
        }
        if let Some(node) = self.next.clone() {
            return node.borrow().access(index - 1);
        }
        None
    }

    fn find(&self, value: i32) -> i32 {
        if self.value == value {
            return 0;
        }
        if let Some(node) = self.next.clone() {
            let a = node.borrow().find(value);
            if a >= 0 {
                return a + 1;
            }
        }
        -1
    }
}

fn main() {
    let head = init_list();
    println!(
        "demo linked list {:?}, head value {}",
        head,
        head.borrow().value
    );
    head.borrow_mut().insert(10);
    println!("after insert{:?}", head);
    head.borrow().next.clone().unwrap().borrow_mut().insert(20);
    println!("after insert next: {:?}", head);
    head.borrow_mut().remove_next();
    println!("remove second node: {:?}", head);
    println!("access second node: {:?}", head.borrow().access(1));
    println!("access fouth node: {:?}", head.borrow().access(3));
    println!("access first node: {:?}", head.borrow().access(0));
    println!("access 100th node: {:?}", head.borrow().access(99));

    println!("find {} index: {}", 3, head.borrow().find(3));
    println!("find {} index: {}", 9, head.borrow().find(9));
    println!("find {} index: {}", 29, head.borrow().find(29));
    println!("find {} index: {}", 10, head.borrow().find(10));
    println!("find {} index: {}", 20, head.borrow().find(20));
    println!("find {} index: {}", 29, head.borrow().find(29));
    println!("find {} index: {}", 42, head.borrow().find(42));

    println!("access first node: {:?}", head.borrow().access(0).unwrap());
    println!("debug link list  : {:?}", head);
}

fn init_list() -> Rc<RefCell<Node>> {
    let n0 = Rc::new(RefCell::new(Node {
        value: 9,
        next: None,
    }));
    let n1 = Rc::new(RefCell::new(Node {
        value: 5,
        next: None,
    }));
    let n2 = Rc::new(RefCell::new(Node {
        value: 28,
        next: None,
    }));
    let n3 = Rc::new(RefCell::new(Node {
        value: 3,
        next: None,
    }));
    let n4 = Rc::new(RefCell::new(Node {
        value: 42,
        next: None,
    }));
    n0.borrow_mut().next = Some(n1.clone());
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());
    n3.borrow_mut().next = Some(n4.clone());
    n4.borrow_mut().next = None;
    n0
}
