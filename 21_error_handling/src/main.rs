use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    open_a_file_with_match("hello_match.txt");
    open_a_file_with_unwrap_or_else("hello_unwrap_or_else.txt");
    // open_a_file_with_expect("hello_expect.txt"); // Will crash
    // open_a_file_with_unwrap("hello_unwrap.txt"); // Will crash
    // read_username_from_file("username.txt").unwrap(); // Will crash
    // read_username_from_file_question_mark("username.txt").unwrap(); // Will crash

    match last_char_of_first_line("eofigeogiez\n zfheogbeogjbze\n") {
        Some(c) => println!("last character of first line is : {}", c),
        None => println!("Text is empty"),
    }
    match last_char_of_first_line("") {
        Some(c) => println!("last character of first line is : {}", c),
        None => println!("Text is empty"),
    }
}

fn open_a_file_with_match(filepath: &str) {
    let f = File::open(filepath);

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filepath) {
                Ok(file) => file,
                Err(err) => panic!("{:?}", err),
            },
            other_error => {
                panic!("Error opening the file : {:?}", other_error)
            }
        },
    };
}
fn open_a_file_with_unwrap_or_else(filepath: &str) {
    let _f = File::open(filepath).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filepath).unwrap_or_else(|error| {
                panic!("Error creating the file : {:?}", error);
            })
        } else {
            panic!("Error opening the file : {:?}", error);
        }
    });
}
fn open_a_file_with_unwrap(filepath: &str) {
    // Will throw an error if no file named filepath
    let _f = File::open(filepath).unwrap();
}
fn open_a_file_with_expect(filepath: &str) {
    // Will throw an error if no file named filepath
    let _f = File::open(filepath).expect("The file set as input is not found");
}

fn read_username_from_file(filepath: &str) -> Result<String, io::Error> {
    let file = File::open(filepath);

    let mut file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn read_username_from_file_question_mark(filepath: &str) -> Result<String, io::Error> {
    let mut file = File::open(filepath)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
