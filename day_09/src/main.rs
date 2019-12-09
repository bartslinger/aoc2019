use aoc;

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
    .expect("Error getting program");

    let original_program_state = aoc::intcode_computer::ProgramState {mem: aoc::intcode_computer::extended(original_program), pc: 0, rb: 0, halted: false};
    let result = aoc::intcode_computer::run_program(original_program_state.clone(), vec![1], vec![]);

    println!("Part 1: {}", result.2[0]);

    let result = aoc::intcode_computer::run_program(original_program_state.clone(), vec![2], vec![]);
    println!("Part 2: {}", result.2[0]);
}
