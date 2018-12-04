extern crate regex;

mod date;
mod event;
pub use self::date::Date;
pub use self::event::Event;

use self::regex::Regex;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    pub date: Date,
    pub event: Event
}

impl Entry {
    pub fn from_string(entry: &String) -> Entry {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x) # insignificant whitespace mode
                \[
                (?P<date>.+)
                \]
            ").unwrap();
        }

        let captures = RE.captures(entry).unwrap();

        Entry {
            date: Date::from_string(&captures["date"].to_owned()),
            event: Event::from_string(&entry)
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "[{}] {}",
            self.date, self.event
        )
    }
}