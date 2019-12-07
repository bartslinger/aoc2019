use std::cmp;

#[cfg(test)]
mod tests {
    use super::get_thrust;

    #[test]
    fn test_amplifiers() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(43210, get_thrust(program, vec![4,3,2,1,0]));

        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
        101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(54321, get_thrust(program, vec![0,1,2,3,4]));

        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(65210, get_thrust(program, vec![1,0,4,3,2]));
    }
}

fn get_thrust(program: Vec<i64>, seq: Vec<i64>) -> i64 {

    let result_a = aoc::intcode_computer::run_program(program, vec![seq[0],0], vec![]);
    let result_b = aoc::intcode_computer::run_program(result_a.0, vec![seq[1],result_a.2[0]], vec![]);
    let result_c = aoc::intcode_computer::run_program(result_b.0, vec![seq[2],result_b.2[0]], vec![]);
    let result_d = aoc::intcode_computer::run_program(result_c.0, vec![seq[3],result_c.2[0]], vec![]);
    let result_e = aoc::intcode_computer::run_program(result_d.0, vec![seq[4],result_d.2[0]], vec![]);
    result_e.2[0]
}

fn part_one(program: Vec<i64>) -> i64 {
    let perms = (0..=4).permutations(5);
    
    let mut max_thrust = 0;
    for p in perms {
        max_thrust = cmp::max(max_thrust, get_thrust(program.clone(), p));
    }

    max_thrust
}

use itertools::Itertools;

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
        .expect("Error getting program");

    println!("Part 1: {}", part_one(original_program.clone()));
    
}