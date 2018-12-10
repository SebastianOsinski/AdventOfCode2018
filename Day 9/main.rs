fn main() {
    // run_game(9, 25);
    // run_game(10, 1618);
    // run_game(13, 7999);
    // run_game(17, 1104);
    // run_game(21, 6111);
    // run_game(30, 5807);
    run_game(466, 71436);
    run_game(466, 7143600);
}

fn run_game(players_count: usize, last_marble_value: u64) {
    let mut player_scores: Vec<u64> = vec![0; players_count];

    let mut marbles: Vec<u64> = Vec::with_capacity(last_marble_value as usize);
    marbles.push(0);

    let mut new_marble = 0u64;
    let mut current_player = 0usize;
    let mut current_marble_index = 0usize;

    // println!("[-] (0)");
    loop {
        new_marble += 1;

        if new_marble > last_marble_value {
            break
        }

        current_player = (current_player % players_count) + 1;

        if new_marble % 23 == 0 {
            player_scores[current_player - 1] += new_marble;

            let marble_to_remove_index = modulo(current_marble_index as isize - 7, marbles.len() as isize) as usize;
            player_scores[current_player - 1] += marbles.remove(marble_to_remove_index);

            current_marble_index = if marble_to_remove_index >= marbles.len() { 0 } else { marble_to_remove_index }; 
        } else {
            if current_marble_index + 2 > marbles.len() {
                current_marble_index = 1;
            } else {
                current_marble_index += 2;
            }

            marbles.insert(current_marble_index, new_marble);
        }

        // print_game_state(&current_player, &marbles, &current_marble_index);
    }

    println!("{} players; last marble is worth {} points; high score is {}", players_count, last_marble_value, player_scores.iter().max().unwrap());
}

fn print_game_state(current_player: &usize, marbles: &Vec<u64>, current_marble_index: &usize) {
    let marbles_strings: Vec<String> = marbles.iter().enumerate().map( |(index, marble)| {
        if index == *current_marble_index { format!("({})", marble) } else { format!("{}", marble) }
    }).collect();

    let marbles_string = marbles_strings.join(" ");
    
    println!("[{}] {}", current_player, marbles_string);
}

fn modulo(a: isize, b: isize) -> isize {
    ((a  % b) + b) % b
}