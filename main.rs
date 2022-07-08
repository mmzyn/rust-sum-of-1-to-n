// Write a program that asks the user for a number n and prints the sum of the numbers 1 to n

use std::io;

fn main() {
    
    println!("Please input your number:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    let sum = (int_input * (1+ int_input)) / 2;

    println!("The sum of 1 to your number is: {}", sum);
}
