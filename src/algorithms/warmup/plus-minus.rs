#![feature(test)]
extern crate test;

use std::io;
use std::iter::Iterator;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_array() -> Vec<i64> {
    let line = read_line();
    line.trim().split(' ').map(|s| { s.parse().unwrap() }).collect()
}

fn counts(v: Vec<i64>) -> (f64, f64, f64) {
    let mut count_positive = 0;
    let mut count_negative = 0;
    let mut count_zero = 0;
    let count = v.len();

    for n in v {
        match n {
            n if n > 0 => count_positive += 1,
            n if n < 0 => count_negative += 1,
            n if n == 0 => count_zero += 1,
            _ => unreachable!(),
        }
    }

    (
        count_positive as f64 / count as f64,
        count_negative as f64 / count as f64,
        count_zero as f64 / count as f64,
    )
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    // consume unused count
    read_line();
    let (frac_positive, frac_negative, frac_zero) = counts(read_array());

    println!("{:.3}", frac_positive);
    println!("{:.3}", frac_negative);
    println!("{:.3}", frac_zero);
}

#[cfg(test)]
mod tests {
    use super::counts;

    #[test]
    fn sample() {
        let (pos, neg, zero) = counts(vec![-4, 3, -9, 0, 4, 1]);
        assert!(pos == 0.5);
        assert!(neg == 1. / 3.);
        assert!(zero == 1. / 6.);
    }
}
