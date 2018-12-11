const GRID_SERIAL_NUMBER: isize = 6878;
const SIZE: isize = 3;

fn main() {
    const MIN_COORD: isize = 1;
    const MAX_COORD: isize = 300;

    // println!("{}", power_level(122, 79, 57));
    // println!("{}", power_level(217, 196, 39));
    // println!("{}", power_level(101, 153, 71));

    let mut max_total_power_x: isize = 0;
    let mut max_total_power_y: isize = 0;
    let mut max_total_power = isize::min_value();

    for x in MIN_COORD..=(MAX_COORD + 1 - SIZE) {
        for y in MIN_COORD..=(MAX_COORD + 1 - SIZE) {
            let total_power = total_power_level(x, y, SIZE);

            if total_power > max_total_power {
                max_total_power = total_power;
                max_total_power_x = x;
                max_total_power_y = y;
            }
        }
    }

    println!("{}, {} = {}", max_total_power_x, max_total_power_y, max_total_power);
}

fn total_power_level(x: isize, y: isize, size: isize) -> isize {
    let mut sum = 0;

    for i in x..(x + size) {
        for k in y..(y + size) {
            sum += power_level(i, k, GRID_SERIAL_NUMBER);
        }
    }

    sum
}

fn power_level(x: isize, y: isize, grid_serial_number: isize) -> isize {
    let rack_id = x + 10;

    let power_level = rack_id * y;
    let power_level = power_level + grid_serial_number;
    let power_level = power_level * rack_id;
    let power_level = (power_level / 100) % 10;

    power_level - 5
}