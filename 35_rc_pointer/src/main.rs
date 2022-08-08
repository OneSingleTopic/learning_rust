use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let con_list_a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&con_list_a));
    let con_list_b = Cons(3, Rc::clone(&con_list_a));
    println!("count after creating b = {}", Rc::strong_count(&con_list_a));
    {
        let con_list_c = Cons(4, Rc::clone(&con_list_a));
        println!("count after creating c = {}", Rc::strong_count(&con_list_a));
    }

    println!(
        "count after releasing c = {}",
        Rc::strong_count(&con_list_a)
    );
}
