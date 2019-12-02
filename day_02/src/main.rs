use aoc;

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn test_run_program() {
        assert_eq!(vec![2,0,0,0,99], run_program(vec![1,0,0,0,99]));
        assert_eq!(vec![2,3,0,6,99], run_program(vec![2,3,0,3,99]));
        assert_eq!(vec![2,4,4,5,99,9801], run_program(vec![2,4,4,5,99,0]));
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], run_program(vec![1,1,1,4,99,5,6,0,99]));
    }
}

fn run_program(mut program: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    loop {
        match program[i] {
            1 => {
                let mut_i = program[i+3];
                program[mut_i] = program[program[i+1]] + program[program[i+2]];
            },
            2 => {
                let mut_i = program[i+3];
                program[mut_i] = program[program[i+1]] * program[program[i+2]];
            },
            99 => break,
            _ => panic!("Invalid opcode")
        }
        i += 4;
    }
    program
}

fn part_one(original_program: &Vec<usize>) -> usize {
    let mut program = original_program.clone();
    program[1] = 12;
    program[2] = 2;
    let output = run_program(program);
    output[0]
}

fn part_two(original_program: &Vec<usize>, goal: usize) -> usize {
    for i in 0..100 {
        for j in 0..100 {
            let mut program = original_program.clone();
            program[1] = i;
            program[2] = j;
            let output = run_program(program);
            if output[0] == goal {
                return 100 * i + j;
            }
        }
    }
    panic!("No valid combination found, cannot execute gravity assist!")
}

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<usize>("data/input1")
        .expect("Error getting program");

    let goal = aoc::value_from_file::<usize>("data/input2")
        .expect("Error getting goal for part two");

    println!("Part 1: {}", part_one(&original_program));
    println!("Part 2: {}", part_two(&original_program, goal));
}
