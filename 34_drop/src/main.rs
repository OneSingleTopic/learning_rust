struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data : {}", self.data);
    }
}
fn main() {
    let custom_pointer = CustomSmartPointer {
        data: "Very Important Data".to_string(),
    };

    let another_custom_pointer = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(another_custom_pointer);
    println!("CustomSmartPointer dropped before the end of main.");
}
