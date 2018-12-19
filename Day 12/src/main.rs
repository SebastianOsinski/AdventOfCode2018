use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let input = File::open("input").unwrap();
    let mut input = BufReader::new(&input);

    let initial_state = read_initial_state(&mut input);
    let rules = read_rules(&mut input);

    let number_of_iterations = 20;
    let padding_size = 2 * number_of_iterations;

    let mut pots = ".".repeat(padding_size);
    pots.push_str(&initial_state);
    pots.push_str(&(".".repeat(padding_size)));

    let pots_len = pots.len();
    for _ in 0..number_of_iterations {
        let mut new_pots = String::with_capacity(pots_len);

        for i in 0..pots_len {
            if i >= 2 && i < pots_len - 2 {
                let neighbors = &pots[(i - 2)..=(i + 2)];
                new_pots.push(*rules.get(&neighbors.to_string()).unwrap());
            } else {
                new_pots.push(pots.chars().nth(i).unwrap());
            }
        }

        pots = new_pots;
    }

    println!("{}", pots);

    let sum = pots
        .chars()
        .enumerate()
        .filter(|(i, c)| *c == '#')
        .fold(0, |sum, (i, _)| sum + i - padding_size);

    println!("{}", sum);
}

fn read_initial_state(input: &mut BufReader<&File>) -> String {
    let mut initial_state_line = String::new();

    input.read_line(&mut initial_state_line).unwrap();

    initial_state_line.split_whitespace().last().unwrap().to_string()
}

fn read_rules(input: &mut BufReader<&File>) -> HashMap<String, char> {
    input.read_line(&mut String::new()).unwrap(); // skip empty line

    let mut rules = HashMap::new();
    let lines = input.lines();

    for line in lines {
        let u_line = line.unwrap();
        let components: Vec<&str> = u_line.split_whitespace().collect();

        rules.insert(components[0].to_string(), components[2].to_string().chars().nth(0).unwrap());
    }

    rules
}