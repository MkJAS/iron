use std::io;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed tor read input.");
    println!("Input: {input}");


}
