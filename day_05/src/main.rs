use aoc;

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_answer() {
        let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");
        let original_program_state = aoc::intcode_computer::ProgramState {mem: original_program, pc: 0, rb: 0, halted: false};
        assert_eq!(13787043, part_one(original_program_state.clone()));
        assert_eq!(3892695, part_two(original_program_state.clone()));
    }

}

fn part_one(program: aoc::intcode_computer::ProgramState) -> i64 {
    let output = aoc::intcode_computer::run_program(program, vec![1], vec![]);
    *output.2.last().unwrap()
}

fn part_two(program: aoc::intcode_computer::ProgramState) -> i64 {
    let output = aoc::intcode_computer::run_program(program, vec![5], vec![]);
    *output.2.last().unwrap()
}

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

    let original_program_state = aoc::intcode_computer::ProgramState {mem: original_program, pc: 0, rb: 0, halted: false};

    println!("Part 1: {:?}", part_one(original_program_state.clone()));
    println!("Part 2: {:?}", part_two(original_program_state.clone()));
}
