use std::fmt::Debug;

fn print_number<T:Debug>(number: T) {
    println!("Your number: {:?}", number);
}

fn main() {
    print_number(150);
}