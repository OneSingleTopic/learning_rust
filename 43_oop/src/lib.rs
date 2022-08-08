pub struct AverageCollection {
    collection: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn new(collection: Vec<i32>) -> AverageCollection {
        let mut result = AverageCollection {
            collection,
            average: 0.0,
        };
        result.update_average();
        result
    }

    pub fn add(&mut self, new_element: i32) {
        self.collection.push(new_element);
        self.update_average();
    }

    pub fn pop(&mut self) -> Option<i32> {
        let pop_element = self.collection.pop();
        let pop_element = match pop_element {
            Some(x) => {
                self.update_average();
                Some(x)
            }
            None => None,
        };
        pop_element
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.collection.iter().sum();

        self.average = total as f64 / self.collection.len() as f64;
    }
}
