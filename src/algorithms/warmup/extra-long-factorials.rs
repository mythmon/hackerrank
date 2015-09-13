#![feature(test)]
extern crate test;
use test::Bencher;

use std::io;
use std::cmp::Eq;
use std::ops::{Add, Mul};
use std::fmt::{Display, Formatter, Error};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_num() -> u64 {
    let line = read_line();
    line.trim().parse().unwrap()
}

/// A naive, but workable BigInt
#[derive(Eq, PartialEq, Debug, Clone)]
struct BigInt {
    radix: u64,
    parts: Vec<u64>,
}

impl BigInt {
    fn from_str(digits: &str) -> BigInt {
        let mut parts: Vec<u64> = digits.chars().map(|c| {
            assert!(c.is_digit(10));
            c.to_digit(10).unwrap() as u64
        }).collect();
        let mut parts_rev = Vec::with_capacity(parts.len());
        for _ in 0..parts.len() {
            parts_rev.push(parts.pop().unwrap());
        }
        BigInt {
            radix: 10,
            parts: parts_rev,
        }
    }

    fn from_raw(digits: Vec<u64>) -> BigInt {
        BigInt { radix: 10, parts: digits }
    }

    fn from_u64(n: u64) -> BigInt {
        let radix = 10;
        let mut s;
        let mut c = n;
        let mut parts = vec![];
        while c > 0 {
            s = c;
            c = 0;
            while s >= radix {
                s -= radix;
                c += 1;
            }
            parts.push(s);
        }
        BigInt { radix: radix, parts: parts }
    }

    fn zero() -> BigInt {
        BigInt { radix: 10, parts: vec![0] }
    }

    fn one() -> BigInt {
        BigInt { radix: 10, parts: vec![1] }
    }
}

impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> BigInt {
        assert_eq!(self.radix, rhs.radix);
        let radix = self.radix;
        let mut a = self.parts;
        let mut b = rhs.parts;
        if a.len() > b.len() {
            let t = a;
            a = b;
            b = t;
        }
        // now b is the longer of the two

        let mut new_digits = vec![];
        let mut c = 0;
        let mut s;
        for i in 0..a.len() {
            let mut s = a[i] + b[i] + c;
            c = 0;
            while s >= radix {
                s -= radix;
                c += 1;
            }
            new_digits.push(s);
        }

        for i in a.len()..b.len() {
            let mut s = b[i] + c;
            c = 0;
            while s >= radix {
                s -= radix;
                c += 1;
            }
            new_digits.push(s);
        }

        while c > 0 {
            s = c;
            c = 0;
            while s >= radix {
                s -= radix;
                c += 1;
            }
            new_digits.push(s);
        }

        BigInt::from_raw(new_digits)
    }
}

impl Mul<u64> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: u64) -> BigInt {
        self * BigInt::from_u64(rhs)
    }
}

impl Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> BigInt {
        assert_eq!(self.radix, rhs.radix);
        let radix = self.radix;
        let mut partial_sums = vec![];
        let mut shift = 0;

        for d1 in self.parts.clone() {
            let mut c = 0;
            let mut acc = vec![];
            for _ in 0..shift {
                acc.push(0);
            }

            let mut s;
            for d2 in rhs.parts.clone() {
                s = d1 * d2 + c;
                c = 0;
                while s >= radix {
                    s -= radix;
                    c += 1;
                }
                acc.push(s);
            }

            while c > 0 {
                s = c;
                c = 0;
                while s >= radix {
                    s -= radix;
                    c += 1;
                }
                acc.push(s);
            }
            partial_sums.push(BigInt::from_raw(acc));
            shift += 1;
        }

        partial_sums.into_iter().fold(BigInt::zero(), |a: BigInt, b: BigInt| { a + b })
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let l = self.parts.len();
        for i in 0..l {
            try!(write!(f, "{}", self.parts[l - i - 1]));
        }
        Ok(())
    }
}

fn factorial(n: u64) -> BigInt {
    let mut acc = BigInt::one();
    for m in 2..(n + 1) {
        acc = acc * m;
    }
    acc
}

fn main() {
    let n = read_num();
    let f = factorial(n);
    println!("{}", f);
}

#[test]
fn test_big_int_add() {
    let a = BigInt::from_str("123");
    let b = BigInt::from_str("456");
    let expected = BigInt::from_str("579");
    let actual = a + b;
    assert_eq!(expected, actual);
}

#[bench]
fn test_big_int_add_bigger(bench: &mut Bencher) {
    bench.iter(|| {
        let a = BigInt::from_str("36893488147419103232");
        let b = BigInt::from_str("295147905179352825856");
        let expected = BigInt::from_str("332041393326771929088");
        let actual = a + b;
        assert_eq!(actual, expected);
        actual
    });
}

#[test]
fn test_big_int_add_carry_with_unequal_lengths() {
    let a = BigInt::from_str("1368");
    let b = BigInt::from_str("9120");
    let c = BigInt::from_str("45600");
    let expected = BigInt::from_str("56088");
    let actual = a + b + c;
    assert_eq!(actual, expected);
}

#[test]
fn test_bit_int_mul() {
    let a = BigInt::from_str("123");
    let b = BigInt::from_str("456");
    let expected = BigInt::from_str("56088");
    let actual = a * b;
    assert_eq!(actual, expected);
}

#[bench]
fn test_big_int_mult_bigger(bench: &mut Bencher) {
    bench.iter(|| {
        let a = BigInt::from_str("36893488147419103232");
        let b = BigInt::from_str("295147905179352825856");
        let expected = BigInt::from_str("10889035741470030830827987437816582766592");
        let actual = a * b;
        assert_eq!(actual, expected);
        actual
    });
}

#[test]
fn test_factorial() {
    let expected = BigInt::from_str("120");
    let actual = factorial(5);
    assert_eq!(actual, expected);
}

#[bench]
fn test_sample(bench: &mut Bencher) {
    bench.iter(|| {
        let expected = BigInt::from_str("15511210043330985984000000");
        let actual = factorial(25);
        assert_eq!(actual, expected);
    });
}

#[test]
fn test_display() {
    let expected = "12345".to_string();
    let actual = format!("{}", BigInt::from_str("12345"));
    assert_eq!(actual, expected);
}
