use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

use entry::Entry;
use entry::Event;
use guard::Guard;

pub fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(&input);

    let lines = input.lines();

    let mut entries: Vec<Entry> = lines
        .map(|s| Entry::from_string(&s.unwrap()))
        .collect();

    entries.sort();

    let mut current_guard_number = 0u16;
    let mut sleep_start = 0u16;
    let mut guards = HashMap::new();

    for entry in entries {
        match entry.event {
            Event::ShiftStarted(guard) => current_guard_number = guard,
            Event::FellAsleep => sleep_start = entry.date.minute,
            Event::WokeUp => {
                let sleep_stop = entry.date.minute;

                let mut guard = guards
                    .entry(current_guard_number)
                    .or_insert(Guard::new());

                guard.record_sleep(sleep_start, sleep_stop);    
            }
        }
    }

    let mut max_guard_number = 0u16;
    let mut max_total_sleep = 0u16;
    let mut max_guard = Guard::new();
    for (guard_number, guard) in guards {
        let total_sleep = guard.total_sleep();

        if total_sleep > max_total_sleep {
            max_total_sleep = total_sleep;
            max_guard_number = guard_number;
            max_guard = guard;
        }
    }

    let max_sleep_minute = max_guard.max_sleep_minute();
    println!("{} {}", max_guard_number, max_sleep_minute);
    println!("{}", max_guard_number * max_sleep_minute);
}