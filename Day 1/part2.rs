use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(&input);
    
    let changes: Vec<i32> = input
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
        
    let changes = changes.iter().cycle();

    let mut reached_frequencies = HashSet::new();
    let mut frequency = 0;

    for change in changes {
        frequency += change;

        if reached_frequencies.contains(&frequency) {
            println!("{}", frequency);
            break;
        } else {
            reached_frequencies.insert(frequency);
        }
    }
}