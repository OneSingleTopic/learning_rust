fn main() {
    println!(
        "{}",
        art::mix(art::PrimaryColor::Red, art::PrimaryColor::Blue).to_str()
    );
    println!(
        "{}",
        art::mix(art::PrimaryColor::Yellow, art::PrimaryColor::Yellow).to_str()
    );
}
