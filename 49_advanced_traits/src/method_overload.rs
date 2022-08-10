pub trait Wizard {
    fn fly(&self);
}
pub trait Pilot {
    fn fly(&self);
}

pub struct Human;
impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}
pub trait Animal {
    fn baby_name() -> String;
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
