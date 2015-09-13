use std::io;
use std::iter::repeat;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_num() -> i64 {
    let line = read_line();
    line.trim().parse().unwrap()
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let size = read_num();
    for n in 1..(size + 1) {
        let left: String = repeat(" ").take((size - n) as usize).collect();
        let right: String = repeat("#").take(n as usize).collect();
        println!("{}{}", left, right);
    }
}
