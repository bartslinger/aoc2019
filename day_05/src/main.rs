use aoc;

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

    let output = aoc::intcode_computer::run_program(original_program, vec![1], vec![]);

    println!("Part 1: {:?}", output.2);

}
