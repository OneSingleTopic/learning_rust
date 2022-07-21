const FIRST_SENTENCE: &str = "On the twelfth day of Christmas, my true love sent to me";

const SENTENCES : [&str;12] = [
    "A partridge in a pear tree",
    "Two turtle doves", 
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    for verse_index in 0..SENTENCES.len()+1{
        println!("{FIRST_SENTENCE}");
        for sentence_index in (1 .. verse_index+1).rev(){
            println!("{}{}", SENTENCES[sentence_index], if sentence_index > 1 {","} else {" and"})
        }
        println!("{}.", SENTENCES[0]);
        println!("");
    }
}
