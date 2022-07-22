fn main() {
    let s = String::new();
    let s: String = "initial content".to_string();

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut coucou = String::from("coucou");

    coucou.push_str("_toto");
    println!("{}", coucou);

    let coucou = "coucou".to_string();
    let toto = "toto".to_string();

    let coucou_toto = coucou + &toto;

    println!("{}", coucou_toto);
    // println!("{}", coucou); // not possible as coucou_toto took the ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("with + {}", s);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("with format! {}", s);

    let hello = String::from("hello");

    let uppercase_hello = some_kind_of_uppercase_first_letter(&hello);
    let mut chars_hello = hello.chars();
    let next_result = chars_hello.next();
    println!("{:?}", next_result);
    println!("{:?}", chars_hello.as_str());

    println!("{}", uppercase_hello);
}
fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
