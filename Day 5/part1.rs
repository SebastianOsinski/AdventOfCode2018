use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = File::open("input").unwrap();
    let mut input = BufReader::new(&input);

    let mut polimer = String::new();
    input.read_line(&mut polimer).expect("");

    let mut units: Vec<char> = polimer.chars().collect();

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
    
    println!("{}", units.len());
}