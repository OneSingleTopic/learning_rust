use std::io;

fn main() {
    loop {
        
        let mut input_temperature = String::new();
        let mut target_unit = String::new();
        
        println!("Precise the target unit [F, C] :");

        io::stdin()
            .read_line(&mut target_unit)
            .expect("Failed to read unit");

        let target_unit: char = match target_unit.trim().parse() {
            Ok(character) => character, 
            Err(_) => continue,
        };
        if !(target_unit == 'F' || target_unit == 'C'){
            continue;
        }


        println!("Input temperature :");

        io::stdin()
            .read_line(&mut input_temperature)
            .expect("Failed to read temperature");
            
        let input_temperature: f64 = match input_temperature.trim().parse() {
            Ok(temperature) => temperature, 
            Err(_) => continue,
        };
        
        let result = if target_unit == 'F' {to_farenheit(input_temperature)} else {to_celsius(input_temperature)}; 
        
        println!("{} {}", result, target_unit);
    }

}

fn to_farenheit(celsius_temperature:f64) -> f64{
    (celsius_temperature * 1.8) + 32.
}

fn to_celsius(farenheit_temperature:f64) -> f64{
    (farenheit_temperature - 32.)/  1.8
}