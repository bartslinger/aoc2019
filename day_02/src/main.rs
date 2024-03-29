use aoc;

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_answers() {
        let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

        assert_eq!(11590668, part_one(&original_program));
        assert_eq!(2254, part_two(&original_program, 19690720));
    }
}

fn part_one(original_program: &Vec<i64>) -> i64 {
    let mut program_state = aoc::intcode_computer::ProgramState {mem: original_program.clone(), pc: 0, rb: 0, halted: false};
        
    program_state.mem[1] = 12;
    program_state.mem[2] = 2;
    let output = aoc::intcode_computer::run_program(program_state, vec![], vec![]);
    output.0.mem[0]
}

fn part_two(original_program: &Vec<i64>, goal: i64) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program_state = aoc::intcode_computer::ProgramState {mem: original_program.clone(), pc: 0, rb: 0, halted: false};
            program_state.mem[1] = noun;
            program_state.mem[2] = verb;
            let output = aoc::intcode_computer::run_program(program_state, vec![], vec![]);
            if output.0.mem[0] == goal {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No valid combination found, cannot execute gravity assist!")
}

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

    println!("Part 1: {}", part_one(&original_program));
    println!("Part 2: {}", part_two(&original_program, 19690720));
}
