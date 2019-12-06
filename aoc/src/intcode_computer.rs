#[cfg(test)]
mod tests {
    use super::program_tick;

    #[test]
    fn test_program_tick() {
        assert_eq!(vec![2,0,0,0,99], program_tick(vec![1,0,0,0,99]));
        assert_eq!(vec![2,3,0,6,99], program_tick(vec![2,3,0,3,99]));
        assert_eq!(vec![2,4,4,5,99,9801], program_tick(vec![2,4,4,5,99,0]));
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], program_tick(vec![1,1,1,4,99,5,6,0,99]));
    }
}

pub fn program_tick(mut program: Vec<i64>) -> Vec<i64> {
    let mut i = 0;
    loop {
        match program[i] {
            1 => {
                let mut_i = program[i+3] as usize;
                program[mut_i] = program[program[i+1] as usize] + program[program[i+2] as usize];
            },
            2 => {
                let mut_i = program[i+3] as usize;
                program[mut_i] = program[program[i+1] as usize] * program[program[i+2] as usize];
            },
            99 => break,
            _ => panic!("Invalid opcode")
        }
        i += 4;
    }
    program
}