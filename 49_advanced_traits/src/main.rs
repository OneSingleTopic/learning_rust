use advanced_traits::Point;
use advanced_traits::Wrapper;
use advanced_traits::{Animal, Dog, Human, Pilot, Wizard};
use advanced_traits::{Counter, CustomIterator};
use advanced_traits::{Couple, OutlinePrint};

fn main() {
    associated_traits();
    operator_overloading();
    method_overload();
    supertraits();
    new_type_pattern();
}
fn new_type_pattern() {
    let mut wrapper = Wrapper(vec![
        "Coucou".to_string(),
        "c'est".to_string(),
        "moi".to_string(),
    ]);

    wrapper.push(" !".to_string());

    println!("{}", wrapper);
}
fn supertraits() {
    let couple = Couple { a: 1, b: 19000 };
    couple.outline_print();
}
fn method_overload() {
    let human = Human;

    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);

    let dog = Dog;
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    println!(
        "But here every {} is called {}",
        <Dog as Animal>::baby_name(),
        Dog::baby_name()
    );
}
fn associated_traits() {
    let mut counter = Counter {
        data: vec![1, 2, 3, 4],
        string_data: vec!["COUCOU".to_string()],
    };

    if let (Some(num), Some(num_custom)) = (counter.next(), counter.custom_next() as Option<u32>) {
        println!("{}", num);
        println!("{}", num_custom);
    }
}

fn operator_overloading() {
    let point0 = Point { x: 0, y: 8 };
    let point1 = Point { x: 1, y: 2 };

    assert_eq!(point0 + point1, Point { x: 1, y: 10 });

    println!("{:?}", point0); // Possible because we add Copy (therefore Clone) trait to Point
}
