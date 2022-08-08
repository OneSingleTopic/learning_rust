use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

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

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    use List::{Cons, Nil};

    let list = Box::new(Cons(3, Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil)))))));

    let mut x = 5;
    let y = &x;
    let z = Box::new(x);
    println!("The value of x is {}", x);
    println!("The value of what y is pointing (x) is {}", *y);
    x = 7;
    println!(
        "The value of what z is pointing (the copy of x) is still {} but x is now {}",
        *z, x
    );

    let my_box = MyBox::new(5);
    println!("MyBox contains {}", *my_box);

    hello("Rust");
    hello(&MyBox::new(String::from("Rust in MyBox")));
}
