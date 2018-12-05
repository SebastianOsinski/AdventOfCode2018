use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = File::open("input").unwrap();
    let mut input = BufReader::new(&input);

    let mut polymer = String::new();
    input.read_line(&mut polymer).expect("");

    // part 1
    let length = length_after_reacting(&polymer);
    println!("{}", length);

    // part 2
    let mut min_length = usize::max_value();
    for unit in b'a' ..= b'z' {
        let stripped_polymer = strip_char_any_case(&polymer, unit as char);
        let length = length_after_reacting(&stripped_polymer);

        min_length = min_length.min(length);
    }

    println!("{}", min_length);
}

fn length_after_reacting(polymer: &String) -> usize {
    let mut units: Vec<char> = polymer.chars().collect();

    let mut index = 0usize;
    loop {
        if index as i64 > units.len() as i64 - 2 {
            break;
        }

        let unit1 = units.get(index).cloned().unwrap() as i16;
        let unit2 = units.get(index + 1).cloned().unwrap() as i16;

        if (unit1 - unit2).abs() == 32 {
            units.remove(index + 1);
            units.remove(index);

            index = 0.max(index as i64 - 1) as usize;
        } else {
            index += 1;
        }
    }

    units.len()
}

fn strip_char_any_case(original : &String, char_to_strip: char) -> String {
    let strip_value = char_to_strip as i16;

    original.chars().filter(|&c| {
        let c_value = c as i16;
        let diff = (c_value - strip_value).abs();
        diff != 0 && diff != 32
    }).collect()
}