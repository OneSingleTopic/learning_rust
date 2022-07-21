fn main() {
    let number: Option<i32> = Some(5);
    println!("{:?}", plus_one(number));
    println!("{:?}", plus_one(None));
}

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        Some(num) => Some(num + 1),
        None => None,
    }
}
