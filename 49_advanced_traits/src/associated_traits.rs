pub struct Counter {
    pub data: Vec<u32>,
    pub string_data: Vec<String>,
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}

// Implementation with associated type here, so we cannot redefined
// it with another associated type
// impl Iterator for Counter {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.string_data.pop()
//     }
// }

// But we can use the same trait to implement another structure with
// another associated type
pub struct StringCounter {
    data: Vec<String>,
}
impl Iterator for StringCounter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}

// We can by the way implement multiple type for the same trait on
// one single struct thanks to generic types but will expect user to
// supply the given time at every call

// We set here a default value for the generic type
pub trait CustomIterator<T = u32> {
    fn custom_next(&mut self) -> Option<T>;
}

impl CustomIterator for Counter {
    fn custom_next(&mut self) -> Option<u32> {
        self.data.pop()
    }
}

impl CustomIterator<String> for Counter {
    fn custom_next(&mut self) -> Option<String> {
        self.string_data.pop()
    }
}
