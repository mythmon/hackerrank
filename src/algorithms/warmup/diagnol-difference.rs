use std::io;
use std::iter::Iterator;

fn read_number() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_array() -> Vec<i32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().split(' ').map(|s| { s.parse().unwrap() }).collect()
}

fn main() {
    let count = read_number();
    let mut matrix: Vec<Vec<i32>> = vec![];
    for _ in 0..count {
        matrix.push(read_array());
    }

    let mut left_diag = 0;
    let mut right_diag = 0;

    for i in 0usize..count {
        left_diag += matrix[i][i];
        right_diag += matrix[i][count - i - 1];
    }

    let diff = left_diag - right_diag;

    println!("{}", diff.abs());
}
