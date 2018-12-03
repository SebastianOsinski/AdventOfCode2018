use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use claim::Claim;
use std::collections::HashMap;

pub fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(&input);

    let lines = input.lines();

    let claims = lines.map(|s| Claim::from_string(&s.unwrap()));

    let mut claimed_inches: HashMap<(u32, u32), u32> = HashMap::new();
    for claim in claims {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                claimed_inches
                    .entry((x, y))
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }

    let overlapping_inches_count = claimed_inches.values().filter(|c| **c > 1).count();

    println!("{}", overlapping_inches_count);
}
