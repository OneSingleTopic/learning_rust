#[macro_export]
macro_rules! vec_custom {
    ( $( $x: expr) *)  => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push ($x);

            )*
            temp_vec
        }
    };
    ( $( $x: expr); *)  => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push ($x);

            )*
            temp_vec
        }
    };

}

fn main() {
    let a = vec_custom![1 2 3];
    let b = vec_custom![1; 2; 3];
    println!("{:?}", a);
    println!("{:?}", b);
}
