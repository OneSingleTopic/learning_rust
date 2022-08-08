use std::vec;

fn main() {
    intricated_if_let();
    while_let_demo();
    detricating_parameter_patterns();
    litterals();
    option_problem();
    litterals_letters();
    destructuring_struct();
    destructuring_struct_with_litterals();
    destructuring_enum();
    destructuring_nested_struct_and_enum();
}
fn destructuring_enum() {
    enum Color {
        Rgb(u8, u8, u8),
        Hsv(u8, u8, u8),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    };

    let messages = vec![
        Message::ChangeColor(Color::Rgb(12, 250, 45)),
        Message::Write("Coucou".to_string()),
        Message::Quit,
        Message::Move { x: 30, y: 90 },
    ];
    for message in messages {
        match message {
            Message::Quit => println!("No destructuring as it is Quit variant"),
            Message::Move { x, y } => println!("Let us move to {}, {}", x, y),
            Message::Write(str) => println!("Write {}", str),
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to RGB {} {} {}", r, g, b)
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to HSV {} {} {}", h, s, v)
            }
        }
    }
}
fn destructuring_nested_struct_and_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}
fn destructuring_struct() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 4, y: 9 };

    let Point { x: a, y: b } = p;

    assert_eq!(4, a);
    assert_eq!(9, b);
    println!("X coordinate corresponds to {}", a);

    let Point { x, y } = p;

    assert_eq!(4, x);
    assert_eq!(9, y);
    println!("Y coordinate corresponds to {}", y);
}
fn destructuring_struct_with_litterals() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 9 };

    match p {
        Point { x, y: 0 } => println!("The point lies on the x axis at {}", x),
        Point { x: 0, y } => println!("The point lies on the y axis at {}", y),
        Point { x, y } => println!("The point is at {}, {}", x, y),
    }
}
fn option_problem() {
    let x = Some(5);
    let y = 3;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched {}", y),
        _ => println!("Default case {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
fn litterals() {
    let x = 3;

    match x {
        0 | 1 => println!("x is equal to zero or one"),
        2 => println!("x is equal to two"),
        0..=8 => println!("x is between zero and eight"),
        _ => println!("x does not follow any known behavior"),
    }
}
fn litterals_letters() {
    let x = 'n';

    match x {
        'a'..='n' => println!("x is early in the alphabet"),
        'o'..='z' => println!("x is late in the alphabet"),
        _ => println!("x not in the alphabet"),
    }
}
fn detricating_parameter_patterns() {
    let point = (5, 6, 10);
    print_coordinates(&point);
}
fn print_coordinates(&(x, _, z): &(i32, i32, i32)) {
    println!("The coordinates are {}, {}", x, z);
}
fn intricated_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday: bool = false;
    let age: Result<u32, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!(
            "Setting background as {} as it is your favorite color !",
            color
        );
    } else if is_tuesday {
        println!("Setting background as red as it is tuesday !");
    } else if let Ok(number) = age {
        if number > 30 {
            println!("Setting background as purple like an adult !");
        } else {
            println!("Setting background as orange because orange is for young !");
        }
    } else {
        println!("Setting background as black by default !");
    }
}

fn while_let_demo() {
    let mut vector = vec![1, 2, 3];

    while let Some(num) = vector.pop() {
        println!("Popped {}", num);
    }
}
