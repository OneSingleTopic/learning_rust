struct Point {
    x: u8,
    y: u8,
    z: u8,
}
fn main() {
    complex_destructuring();
    ignoring_param_pattern();
    setting_values();
    ignoring_rest_of_struct();
    match_guard();
    shadow_or_not_shadow();
    match_guard_and_or_pattern();
}

fn bind_pattern() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 3 };

    match msg {
        Message::Hello {
            id: id_variable @ 1..=6,
        } => println!("Found id in range {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range ")
        }
        Message::Hello { id } => println!("Found id out of range {}", id_variable),
    }
}

fn match_guard_and_or_pattern() {
    let x = 4;
    let y = false;

    // 4|5|6 = PATTERN / if y = MATCH GUARD
    // So here if (4 or 5 or 6) and y and not (4 or 5 or (6 and y))
    // Here should be no
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn shadow_or_not_shadow() {
    let x = Some(5);
    let y = 10;

    // Y is shadowed and therefore cannot be used
    match x {
        Some(50) => println!("x is Fifty"),
        Some(y) => println!("x is equal to {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // Here we are not shadowing y
    match x {
        Some(50) => println!("x is Fifty"),
        Some(n) if n == y => println!("x is equal y and is {}", y),
        _ => println!("Default case, x = {:?}", x),
    }
}
fn match_guard() {
    let x = Some(4);

    match x {
        Some(x) if x % 2 == 0 => println!("x is {} and even", x),
        Some(x) => println!("x is {} and odd", x),
        None => (),
    }
}
fn ignoring_rest_of_struct() {
    let origin = Point { x: 0, y: 20, z: 0 };

    match origin {
        Point { y, x, .. } => println!("The origin X Y is {} {}", x, y),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("First: {}, Last:{}", first, last)
        }
    }
}

fn not_used_variable() {
    let s = Some(String::from("Hello!"));

    // Cannot compile as we specify the string is moved in s
    // but never used anymore
    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

fn setting_values() {
    let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Cannot overwrite already custom value, try unsetting it before")
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn complex_destructuring() {
    let result = ((1, 2), Point { x: 3, y: 4, z: 10 });
    let ((a, b), Point { x, y, z }) = result;

    println!("a {}, b {}, x {}, y {}, z {}", a, b, x, y, z);
}

fn ignoring_param_pattern() {
    ignore_first_param(5, 12);
}

fn ignore_first_param(_: i32, y: i32) {
    println!("This function only use the value of y : {}", y);
}
