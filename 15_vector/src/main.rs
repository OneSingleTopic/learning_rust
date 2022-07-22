fn main() {
    let mut my_list = vec![1, 2, 3, 4];

    let third = my_list[2];
    println!("{}", third);

    my_list.push(my_list.len() + 1);
    let third = my_list.get(2);
    println!("{:?}", third);

    // Modify the list so third cannot be split from its println with this bloc
    for i in &mut my_list {
        *i += 1;
        println!("print {}", i);
    }
    for i in &my_list {
        println!("print not mut  {}", i);
    }

    // if we remove the & before my_list we move the owner of it and cannot use it anymore after
    for i in my_list {
        println!("Move owner {}", i);
    }

    // println!("{:?}", my_list); // thus impossible
}
