extern crate regex;

use self::regex::Regex;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    ShiftStarted(u16),
    FellAsleep,
    WokeUp
}

impl Event {
    pub fn from_string(event: &String) -> Event {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(?P<guard>\d+)").unwrap();
        }

        if event.contains("wakes up") {
            Event::WokeUp
        } else if event.contains("falls asleep") {
            Event::FellAsleep
        } else {
            let captures = RE.captures(event).unwrap();
            Event::ShiftStarted(captures["guard"].parse().unwrap())
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let string: String = match self {
            Event::ShiftStarted(guard) => format!("Guard #{} begins shift", guard),
            Event::FellAsleep => "falls asleep".to_owned(),
            Event::WokeUp => "wakes up".to_owned()
        };

        write!(formatter, "{}", string)
    }
}