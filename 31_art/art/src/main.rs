use art::NamedColor;

fn main() {
    println!(
        "{}",
        art::mix(art::PrimaryColor::Red, art::PrimaryColor::Blue).name()
    );
    println!(
        "{}",
        art::mix(art::PrimaryColor::Yellow, art::PrimaryColor::Yellow).name()
    );

    println!("With NamedColor {}", art::SecondaryColor::Brown.name());
}
