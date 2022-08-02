#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Closures and functions
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    add_one_v3(2); // Must be called to indicate to the compiler the type
    add_one_v4(2); // Must be called to indicate to the compiler the type

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // Cannot compile because type was inferred by previous line

    let mut vector = vec![1, 2, 3];
    println!("Before defining closure : {:?}", vector);
    let mut borrow_mutability = || vector.push(4);
    borrow_mutability();
    let borrow_only = || println!("From within closure : {:?}", vector);
    println!("Before calling closure : {:?}", vector);
    borrow_only();
    println!("After calling closure : {:?}", vector);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
