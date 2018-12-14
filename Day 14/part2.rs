fn main() {
    let score_sequence = vec![1, 4, 7, 0, 6, 1];

    let mut scores: Vec<u8> = Vec::new();
    let mut first_elf_index = 0;
    let mut second_elf_index = 1;

    scores.push(3);
    scores.push(7);

    loop {
        let first_elf_score = scores[first_elf_index];
        let second_elf_score = scores[second_elf_index];

        let sum = first_elf_score + second_elf_score;

        if sum < 10 {
            scores.push(sum);
        } else {
            scores.push(1);

            if is_match(&scores, &score_sequence) {
                break;
            }

            scores.push(sum % 10);
        }

        if is_match(&scores, &score_sequence) {
            break;
        }

        first_elf_index = (first_elf_index + first_elf_score as usize + 1) % scores.len();
        second_elf_index = (second_elf_index + second_elf_score as usize + 1) % scores.len();
    }

    println!("{}", scores.len() - score_sequence.len());
}

fn is_match(scores: &Vec<u8>, score_sequence: &Vec<u8>) -> bool {
    if scores.len() < score_sequence.len(){
        return false;
    }

    for i in 1..=score_sequence.len() {
        if scores[scores.len() - i] != score_sequence[score_sequence.len() - i] {
            return false;
        }
    }

    return true;
}

