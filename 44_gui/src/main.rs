use gui::{Button, Draw, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 50,
                label: "Main Button".to_string(),
            }),
            Box::new(SelectBox {
                width: 50,
                height: 50,
                option: true,
                label: "Select box 1".to_string(),
            }),
            Box::new(Button {
                width: 50,
                height: 50,
                label: "Secondary Button".to_string(),
            }),
        ],
    };

    screen.run();
}
