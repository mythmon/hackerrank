use std::io;

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    println!("{}", read_number() + read_number());
}
