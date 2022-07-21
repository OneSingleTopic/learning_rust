struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = build_user(String::from("coucou"), String::from("toto"));

    let user2 = User {
        username: String::from("titi"),
        ..user
    };

    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user.email);
    println!("{}", user.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
