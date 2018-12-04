#[derive(Clone)]
pub struct Guard {
    pub sleep_counts: [u16; 60]
}

impl Guard {
    pub fn new() -> Guard {
        Guard {
            sleep_counts: [0; 60]
        }
    }

    pub fn record_sleep(&mut self, from: u16, to: u16) {
        for i in from..to {
            self.sleep_counts[i as usize] += 1;
        }
    }

    pub fn total_sleep(&self) -> u16 {
        self.sleep_counts.iter().fold(0, |acc, n| acc + n)
    }

    pub fn max_sleep_minute(&self) -> u16 {
        let mut max_index = 0;
        let mut max_value = 0;

        for (index, value) in self.sleep_counts.iter().enumerate() {
            if *value > max_value {
                max_index = index;
                max_value = *value;
            }
        }

        max_index as u16
    }
}