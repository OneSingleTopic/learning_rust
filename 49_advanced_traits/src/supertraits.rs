use std::fmt;
pub struct Couple {
    pub a: i32,
    pub b: i32,
}

pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let len_str = self.to_string().len();
        println!("{}", "*".repeat(len_str + 4));
        println!("*{}*", " ".repeat(len_str + 2));
        println!("* {} *", self.to_string());
        println!("*{}*", " ".repeat(len_str + 2));
        println!("{}", "*".repeat(len_str + 4));
    }
}

impl fmt::Display for Couple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

impl OutlinePrint for Couple {}
