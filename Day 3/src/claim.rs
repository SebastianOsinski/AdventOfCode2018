use regex::Regex;
use std::fmt;

pub struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl fmt::Display for Claim {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "#{} @ {},{}: {}x{}",
            self.id, self.x, self.y, self.width, self.height
        )
    }
}

impl Claim {
    pub fn from_string(claim: &String) -> Claim {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x) # insignificant whitespace mode
                (?P<id>\d+)
                \s@\s
                (?P<x>\d+),(?P<y>\d+)
                :\s 
                (?P<width>\d+)x(?P<height>\d+)
            ").unwrap();
        }

        let captures = RE.captures(claim).unwrap();

        Claim {
            id: captures["id"].parse().unwrap(),
            x: captures["x"].parse().unwrap(),
            y: captures["y"].parse().unwrap(),
            width: captures["width"].parse().unwrap(),
            height: captures["height"].parse().unwrap(),
        }
    }
}
