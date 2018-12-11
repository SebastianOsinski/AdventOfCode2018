const MIN_COORD: usize = 1;
const MAX_COORD: usize = 300;
const GRID_SERIAL_NUMBER: usize = 6878;
const SIZE: usize = 3;

fn main() {
    let mut max_total_power_x: usize = 0;
    let mut max_total_power_y: usize = 0;
    let mut max_total_power_size: usize = 0;
    let mut max_total_power = isize::min_value();

    let mut power_levels = [[0; MAX_COORD]; MAX_COORD];
    
    for x in MIN_COORD..=MAX_COORD {
        for y in MIN_COORD..=MAX_COORD {
            power_levels[x - 1][y - 1] = power_level(x, y); 
        }
    }

    for size in 1..=300 {
        println!("SIZE: {}", size);
        for x in MIN_COORD..=(MAX_COORD + 1 - size) {
            for y in MIN_COORD..=(MAX_COORD + 1 - size) {
                let total_power = total_power_level(x, y, size, &power_levels);

                if total_power > max_total_power {
                    max_total_power = total_power;
                    max_total_power_x = x;
                    max_total_power_y = y;
                    max_total_power_size = size;
                }
            }
        }
    }

    println!("{},{},{} = {}", max_total_power_x, max_total_power_y, max_total_power_size, max_total_power);
}

fn total_power_level(x: usize, y: usize, size: usize, power_levels: &[[isize; MAX_COORD]]) -> isize {
    let mut sum = 0;

    for i in x..(x + size) {
        for k in y..(y + size) {
            sum += power_levels[i - 1][k - 1];
        }
    }

    sum
}

fn power_level(x: usize, y: usize) -> isize {
    let rack_id = (x as isize) + 10;

    let power_level = rack_id * y as isize;
    let power_level = power_level + GRID_SERIAL_NUMBER as isize;
    let power_level = power_level * rack_id;
    let power_level = (power_level / 100) % 10;

    power_level - 5
}