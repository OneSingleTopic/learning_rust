use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    t_shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_in_stock())
    }
    fn most_in_stock(&self) -> ShirtColor {
        let shirt_colors = vec![ShirtColor::Blue, ShirtColor::Red];
        let count = vec![0, 0];
        let mut color_stock: HashMap<_, _> =
            shirt_colors.into_iter().zip(count.into_iter()).collect();

        for shirt in &self.t_shirts {
            let color_stock_shirt = color_stock.entry(*shirt).or_insert(0);
            *color_stock_shirt += 1;
        }

        if color_stock.get(&ShirtColor::Red) > color_stock.get(&ShirtColor::Blue) {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        t_shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_0_pref = Some(ShirtColor::Red);
    let giveaway_0 = store.giveaway(user_0_pref);
    println!(
        "The user 0 prefered {:?} and is given a {:?} T-shirt ! ",
        user_0_pref, giveaway_0
    );

    let user_1_pref = None;
    let giveaway_0 = store.giveaway(user_1_pref);
    println!(
        "The user 1 had no preference and was given a {:?} T-shirt ! ",
        giveaway_0
    );
}
