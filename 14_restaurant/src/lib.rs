mod back_of_house;
mod front_of_house;

fn deliver_order() {}

mod Customer {
    use crate::back_of_house;
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Apetizer::Salad;
        let order2 = back_of_house::Apetizer::Soup;
        let mut meal = back_of_house::Breakfast::summer(String::from("Plain"));

        meal.toast = String::from("Rye");
        println!("I'd like {} toast please.", meal.toast);

        hosting::add_to_waitlist();
    }
}
