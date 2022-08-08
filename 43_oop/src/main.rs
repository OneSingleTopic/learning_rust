use oop::AverageCollection;

fn main() {
    let mut average_collection = AverageCollection::new(vec![1, 2, 3]);

    println!("Initial average {}", average_collection.average());
    average_collection.add(10);
    println!("New average {}", average_collection.average());
    let _ = average_collection.pop();
    let _ = average_collection.pop();
    println!("Pop twice average {}", average_collection.average());
    let _ = average_collection.pop();
    let _ = average_collection.pop();
    println!("No element left {}", average_collection.average());
}
