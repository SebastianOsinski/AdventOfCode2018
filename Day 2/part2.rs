use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(&input);

    let ids: Vec<String> = input.lines().map(|s| s.unwrap()).collect();

    for lhs in &ids {
        for rhs in &ids {
            if diff(&lhs, &rhs) == 1 {
                println!("{}", same_chars(&lhs, &rhs));
                return;
            }
        }
    }
}

fn diff(lhs: &String, rhs: &String) -> u32 {
    return lhs.chars()
        .zip(rhs.chars())
        .fold(0, |acc, (l, r)| acc + (l != r) as u32);
}

fn same_chars(lhs: &String, rhs: &String) -> String {
    return lhs.chars()
        .zip(rhs.chars())
        .fold(String::new(), |acc, (l, r)| {
            if l == r {
                return acc + &l.to_string();
            } else {
                return acc;
            }
        })
}