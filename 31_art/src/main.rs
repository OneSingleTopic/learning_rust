fn main() {
    println!(
        "{:?}",
        art::mix(art::PrimaryColor::Red, art::PrimaryColor::Blue)
    );
    println!(
        "{:?}",
        art::mix(art::PrimaryColor::Yellow, art::PrimaryColor::Yellow)
    );
}
