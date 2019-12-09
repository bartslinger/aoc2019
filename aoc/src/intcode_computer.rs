#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let program_state = ProgramState{mem: vec![1,0,0,0,99], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(vec![2,0,0,0,99], result.0.mem);

        let program_state = ProgramState{mem: vec![2,3,0,3,99], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(vec![2,3,0,6,99], result.0.mem);

        let program_state = ProgramState{mem: vec![2,4,4,5,99,0], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(vec![2,4,4,5,99,9801], result.0.mem);

        let program_state = ProgramState{mem: vec![1,1,1,4,99,5,6,0,99], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], result.0.mem);
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
        // Add some relative mode checks
        assert_eq!(2, get_parameter_mode(20202, 1));
        assert_eq!(0, get_parameter_mode(20202, 2));
        assert_eq!(2, get_parameter_mode(20202, 3));
    }

    #[test]
    fn test_run_program_with_immediate_mode() {
        let program_state = ProgramState{mem: vec![1002,4,3,4,33], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(vec![1002,4,3,4,99], result.0.mem);

        let program_state = ProgramState{mem: vec![1101,100,-1,4,0], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(vec![1101,100,-1,4,99], result.0.mem);
    }

    #[test]
    fn test_run_program_with_input_output() {
        let program_state = ProgramState{mem: vec![3,0,4,0,99], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![43], vec![]);
        assert_eq!(vec![43,0,4,0,99], result.0.mem);
        assert_eq!(vec![43], result.2);
    }

    #[test]
    fn test_adjust_relative_base() {
        let program_state = ProgramState{mem: vec![109,123,99], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(123, result.0.rb);

        let program_state = ProgramState{mem: vec![9,3,99,43], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(43, result.0.rb);

        let program_state = ProgramState{mem: vec![9,5,109,10,99,43], pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(53, result.0.rb);

    }

    #[test]
    fn test_programs_with_relative_mode() {
        let program = extended(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        let program_state = ProgramState{mem: program, pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], result.2);

        let program = extended(vec![1102,34915192,34915192,7,4,7,99,0]);
        let program_state = ProgramState{mem: program, pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(1219070632396864, result.2[0]);

        let program = extended(vec![104,1125899906842624,99]);
        let program_state = ProgramState{mem: program, pc: 0, rb: 0, halted: false};
        let result = run_program(program_state, vec![], vec![]);
        assert_eq!(1125899906842624, result.2[0]);
    }
}

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub mem: Vec<i64>,
    pub pc: usize,
    pub rb: usize,
    pub halted: bool
}

pub fn extended(mut input: Vec<i64>) -> Vec<i64> {
    let mut more: Vec<i64> = vec![0;2000];
    input.append(&mut more);
    input
}

fn get_opcode(instruction: i64) -> i64 {
    instruction % 100
}

fn get_parameter_mode(instruction: i64, parameter: u32) -> usize {
    assert!(parameter > 0 && parameter < 4);
    ((instruction % 10_i64.pow(parameter + 2)) / 10_i64.pow(parameter + 1)) as usize
}

fn get_value(program: &Vec<i64>, index: i64, mode: usize, rb: &usize) -> i64 {
    match mode {
        0 => program[program[index as usize] as usize],
        1 => program[index as usize],
        2 => program[(*rb as i64 + program[index as usize]) as usize],
        _ => panic!("Invalid parameter mode")
    }
}

pub fn run_program(mut program_state: ProgramState, mut inputs: Vec<i64>, mut outputs: Vec<i64>) -> (ProgramState,Vec<i64>,Vec<i64>) {
    let mut i = program_state.pc as i64;
    let mut program = program_state.mem;


    while !program_state.halted {
        let instruction = program[i as usize];
        match get_opcode(instruction) {
            1 => {
                let mut_i = match get_parameter_mode(instruction, 3) {
                    2 => {
                        (program_state.rb as i64 + program[(i+3) as usize]) as usize
                    },
                    _ => {
                        program[(i+3) as usize] as usize
                    }
                };
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1), &program_state.rb);
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2), &program_state.rb);
                program[mut_i] = first_value + second_value;
                i += 4;
            },
            2 => {
                let mut_i = match get_parameter_mode(instruction, 3) {
                    2 => {
                        (program_state.rb as i64 + program[(i+3) as usize]) as usize
                    },
                    _ => {
                        program[(i+3) as usize] as usize
                    }
                };
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1), &program_state.rb);
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2), &program_state.rb);
                program[mut_i] = first_value * second_value;
                i += 4;
            },
            3 => {
                if inputs.len() == 0 {
                    // Pause if an input is needed
                    break;
                }
                match get_parameter_mode(instruction, 1) {
                    2 => {
                        let mut_i = (program_state.rb as i64 + program[(i+1) as usize]) as usize;
                        program[mut_i] = inputs.remove(0);
                    },
                    _ => {
                        let mut_i = program[(i+1) as usize] as usize;
                        program[mut_i] = inputs.remove(0);
                    }
                }
                i += 2;
            },
            4 => {
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1), &program_state.rb);
                outputs.push(first_value);
                i += 2;
            },
            5 => {
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1), &program_state.rb);
                if first_value != 0 {
                    i = get_value(&program, i+2, get_parameter_mode(instruction, 2), &program_state.rb);
                } else {
                    i += 3;
                }
            },
            6 => {
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1), &program_state.rb);
                if first_value == 0 {
                    i = get_value(&program, i+2, get_parameter_mode(instruction, 2), &program_state.rb);
                } else {
                    i += 3;
                }
            },
            7 => {
                let mut_i = match get_parameter_mode(instruction, 3) {
                    2 => {
                        (program_state.rb as i64 + program[(i+3) as usize]) as usize
                    },
                    _ => {
                        program[(i+3) as usize] as usize
                    }
                };
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1), &program_state.rb);
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2), &program_state.rb);
                if first_value < second_value {
                    program[mut_i] = 1;
                } else {
                    program[mut_i] = 0;
                }
                i += 4;
            },
            8 => {
                let mut_i = match get_parameter_mode(instruction, 3) {
                    2 => {
                        (program_state.rb as i64 + program[(i+3) as usize]) as usize
                    },
                    _ => {
                        program[(i+3) as usize] as usize
                    }
                };
                let first_value = get_value(&program, i+1, get_parameter_mode(instruction, 1), &program_state.rb);
                let second_value = get_value(&program, i+2, get_parameter_mode(instruction, 2), &program_state.rb);
                if first_value == second_value {
                    program[mut_i] = 1;
                } else {
                    program[mut_i] = 0;
                }
                i += 4;
            },
            9 => {
                let delta = get_value(&program, i+1, get_parameter_mode(instruction, 1), &program_state.rb);
                let new = program_state.rb as i64 + delta;
                program_state.rb = new as usize;
                i += 2;
            },
            99 => {
                program_state.halted = true;
                break;
            },
            _ => panic!("Invalid opcode")
        }
    }
    program_state.mem = program;
    program_state.pc = i as usize;
    (program_state, inputs, outputs)
}