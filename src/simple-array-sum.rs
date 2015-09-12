use std::io;
use std::iter::Iterator;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_array() -> Vec<i32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().split(' ').map(|s| { s.parse().unwrap() }).collect()
}

fn main() {
    // Consume the count we don't use
    read_line();
    let v = read_array();
    let sum: i32 = v.iter().fold(0, |a, b| { a + b });
    println!("{}", sum);
}
