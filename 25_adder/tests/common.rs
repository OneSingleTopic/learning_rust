use adder::shapes::rectangle::Rectangle;

pub fn build_rectangles() -> Vec<Rectangle> {
    vec![
        Rectangle {
            width: 100,
            height: 190,
        },
        Rectangle {
            width: 2,
            height: 2,
        },
        Rectangle {
            width: 10,
            height: 55,
        },
    ]
}
