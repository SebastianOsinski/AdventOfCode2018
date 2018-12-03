use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use claim::Claim;

pub fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(&input);

    let lines = input.lines();

    let claims: Vec<Claim> = lines
        .map(|s| Claim::from_string(&s.unwrap()))
        .collect();

    'outer: for claim in &claims {
        for other_claim in &claims {
            if claim.id != other_claim.id && claim.overlaps(&other_claim) {
                continue 'outer;
            }
        }

        println!("{}", claim);
        break;
    }
}