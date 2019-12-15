use std::collections::HashMap;
use aoc;

#[derive(Debug)]
struct Robot {
    x: i64, // right
    y: i64, // down
    dir: u8 // 0 up; 1 right, 2 down, 3 left
}

fn paint_hull(mut program_state: aoc::intcode_computer::ProgramState, start_color: i64) -> HashMap<(i64,i64), i64> {

    let mut hull: HashMap<(i64,i64), i64> = HashMap::new();
    let mut robot = Robot{x: 0, y:0, dir: 0};

    if start_color != 0 {
        hull.insert((0,0), start_color);
    }

    loop {
        // Get color of current pixel
        let color = match hull.get(&(robot.x, robot.y)) {
            Some(x) => *x,
            _ => 0
        };

        // Run program
        let (state, _input, output) = aoc::intcode_computer::run_program(program_state, vec![color], vec![]);
        program_state = state;
        if program_state.halted {
            break;
        }
        assert_eq!(2, output.len());

        // Paint hull
        if output[0] == 1 {
            hull.insert((robot.x, robot.y), 1);
        } else {
            if hull.get(&(robot.x, robot.y)).is_some() {
                hull.insert((robot.x, robot.y), 0);
            }
        }

        // Update robot position
        let dir_change = output[1] * 2 - 1;
        robot.dir = (((robot.dir as i64 + dir_change) + 4) % 4) as u8;
        match robot.dir {
            0 => { robot.y -= 1},
            1 => { robot.x += 1},
            2 => { robot.y += 1},
            3 => { robot.x -= 1},
            _ => panic!("invalid dir")
        }        
    }
    hull
}

fn part_two(original_program: &Vec<i64>) {

    let program_state = aoc::intcode_computer::ProgramState {mem: aoc::intcode_computer::extended(original_program.clone()), pc: 0, rb: 0, halted: false};
    let hull = paint_hull(program_state, 1);
    
    // Get dimensions of the painting
    let painted: HashMap<(i64,i64),i64> = hull.into_iter().filter(|x| x.1 == 1).collect();
    let min_x = (painted.iter().min_by( |&a, &b| ((a.0).0).partial_cmp(&((b.0).0)).unwrap()).unwrap().0).0;
    let max_x = (painted.iter().max_by( |&a, &b| ((a.0).0).partial_cmp(&((b.0).0)).unwrap()).unwrap().0).0;
    let min_y = (painted.iter().min_by( |&a, &b| ((a.0).1).partial_cmp(&((b.0).1)).unwrap()).unwrap().0).1;
    let max_y = (painted.iter().max_by( |&a, &b| ((a.0).1).partial_cmp(&((b.0).1)).unwrap()).unwrap().0).1;
    

    for y in min_y..=max_y {
        let mut row = String::new();
        for x in min_x..=max_x {
            row += match painted.get(&(x,y)) {
                Some(1) => "#",
                _ => " "
            };
        }
        println!("{}", row);
    }
}

fn part_one(original_program: &Vec<i64>) -> usize {
    let program_state = aoc::intcode_computer::ProgramState {mem: aoc::intcode_computer::extended(original_program.clone()), pc: 0, rb: 0, halted: false};
    let hull = paint_hull(program_state, 0);
    hull.len()
}

fn main() {
    let original_program = aoc::vector_from_comma_separated_file::<i64>("data/input")
    .expect("Error getting program");

    println!("Part 1: {}", part_one(&original_program));
    println!("Part 2:");
    part_two(&original_program);
}
