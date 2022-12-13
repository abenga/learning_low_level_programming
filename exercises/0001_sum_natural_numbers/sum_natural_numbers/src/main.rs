// The Modern C++ Challenge (Cap 1, Q1)
// Write a program that calculates and prints the sum of all the natural numbers divisible by
// either 3 or 5, up to a given limit entered by the user.

use std::io;

fn main() {
    println!("Please submit the limit");
    let mut limit = String::new();

    io::stdin().read_line(&mut limit).expect("Failed to read limit");

    let limit: i32 = limit.trim().parse().expect("Expected an integer!");

    let mut total = 0;
    for i in 1..(limit+1) {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }
    println!("total = {}", total);
}
