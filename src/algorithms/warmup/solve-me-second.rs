use std::io;

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_number_pair() -> (i32, i32) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let v: Vec<&str> = line.trim().split(' ').collect();
    (v[0].parse().unwrap(), v[1].parse().unwrap())
}

fn main() {
    let count = read_number();
    for _ in 0..count {
        let (a, b) = read_number_pair();
        println!("{}", a + b);
    }
}
