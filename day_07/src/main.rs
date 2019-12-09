use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplifiers() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(43210, get_thrust(&program, vec![4,3,2,1,0]));

        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
        101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(54321, get_thrust(&program, vec![0,1,2,3,4]));

        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(65210, get_thrust(&program, vec![1,0,4,3,2]));
    }

    #[test]
    fn test_amplifiers_with_feedback() {
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
        27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        assert_eq!(139629729, get_thrust_with_feedback(&program, vec![9,8,7,6,5]));

        let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
        -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        assert_eq!(18216, get_thrust_with_feedback(&program, vec![9,7,8,5,6]));
    }

    #[test]
    fn test_puzzle_answers() {
        let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

        assert_eq!(366376, part_one(&original_program));
        assert_eq!(21596786, part_two(&original_program));
    }
}

fn get_thrust(program: &Vec<i64>, seq: Vec<i64>) -> i64 {

    let clean_state = aoc::intcode_computer::ProgramState {mem: program.clone(), pc: 0, rb: 0, halted: false};
    let result_a = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[0],0], vec![]);
    let result_b = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[1],result_a.2[0]], vec![]);
    let result_c = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[2],result_b.2[0]], vec![]);
    let result_d = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[3],result_c.2[0]], vec![]);
    let result_e = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[4],result_d.2[0]], vec![]);
    result_e.2[0]
}

fn get_thrust_with_feedback(program: &Vec<i64>, seq: Vec<i64>) -> i64 {
    let clean_state = aoc::intcode_computer::ProgramState {mem: program.clone(), pc: 0, rb: 0, halted: false};
    let mut amp_a = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[0]], vec![]);
    let mut amp_b = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[1]], vec![]);
    let mut amp_c = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[2]], vec![]);
    let mut amp_d = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[3]], vec![]);
    let mut amp_e = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[4]], vec![]);

    let mut input_a = 0;

    loop {
        if amp_a.0.halted && amp_b.0.halted && amp_c.0.halted && amp_d.0.halted && amp_e.0.halted {
            break;
        }
        amp_a = aoc::intcode_computer::run_program(amp_a.0, vec![input_a], vec![]);
        amp_b = aoc::intcode_computer::run_program(amp_b.0, vec![amp_a.2[0]], vec![]);
        amp_c = aoc::intcode_computer::run_program(amp_c.0, vec![amp_b.2[0]], vec![]);
        amp_d = aoc::intcode_computer::run_program(amp_d.0, vec![amp_c.2[0]], vec![]);
        amp_e = aoc::intcode_computer::run_program(amp_e.0, vec![amp_d.2[0]], vec![]);
        input_a = amp_e.2[0];
    }
    input_a
}

fn part_one(program: &Vec<i64>) -> i64 {
    (0..=4).permutations(5).map(|p| get_thrust(program, p)).max().unwrap()
}

fn part_two(program: &Vec<i64>) -> i64 {
    (5..=9).permutations(5).map(|p| get_thrust_with_feedback(program, p)).max().unwrap()
}

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

    println!("Part 1: {}", part_one(&original_program));
    println!("Part 2: {}", part_two(&original_program));
    
}