#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

#[derive(Debug)]
pub struct Couple {
    a: u32,
    b: u32,
}

impl Couple {
    pub fn new(a: u32, b: u32) -> Couple {
        Couple { a, b }
    }
}

fn main() {
    println!("The result from do_twice is {}", do_twice(add_one, 1));

    let list = vec![251, 5, 68];
    let list_of_strings: Vec<String> = list.iter().map(|x| x.to_string()).collect();
    let another_list_of_strings: Vec<String> = list.iter().map(ToString::to_string).collect();

    println!("{:?}", list_of_strings);
    println!("{:?}", another_list_of_strings);

    let status_list: Vec<Status> = (0u32..32).map(Status::Value).collect();
    println!("{:?}", status_list);

    let couple_list: Vec<Couple> = (0u32..20)
        .zip(20u32..40)
        .map(|(a, b)| Couple::new(a, b))
        .collect();

    println!("{:?}", couple_list);

    println!(
        "The return closure is {}",
        return_closure()(2, "COUCOU".to_string())
    );
}

fn return_closure() -> Box<dyn Fn(i32, String) -> i32> {
    Box::new(|a: i32, b: String| (a))
}
fn add_one(arg: i32) -> i32 {
    arg + 1
}

fn do_twice(function: fn(i32) -> i32, arg: i32) -> i32 {
    function(arg) + function(arg)
}
