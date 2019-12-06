#[cfg(test)]
mod tests {
    use super::{run_program, get_opcode, get_parameter_mode};

    #[test]
    fn test_run_program() {
        assert_eq!(vec![2,0,0,0,99], run_program(vec![1,0,0,0,99]));
        assert_eq!(vec![2,3,0,6,99], run_program(vec![2,3,0,3,99]));
        assert_eq!(vec![2,4,4,5,99,9801], run_program(vec![2,4,4,5,99,0]));
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], run_program(vec![1,1,1,4,99,5,6,0,99]));
    }

    #[test]
    fn test_get_opcode() {
        assert_eq!(2, get_opcode(1002));
        assert_eq!(12, get_opcode(1012));
    }

    #[test]
    fn test_get_parameter_mode() {
        // From example
        assert_eq!(0, get_parameter_mode(1002, 1));
        assert_eq!(1, get_parameter_mode(1002, 2));
        assert_eq!(0, get_parameter_mode(1002, 3));
        // Additional
        assert_eq!(1, get_parameter_mode(10102, 1));
        assert_eq!(0, get_parameter_mode(10102, 2));
        assert_eq!(1, get_parameter_mode(10102, 3));
    }

    #[test]
    fn test_run_program_with_immediate_mode() {
        assert_eq!(vec![1002,4,3,4,99], run_program(vec![1002,4,3,4,33]));
    }
}

fn get_opcode(instruction: i64) -> i64 {
    instruction % 100
}

fn get_parameter_mode(instruction: i64, parameter: u32) -> usize {
    assert!(parameter > 0 && parameter < 4);
    ((instruction % 10_i64.pow(parameter + 2)) / 10_i64.pow(parameter + 1)) as usize
}

fn get_value(program: &Vec<i64>, index: usize, mode: usize) -> i64 {
    match mode {
        0 => program[program[index] as usize],
        1 => program[index],
        _ => panic!("Invalid parameter mode")
    }
}

pub fn run_program(mut program: Vec<i64>) -> Vec<i64> {
    let mut i = 0;

    loop {
        let instruction = program[i];
        match get_opcode(instruction) {
            1 => {
                let mut_i = program[i+3] as usize;
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1));
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2));
                program[mut_i] = first_value + second_value;
            },
            2 => {
                let mut_i = program[i+3] as usize;
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1));
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2));
                program[mut_i] = first_value * second_value;
            },
            99 => break,
            _ => panic!("Invalid opcode")
        }
        i += 4;
    }
    program
}