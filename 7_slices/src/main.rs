fn main() {
    let hello_world = String::from("hello world");

    println!("{}", first_word(&hello_world));

    println!("{}", first_word("coucou toto"));

    println!("{}", second_word(&hello_world));

    println!("{}", second_word("coucou t titi"));

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(sentence: &str) -> &str {
    let sentences_bytes = sentence.chars();

    for (index, char_item) in sentences_bytes.enumerate() {
        if char_item == ' ' {
            return &sentence[..index];
        }
    }
    return &sentence[..];
}

fn second_word(sentence: &str) -> &str {
    let sentences_bytes = sentence.chars();

    let mut word_count = 0;
    let mut start_index = 0;
    for (index, char_item) in sentences_bytes.enumerate() {
        if char_item == ' ' {
            word_count += 1;
            if word_count == 2 {
                return &sentence[start_index + 1..index];
            }
            start_index = index;
        }
    }
    if word_count < 1 {
        return &sentence[0..0];
    }
    return &sentence[start_index + 1..];
}
