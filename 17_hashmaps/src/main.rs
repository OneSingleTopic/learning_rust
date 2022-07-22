use std::collections::HashMap;

fn main() {
    score_match_counter();
    word_counter();
}

fn score_match_counter() {
    let teams = vec!["Blue".to_string(), "Red".to_string()];
    let scores = vec![2, 4];

    let mut score_match: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();
    println!("{:?}", score_match);

    // Hashmaps takes ownership

    let challenger = String::from("Green");

    score_match.insert(challenger, 0);

    let challenger = String::from("Green");
    println!("{:?}", score_match.get(&challenger));

    score_match.entry("Purple".to_string()).or_insert(5);
    score_match.entry("Green".to_string()).or_insert(5);

    for (team, score) in &score_match {
        println!("{} : {}", team, score);
    }
}

fn word_counter() {
    let sentence = "hello world coucou toto coucou";

    let mut word_count = HashMap::new();

    for word in sentence.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count)
}
