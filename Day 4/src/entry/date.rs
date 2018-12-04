extern crate regex;

use self::regex::Regex;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16
}

impl Date {
    pub fn from_string(date: &String) -> Date {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x) # insignificant whitespace mode
                (?P<year>\d+)
                -
                (?P<month>\d+)
                -
                (?P<day>\d+)
                \s
                (?P<hour>\d+)
                :
                (?P<minute>\d+)
            ").unwrap();
        }

        let captures = RE.captures(date).unwrap();

        Date {
            year: captures["year"].parse().unwrap(),
            month: captures["month"].parse().unwrap(),
            day: captures["day"].parse().unwrap(),
            hour: captures["hour"].parse().unwrap(),
            minute: captures["minute"].parse().unwrap()
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}-{}-{} {:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

