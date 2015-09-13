use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_array() -> Vec<i64> {
    let line = read_line();
    line.trim().split(' ').map(|s| { s.parse().unwrap() }).collect()
}

fn read_num() -> i64 {
    let line = read_line();
    line.trim().parse().unwrap()
}

fn main() {
    let v = read_num();
    // consumed un-used count
    read_line();
    let arr = read_array();
    for i in 0..arr.len() {
        if arr[i] == v {
            println!("{}", i);
            break;
        }
    }
}
