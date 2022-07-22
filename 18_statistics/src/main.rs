use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut rng = rand::thread_rng();
    let list_size: usize = rng.gen_range(0..50);
    let mut integer_list: Vec<i32> = (0..list_size).map(|_| rng.gen_range(-10..=10)).collect();
    let (median, (mode, count_max)) = median_and_mode(&mut integer_list);

    match (median, mode) {
        (Some(median), Some(mode)) => println!(
            "The median is {} and the mode is {} with {} occurences",
            median, mode, count_max
        ),
        _ => println!("The list is empty, there is neither median nor mode"),
    };
}

fn median_and_mode(integer_list: &mut Vec<i32>) -> (Option<i32>, (Option<i32>, u32)) {
    integer_list.sort();
    println!("{:?}", integer_list);

    let median_index: usize = if integer_list.len() <= 1 {
        0
    } else {
        integer_list.len() / 2 - 1
    };

    let median = match integer_list.get(median_index) {
        Some(integer) => Some(*integer),
        None => None,
    };

    let mut counter = HashMap::new();
    for element in integer_list {
        let count = counter.entry(element).or_insert(0);
        *count += 1;
    }

    let mut mode: Option<i32> = Option::None;
    let mut count_max = 0;

    for (element, count) in counter {
        if count > count_max {
            mode = Some(*element);
            count_max = count;
        }
    }

    (median, (mode, count_max))
}
