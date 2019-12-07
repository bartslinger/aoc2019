use aoc;

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

    let part_one = aoc::intcode_computer::run_program(original_program.clone(), vec![1], vec![]);
    let part_two = aoc::intcode_computer::run_program(original_program.clone(), vec![5], vec![]);

    println!("Part 1: {:?}", part_one.2);
    println!("Part 2: {:?}", part_two.2);


}
