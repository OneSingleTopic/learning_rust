use blog::oop;
use blog::rust_like;

fn main() {
    oop_demo();
    rust_like_demo();
}

fn rust_like_demo() {
    let mut post = rust_like::Post::new();

    post.add_text("I ate a tortilla for lunch today !");

    let post = post.request_review();

    let post = post.approve();

    let mut post = post.reject();
    post.add_text(" And it was cool !");

    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();
    assert_eq!(
        "I ate a tortilla for lunch today ! And it was cool !",
        post.content()
    );
}

fn oop_demo() {
    let mut post = oop::Post::new();

    post.add_text("I ate a tortilla for lunch today !");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text(" You know I like tortilla !");
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.reject();
    post.add_text(" And it was cool !");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    post.add_text(" So cool !");
    assert_eq!(
        "I ate a tortilla for lunch today ! And it was cool !",
        post.content()
    );
}
