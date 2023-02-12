use std::io;
fn main() {
    println!("Hi welcome to the guessing game!!");
    println!("Enter a number");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read Number");

    println!("You have guessed : {number}");
}
