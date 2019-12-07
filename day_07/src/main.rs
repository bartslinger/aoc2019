use std::cmp;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::{get_thrust, get_thrust_with_feedback};

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
}

fn get_thrust(program: &Vec<i64>, seq: Vec<i64>) -> i64 {

    let clean_state = aoc::intcode_computer::ProgramState {mem: program.clone(), pc: 0, halted: false};
    let result_a = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[0],0], vec![]);
    let result_b = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[1],result_a.2[0]], vec![]);
    let result_c = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[2],result_b.2[0]], vec![]);
    let result_d = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[3],result_c.2[0]], vec![]);
    let result_e = aoc::intcode_computer::run_program(clean_state.clone(), vec![seq[4],result_d.2[0]], vec![]);
    result_e.2[0]
}

fn get_thrust_with_feedback(program: &Vec<i64>, seq: Vec<i64>) -> i64 {
    let clean_state = aoc::intcode_computer::ProgramState {mem: program.clone(), pc: 0, halted: false};
    let mut state_a = clean_state.clone();
    let mut state_b = clean_state.clone();
    let mut state_c = clean_state.clone();
    let mut state_d = clean_state.clone();
    let mut state_e = clean_state.clone();

    // println!("{:?}", state_a);

    let mut input_a = 0;
    loop {
        // println!("input a: {}", input_a);
        let result_a = aoc::intcode_computer::run_program(state_a, vec![seq[0],input_a], vec![]);
        state_a = result_a.0;

        let result_b = aoc::intcode_computer::run_program(state_b, vec![seq[1],result_a.2[0]], vec![]);
        state_b = result_b.0;
        
        let result_c = aoc::intcode_computer::run_program(state_c, vec![seq[2],result_b.2[0]], vec![]);
        state_c = result_c.0;
        
        let result_d = aoc::intcode_computer::run_program(state_d, vec![seq[3],result_c.2[0]], vec![]);
        state_d = result_d.0;
        
        let result_e = aoc::intcode_computer::run_program(state_e, vec![seq[4],result_d.2[0]], vec![]);
        state_e = result_e.0;

        println!("out: {:?} {:?} {:?} {:?} {:?}", result_a.2, result_b.2, result_c.2, result_d.2, result_e.2);

        input_a = result_e.2[0];

        if state_a.halted &&
            state_b.halted &&
            state_c.halted &&
            state_d.halted &&
            state_e.halted {
                break;
            }
    }
    input_a
}

fn part_one(program: &Vec<i64>) -> i64 {
    let perms = (0..=4).permutations(5);
    
    let mut max_thrust = 0;
    for p in perms {
        max_thrust = cmp::max(max_thrust, get_thrust(program, p));
    }

    max_thrust
}

fn part_two(program: &Vec<i64>) -> i64 {
    let perms = (5..=9).permutations(5);
    
    let mut max_thrust = 0;
    for p in perms {
        println!("{:?}", p);
        max_thrust = cmp::max(max_thrust, get_thrust_with_feedback(program, p));
        break;
    }

    max_thrust
}

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

    println!("Part 1: {}", part_one(&original_program));
    println!("Part 2: {}", part_two(&original_program));
    
}