pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{} is now drawn", self.label);
    }
}
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: bool,
    pub label: String,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{} is now drawn", self.label);
    }
}
