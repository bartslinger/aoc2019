#[cfg(test)]
mod tests {
    use super::{run_program, get_opcode, get_parameter_mode};

    #[test]
    fn test_run_program() {
        assert_eq!((vec![2,0,0,0,99], vec![], vec![]), run_program(vec![1,0,0,0,99], vec![], vec![]));
        assert_eq!((vec![2,3,0,6,99], vec![], vec![]), run_program(vec![2,3,0,3,99], vec![], vec![]));
        assert_eq!((vec![2,4,4,5,99,9801], vec![], vec![]), run_program(vec![2,4,4,5,99,0], vec![], vec![]));
        assert_eq!((vec![30,1,1,4,2,5,6,0,99], vec![], vec![]), run_program(vec![1,1,1,4,99,5,6,0,99], vec![], vec![]));
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
        assert_eq!((vec![1002,4,3,4,99], vec![], vec![]), run_program(vec![1002,4,3,4,33], vec![], vec![]));
        assert_eq!((vec![1101,100,-1,4,99], vec![], vec![]), run_program(vec![1101,100,-1,4,0], vec![], vec![]));
    }

    #[test]
    fn test_run_program_with_input_output() {
        assert_eq!((vec![43,0,4,0,99], vec![], vec![43]), run_program(vec![3,0,4,0,99], vec![43], vec![]));
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

pub fn run_program(mut program: Vec<i64>, mut inputs: Vec<i64>, mut outputs: Vec<i64>) -> (Vec<i64>,Vec<i64>,Vec<i64>) {
    let mut i = 0;

    loop {
        let instruction = program[i];
        match get_opcode(instruction) {
            1 => {
                let mut_i = program[i+3] as usize;
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1));
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2));
                program[mut_i] = first_value + second_value;
                i += 4;
            },
            2 => {
                let mut_i = program[i+3] as usize;
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1));
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2));
                program[mut_i] = first_value * second_value;
                i += 4;
            },
            3 => {
                let mut_i = program[i+1] as usize;
                program[mut_i] = inputs.remove(0);
                i += 2;
            }
            4 => {
                outputs.push(program[program[i+1] as usize]);
                i += 2;
            },
            5 => {
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1));
                if first_value != 0 {
                    i = get_value(&program, i+2, get_parameter_mode(instruction, 2)) as usize;
                } else {
                    i += 3;
                }
            },
            6 => {
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1));
                if first_value == 0 {
                    i = get_value(&program, i+2, get_parameter_mode(instruction, 2)) as usize;
                } else {
                    i += 3;
                }
            },
            7 => {
                let mut_i = program[i+3] as usize;
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1));
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2));
                if first_value < second_value {
                    program[mut_i] = 1;
                } else {
                    program[mut_i] = 0;
                }
                i += 4;
            },
            8 => {
                let mut_i = program[i+3] as usize;
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1));
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2));
                if first_value == second_value {
                    program[mut_i] = 1;
                } else {
                    program[mut_i] = 0;
                }
                i += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode")
        }
    }
    (program, inputs, outputs)
}