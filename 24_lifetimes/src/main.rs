fn main() {
    let string_0 = String::from("abc");
    let result;
    let result_bis;
    {
        let string_1 = String::from("defghijklmnop");
        result = longest(string_0.as_str(), string_1.as_str());
        result_bis = longest_no_ref(string_0.as_str(), string_1.as_str());
        println!("{}", result);
    }
    println!("{}", result_bis);
}

fn longest<'a>(string_0: &'a str, string_1: &'a str) -> &'a str {
    if string_0.len() > string_1.len() {
        string_0
    } else {
        string_1
    }
}
fn longest_no_ref(string_0: &str, string_1: &str) -> String {
    if string_0.len() > string_1.len() {
        String::from(string_0)
    } else {
        String::from(string_1)
    }
}
