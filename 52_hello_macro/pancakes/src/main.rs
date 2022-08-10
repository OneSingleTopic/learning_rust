use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use print_macro::prt;

#[derive(HelloMacro)]
struct Pancakes;

pub fn impl_sql_macro() {}
fn main() {
    Pancakes::hello_macro();

    prt!(COUCOU, je fais (un test));
}
