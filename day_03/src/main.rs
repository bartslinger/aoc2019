use std::collections::HashMap;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_closest_crossing() {
        let r = closest_crossing(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(159, r.0);
        assert_eq!(610, r.1);
        
        let r = closest_crossing(
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

#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(PartialEq)]
#[derive(Debug)]
struct WireSection {
    direction: Direction,
    length: u32
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32
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

fn closest_crossing(wire1: &str, wire2: &str) -> (u32, u32) {

    let mut h = HashMap::new();
    h.insert(Position{x:1,y:1}, 1);
    let v1 = parse_wire_string(wire1);
    let v2 = parse_wire_string(wire2);

    // Do first wire
    let mut up = 0;
    let mut right = 0;
    let mut len: u32 = 1;
    for instr in v1 {
        let mut slen = instr.length;
        while slen > 0 {
            match instr.direction {
                Direction::UP => up += 1,
                Direction::DOWN => up -= 1,
                Direction::LEFT => right -= 1,
                Direction::RIGHT => right += 1
            }
            h.insert(Position{x:right, y:up}, len);
            len += 1;
            slen -= 1;
        }
    }

    // Find matches with second string
    let mut minimum_distance = u32::max_value();
    let mut minimum_delay = u32::max_value();
    let mut matches = HashMap::new();
    let mut up = 0;
    let mut right = 0;
    let mut len: u32 = 1;
    for instr in v2 {
        let mut slen = instr.length;
        while slen > 0 {
            match instr.direction {
                Direction::UP => up += 1,
                Direction::DOWN => up -= 1,
                Direction::LEFT => right -= 1,
                Direction::RIGHT => right += 1
            }
            if h.contains_key(&Position{x:right, y:up}) {
                let w1_len = h.get(&Position{x:right, y:up}).unwrap();
                let delay = w1_len + len;
                if delay < minimum_delay {
                    minimum_delay = delay;
                }
                matches.insert(Position{x:right, y:up}, 1);
                let dist: u32 = up.abs() as u32 + right.abs() as u32;
                if dist < minimum_distance {
                    minimum_distance = dist;
                }
            }
            len += 1;
            slen -= 1;
        }
    }

    (minimum_distance, minimum_delay)
}

fn main() {
    let contents = fs::read_to_string("data/input").unwrap();
    
    let mut lines = Vec::new();
    for line in contents.lines() {
        lines.push(line);
    }
    let result = closest_crossing(lines.get(0).unwrap(), lines.get(1).unwrap());
    
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}
