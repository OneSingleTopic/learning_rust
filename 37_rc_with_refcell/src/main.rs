use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(3));

    let cons_a = Rc::new(Cons(
        Rc::clone(&value),
        Rc::new(Cons(
            Rc::new(RefCell::new(4)),
            Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Nil))),
        )),
    ));

    let cons_b = Cons(Rc::new(RefCell::new(2)), Rc::clone(&cons_a));

    let cons_c = Cons(Rc::new(RefCell::new(1)), Rc::clone(&cons_a));

    *value.borrow_mut() += 10;

    println!("{:?}", cons_b);
    println!("{:?}", cons_c);
}
