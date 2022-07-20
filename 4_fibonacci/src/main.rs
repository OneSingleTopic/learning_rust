use std::io;

const F1:u64 = 1;
const MAX_FIB: usize = 80;

fn main() {
    let mut cache : [u64; MAX_FIB] = [0; MAX_FIB];
    loop {
        let mut input_n = String::new();

        println!("Enter a number lower than {MAX_FIB}");
        io::stdin()
            .read_line(&mut input_n)
            .expect("Failed to read line");

        let input_n : usize = match input_n.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        if input_n >= MAX_FIB{
            continue
        }
        
        let (result, _cache) = fibonacci(input_n, cache);
        cache = _cache;
        println!("{result}");
    }
}

fn fibonacci(input_n: usize, cache : [u64; MAX_FIB]) -> (u64, [u64; MAX_FIB]){
    let mut computed_fib = cache; 
    let mut first_element = {
        let mut first_element = 0;
        for element in 1..MAX_FIB{
            if computed_fib[element] == 0 {
                first_element = element;
                break;
            }
        }
        first_element
    };
    if first_element == 1 {
        computed_fib[first_element] = F1;
        first_element=2;
    }

    if first_element < input_n{
        for element in first_element..input_n+1{
            computed_fib[element] = computed_fib[element-2] + computed_fib[element-1]
        }
    } 
    (
        computed_fib[input_n], 
        computed_fib
    )
}