use std::io;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};

#[derive(Clone)]
pub enum Time {
    AmPm { hour: u8, minute: u8, second: u8, am: bool },
    TwentyFour { hour: u8, minute: u8, second: u8 },
}

impl Time {
    pub fn new_ampm(hour: u8, minute: u8, second: u8, am: bool) -> Time {
        Time::AmPm { hour: hour, minute: minute, second: second, am: am }
    }

    pub fn new_twenty_four(hour: u8, minute: u8, second: u8) -> Time {
        Time::TwentyFour { hour: hour, minute: minute, second: second }
    }

    pub fn to_twenty_four(&self) -> Time {
        match *self {
            Time::AmPm { hour, minute, second, am } => {
                let new_hour = {
                    if am {
                        if hour == 12 { 0 } else { hour }
                    } else {
                        if hour == 12 { 12 } else { hour + 12 }
                    }
                };
                Time::new_twenty_four(new_hour, minute, second)
            },
            Time::TwentyFour { hour: _, minute: _, second: _ } => self.clone(),
        }
    }

    pub fn to_ampm(&self) -> Time {
        match *self {
            Time::AmPm { hour: _, minute: _, second: _, am: _ } => self.clone(),
            Time::TwentyFour { hour, minute, second } => {
                let new_hour = hour - if hour > 12 { 12 } else { 0 };
                let am = hour < 12;
                Time::new_ampm(new_hour, minute, second, am)
            },
        }
    }
}

impl FromStr for Time {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hour: u8 = s[0..2].parse().unwrap();
        let minute: u8 = s[3..5].parse().unwrap();
        let second: u8 = s[6..8].parse().unwrap();
        if s.len() > 8 {
            let am = s[8..10] == *"AM";
            Ok(Time::new_ampm(hour, minute, second, am))
        } else {
            Ok(Time::new_twenty_four(hour, minute, second))
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Time::AmPm { hour, minute, second, am } => {
                try!(write!(f, "{:0>2}:{:0>2}:{:0>2}{}",
                       hour, minute, second,
                       if am { "AM" } else { "PM" }));
            },
            Time::TwentyFour { hour, minute, second } => {
                try!(write!(f, "{:0>2}:{:0>2}:{:0>2}", hour, minute, second));
            },
        }
        Ok(())
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_time() -> Time {
    read_line().trim().parse().unwrap()
}

fn main() {
    let time = read_time();
    println!("{}", time.to_twenty_four());
}

#[cfg(test)]
mod tests {
    use super::Time;

    #[test]
    fn test_sample() {
        let t: Time = "07:05:45PM".parse().unwrap();
        let s = format!("{}", t.to_twenty_four());
        assert_eq!(s, "19:05:45");
    }

    #[test]
    fn test_case_1() {
        let t: Time = "12:40:22AM".parse().unwrap();
        let s = format!("{}", t.to_twenty_four());
        assert_eq!(s, "00:40:22");
    }
}
