const NUMBER_OF_RECIPES_TO_SKIP: usize = 147061;

fn main() {
    let mut scores: Vec<u8> = Vec::with_capacity(NUMBER_OF_RECIPES_TO_SKIP + 10);
    let mut first_elf_index = 0;
    let mut second_elf_index = 1;

    scores.push(3);
    scores.push(7);

    while scores.len() < NUMBER_OF_RECIPES_TO_SKIP + 10 {
        let first_elf_score = scores[first_elf_index];
        let second_elf_score = scores[second_elf_index];

        let sum = first_elf_score + second_elf_score;

        if sum < 10 {
            scores.push(sum);
        } else {
            scores.push(1);
            scores.push(sum % 10);
        }

        first_elf_index = (first_elf_index + first_elf_score as usize + 1) % scores.len();
        second_elf_index = (second_elf_index + second_elf_score as usize + 1) % scores.len();
    }

    for i in NUMBER_OF_RECIPES_TO_SKIP..(NUMBER_OF_RECIPES_TO_SKIP + 10) {
        print!("{}", scores[i]);
    }
    println!("");
}