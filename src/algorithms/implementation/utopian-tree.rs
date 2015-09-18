use std::{io, str, fmt};

// https://www.hackerrank.com/challenges/utopian-tree

fn read_line<T>() -> T
    where T: str::FromStr + fmt::Debug,
          T::Err: fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn utopian_tree(cycles: i32) -> i32 {
    let mut height = 1;
    for i in 0..cycles {
        match i % 2 {
            0 => height *= 2,
            1 => height += 1,
            _ => unreachable!(),
        };
    }
    height
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let count: i32 = read_line();
    for _ in 0..count {
        let cycles = read_line();
        let height = utopian_tree(cycles);
        println!("{}", height);
    }
}

#[test]
fn test_sample() {
    assert_eq!(utopian_tree(0), 1);
    assert_eq!(utopian_tree(1), 2);
    assert_eq!(utopian_tree(4), 7);
}
