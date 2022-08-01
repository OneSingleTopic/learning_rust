use adder;

mod common;

#[test]
fn integration_test() {
    assert_eq!(adder::add_two(2), 4)
}

#[test]
fn rectangle_integration_test() {
    let rectangles = common::build_rectangles();

    assert!(rectangles[0].can_hold(&rectangles[1]));
}
