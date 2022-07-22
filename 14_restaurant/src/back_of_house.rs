pub enum Apetizer {
    Soup,
    Salad,
}
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
impl Breakfast {
    pub fn summer(toast: String) -> Breakfast {
        Breakfast {
            toast,
            seasonal_fruit: String::from("peach"),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {}
