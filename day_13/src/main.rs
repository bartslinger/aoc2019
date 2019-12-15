use aoc;
use std::collections::HashMap;

fn part_two(mut program_state: aoc::intcode_computer::ProgramState) -> i64 {
    let mut screen = HashMap::new();
    program_state.mem[0] = 2;
    let mut next_input = vec![];
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;

    while program_state.halted == false {
    // for _ in 0..239 {
        let (state, _input, output) = aoc::intcode_computer::run_program(program_state, next_input.clone(), vec![]);
        program_state = state;
        assert_eq!(0, output.len() % 3);
        let instructions = output.chunks(3);
        for i in instructions {
            if i[0] == -1 && i[1] == 0 {
                score = i[2];
            } else {
                // update screen
                screen.insert((i[0], i[1]), i[2]);
            }
            if i[2] == 3 {
                // Update paddle
                paddle_x = i[0];
            }
            if i[2] == 4 {
                // Update ball
                ball_x = i[0];
            }
        }

        // Feedback control
        let mut cmd = 0;
        if ball_x != paddle_x {
            cmd = ball_x-paddle_x;
            cmd = cmd/cmd.abs();
        }

        next_input = vec![cmd];
    }

    // Show screen
    for y in 0..23 {
        let mut row = String::new();
        for x in 0..37 {
            let pixel = match screen.get(&(x,y)) {
                Some(0) => ' ',
                Some(1) => 'w',
                Some(2) => '#',
                Some(3) => '_',
                Some(4) => 'o',  
                _ => '.'
            };
            row.push(pixel);
        }
        println!("{}", row);
    }

    score
}

fn part_one(program_state: aoc::intcode_computer::ProgramState) -> usize {
    let (_state, _input, output) = aoc::intcode_computer::run_program(program_state, vec![], vec![]);
    assert_eq!(0, output.len() % 3);
    
    let instructions = output.chunks(3);
    instructions.filter(|&x| x[2]==2).count()
}

fn main() {

    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
    .expect("Error getting program");

    let original_program_state = aoc::intcode_computer::ProgramState {mem: aoc::intcode_computer::extended(original_program), pc: 0, rb: 0, halted: false};
    
    println!("Part 1: {}", part_one(original_program_state.clone()));
    println!("Part 2: {}", part_two(original_program_state.clone()));
}
