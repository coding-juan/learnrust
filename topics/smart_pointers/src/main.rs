use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    // Box<T>
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Deref Trait
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox::new(x);
    assert_eq!(5, x);
    //assert_eq!(5, y); // This wont work because we are comparing 5 to a pointer, no the value
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *a);

    // Implicit Deref Coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Compiler turns &MyBox<String> into &String by calling deref
               //and in turn calls deref in &String which returns and &str
    hello(&(*m)[..]); // The other way of calling it, this is whay the compiler does under the hood
    // All the above is done at compile time so it doesn't affect runtime

    // Deref Coercion and Mutability
    let x = 5;
    let mut y = MyBox::new(x);
    println!("First value {}", *y); // Both this and line 30 are &mut T to &U;
    *y = 10;
    println!("Second value {}", *y);
    let z = MyBox::new(x);
    // *z = 11; // This won't work as rust doesn't allow &T to &mut U even though it is correct

    // Drop Trait
    println!("Drop trait starts here");
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    println!("CustomSmartPointers created");
    std::mem::drop(c);
    println!("CustomSmartPointer c dropped before the end of scope");

    // Rc<T> Reference Counted Smart Pointer
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(0, Rc::new(RcList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Weak<T> avoiding reference cycles
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// Box<T>
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Deref Trait
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implicit Deref Coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Deref Coerncion and Mutability
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Drop Trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'", self.data);
    }
}

// Rc<T> Reference Counted Smart Pointer
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil
}

// Weak<T> avoiding reference cycles
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}