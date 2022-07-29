fn main() {
    let input_list = vec![13, 24, 9000];

    let largest_element = largest_i32(&input_list);
    if let Some(number) = largest_element {
        println!("The largest number is {}", number);
    };

    let input_list: Vec<i32> = vec![];
    let largest_element = largest_i32(&input_list);
    match largest_element {
        Some(number) => println!("{}", number),
        None => println!("List is empty"),
    };

    let input_chars = vec!['a', 'g', 'd'];

    let largest_element = largest_char(&input_chars);
    match largest_element {
        Some(number) => println!("The largest char is {}", number),
        None => println!("List is empty"),
    };

    let largest_element = largest(&input_chars);
    match largest_element {
        Some(number) => println!("The largest element is {}", number),
        None => println!("List is empty"),
    };
}

fn largest_i32(input_list: &[i32]) -> Option<i32> {
    let mut largest_element = input_list.get(0)?;

    for number in input_list {
        if number > largest_element {
            largest_element = number;
        }
    }
    Some(*largest_element)
}
fn largest_char(input_list: &[char]) -> Option<char> {
    let mut largest_element = input_list.get(0)?;

    for number in input_list {
        if number > largest_element {
            largest_element = number;
        }
    }
    Some(*largest_element)
}

fn largest<T: PartialOrd + Copy>(input_list: &[T]) -> Option<&T> {
    let mut largest_element = input_list.get(0)?;

    for number in input_list {
        if *number > *largest_element {
            largest_element = number;
        }
    }
    Some(largest_element)
}
