#[derive(Debug)]
enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrType::V4(127, 0, 0, 1);

    let loopback = IpAddrType::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let a: Option<u32> = Some(5);
    let b: u32 = 1;

    println!("{}", a + b);
}
