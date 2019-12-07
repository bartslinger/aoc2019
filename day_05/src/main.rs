use aoc;

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_answer() {
        let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");
        assert_eq!(13787043, part_one(original_program.clone()));
        assert_eq!(3892695, part_two(original_program.clone()));
    }

}

fn part_one(program: Vec<i64>) -> i64 {
    let output = aoc::intcode_computer::run_program(program, vec![1], vec![]);
    *output.2.last().unwrap()
}

fn part_two(program: Vec<i64>) -> i64 {
    let output = aoc::intcode_computer::run_program(program, vec![5], vec![]);
    *output.2.last().unwrap()
}

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

    println!("Part 1: {:?}", part_one(original_program.clone()));
    println!("Part 2: {:?}", part_two(original_program.clone()));
}
