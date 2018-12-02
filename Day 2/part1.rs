use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(&input);

    let ids = input.lines();
    let id_tests: Vec<(bool, bool)> = ids.map(|s| test_id(s.unwrap())).collect();

    let (count2, count3) = id_tests.iter().fold((0u32, 0u32), { |(acc_l, acc_r), (l, r)|
        (acc_l + *l as u32, acc_r + *r as u32)
    });

    println!("{}", count2 * count3);
}

fn test_id(id: String) -> (bool, bool) {
    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for c in id.chars() {
        char_counts
            .entry(c)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut has2 = false;
    let mut has3 = false;

    for value in char_counts.values() {
        if *value == 2 {
            has2 = true;
        }
        if *value == 3 {
            has3 = true;
        }
    }

    return (has2, has3);
}