enum TypeOfWord {
    Vowel,
    Consonnant,
}

fn main() {
    let text = String::from(
        "Language is the key to the heart of people. Language is the key to the heart of people.",
    );
    let mut pig_latin_sentences: Vec<String> = Vec::new();

    for sentence in text.split(".") {
        if !sentence.is_empty() {
            let mut pig_words: Vec<String> = Vec::new();
            for word in sentence.split_ascii_whitespace() {
                pig_words.push(transform_word_to_pig_latin(word));
            }
            pig_latin_sentences.push(format!(
                "{}{}",
                {
                    let sentence = pig_words.join(" ");
                    let mut sentence_chars = sentence.chars();
                    match sentence_chars.next() {
                        Some(letter) => {
                            letter.to_uppercase().collect::<String>() + sentence_chars.as_str()
                        }
                        None => String::new(),
                    }
                },
                "."
            ));
        }
    }
    let pig_latin_text = pig_latin_sentences.join(" ");
    println!("{}", pig_latin_text);
}

fn transform_word_to_pig_latin(word: &str) -> String {
    let mut word_chars = word.chars();
    let pig_latin_word = match word_chars.next() {
        Some(first) => match type_of_letter(&first) {
            TypeOfWord::Vowel => vowel_pig_latin(first, word_chars.as_str()),
            TypeOfWord::Consonnant => consonnant_pig_latin(first, word_chars.as_str()),
        },
        None => String::new(),
    };
    pig_latin_word
}

fn type_of_letter(letter: &char) -> TypeOfWord {
    match "aeiouy".to_string().find(*letter) {
        None => TypeOfWord::Consonnant,
        _ => TypeOfWord::Vowel,
    }
}

fn consonnant_pig_latin(first: char, rest_of_the_word: &str) -> String {
    format!(
        "{}{}{}",
        rest_of_the_word,
        first.to_ascii_lowercase(),
        "-hay"
    )
}
fn vowel_pig_latin(first: char, rest_of_the_word: &str) -> String {
    format!(
        "{}{}{}",
        first.to_ascii_lowercase(),
        rest_of_the_word,
        "-hay"
    )
}
