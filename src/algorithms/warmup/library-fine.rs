use std::io;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use std::cmp::{PartialOrd, Ord, Ordering};

#[derive(PartialEq, Eq, Ord)]
struct Date {
    day: i32,
    month: i32,
    year: i32,
}

impl Date {
    pub fn new(day: i32, month: i32, year: i32) -> Date {
        Date { day: day, month: month, year: year }
    }
}

impl FromStr for Date {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        assert_eq!(parts.len(), 3);
        let day: i32 = parts[0].parse().unwrap();
        let month: i32 = parts[1].parse().unwrap();
        let year: i32 = parts[2].parse().unwrap();
        Ok(Date::new(day, month, year))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(write!(f, "{} {} {}", self.day, self.month, self.year));
        Ok(())
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        let a = (self.year, self.month, self.day);
        let b = (other.year, other.month, other.day);
        Some(a.cmp(&b))
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_date() -> Date {
    read_line().trim().parse().unwrap()
}

fn calculate_fine(turned_in: Date, due: Date) -> i32 {
    if turned_in <= due {
        0
    } else if turned_in.year > due.year {
        10000
    } else if turned_in.month > due.month {
        500 * (turned_in.month - due.month)
    } else {
        15 * (turned_in.day - due.day)
    }
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let turned_in = read_date();
    let due = read_date();
    println!("{}", calculate_fine(turned_in, due));
}

#[cfg(test)]
mod tests {
    use super::{Date, calculate_fine};

    #[test]
    fn test_comparison() {
        let a = Date::new(12, 9, 2015);
        let b = Date::new(14, 9, 2015);
        let c = Date::new(14, 9, 2015);
        assert!(a < b);
        assert!(b > a);
        assert!(b == c);
        assert!(a != c);
    }

    #[test]
    fn test_sample() {
        let turned_in = Date::new(9, 6, 2015);
        let due = Date::new(6, 6, 2015);
        assert_eq!(calculate_fine(turned_in, due), 45);
    }
}
