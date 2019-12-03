use std::collections::HashMap;
use std::fs;
use std::time;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trace_wires() {
        let r = trace_wires(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(159, r.0);
        assert_eq!(610, r.1);
        
        let r = trace_wires(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(135, r.0);
        assert_eq!(410, r.1);
    }

    #[test]
    fn test_parse_wiresection() {
        assert_eq!(WireSection{direction: Direction::UP, length: 10}, parse_wire_section("U10"));
        assert_eq!(WireSection{direction: Direction::DOWN, length: 11}, parse_wire_section("D11"));
        assert_eq!(WireSection{direction: Direction::LEFT, length: 12}, parse_wire_section("L12"));
        assert_eq!(WireSection{direction: Direction::RIGHT, length: 13}, parse_wire_section("R13"));
    }

}

#[derive(PartialEq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(PartialEq, Debug)]
struct WireSection {
    direction: Direction,
    length: u32
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32
}

fn step(x: &mut i32, y: &mut i32, direction: &Direction) -> Position {
    match direction {
        Direction::UP => *y += 1,
        Direction::DOWN => *y -= 1,
        Direction::LEFT => *x -= 1,
        Direction::RIGHT => *x += 1
    }
    Position{ x:*x, y:*y }
}

fn trace_wires(wire1: &str, wire2: &str) -> (u32, u32) {

    let mut minimum_distance = u32::max_value();
    let mut minimum_delay = u32::max_value();
    let mut h: HashMap<Position, u32> = HashMap::new();
    
    let w1 = parse_wire_string(wire1);
    let w2 = parse_wire_string(wire2);

    // Trace the first wire
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    let mut len: u32 = 1;
    for segm in w1 {
        let mut seglen = segm.length;
        while seglen > 0 {
            let pos = step(&mut x, &mut y, &segm.direction);
            h.insert(pos, len);
            seglen -= 1;
            len += 1;
        }
    }

    // Find crossings with second wire
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    let mut len: u32 = 1;
    for segm in w2 {
        let mut seglen = segm.length;
        while seglen > 0 {
            let pos = step(&mut x, &mut y, &segm.direction);
            match h.get(&pos) {
                Some(w1_len) => {
                    // found crossing with first wire
                    let delay = w1_len + len;
                    if delay < minimum_delay {
                        minimum_delay = delay;
                    }
                    let dist = y.abs() as u32 + x.abs() as u32;
                    if dist < minimum_distance {
                        minimum_distance = dist;
                    }
                },
                None => ()
            };
            len += 1;
            seglen -= 1;
        }
    }

    (minimum_distance, minimum_delay)
}

fn parse_wire_section(section: &str) -> WireSection {
    let direction = match &section.chars().next().unwrap() {
        'U' => Direction::UP,
        'D' => Direction::DOWN,
        'L' => Direction::LEFT,
        'R' => Direction::RIGHT,
        _ => panic!("Invalid instruction")
    };
    let len: u32 = section[1..].parse().unwrap();
    
    WireSection {
        direction: direction,
        length: len
    }
}

fn parse_wire_string(wire: &str) -> Vec<WireSection> {
    let mut v = Vec::new();
    for line in wire.split(',') {
        v.push(parse_wire_section(line));
    }
    v
}

fn main() {
    let start = time::Instant::now();
    let contents = fs::read_to_string("data/input").unwrap();
    
    let mut lines = Vec::new();
    for line in contents.lines() {
        lines.push(line);
    }
    let result = trace_wires(lines.get(0).unwrap(), lines.get(1).unwrap());
    
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}
