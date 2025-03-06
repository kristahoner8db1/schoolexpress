  // This is a simple Rust program that returns the sum of two numbers.

fn main() {
    let x = 5;
    let y = 7;
    println!("The sum of {} and {} is {}", x, y, sum(x, y));
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}