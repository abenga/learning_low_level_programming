
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
