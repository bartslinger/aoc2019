use std::fs;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::{get_orbit_count, get_orbits_vector};

    #[test]
    fn test_orbit_count() {
        // changed input to be out of order
        let input = "COM)B
        B)C
        J)K
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        K)L";
        let orbits = get_orbits_vector(input);
        let orbit_count = get_orbit_count(orbits);
        assert_eq!(42, orbit_count);
    }
}

fn get_orbit_count(mut orbits: Vec<Vec<&str>>) -> usize {

    let mut galaxy = HashMap::new();
    galaxy.insert("COM", 0);

    let mut sum = 0;
    while orbits.len() > 0 {
        let mut new_todo = Vec::new();

        for orbit in &orbits {
            match galaxy.get(orbit[0]) {
                Some(v) => {
                    let val = v + 1;
                    galaxy.insert(orbit[1], val);
                    sum += val;
                },
                None => {
                    new_todo.push(orbit.clone());
                }
            };
        }

        orbits = new_todo;
    }

    sum
}

fn get_orbits_vector(raw_input: &str) -> Vec<Vec<&str>> {
    let mut orbits = Vec::new();
    for line in raw_input.lines() {
        let orbit = line.trim().split(')').collect::<Vec<_>>();
        orbits.push(orbit);
    }
    orbits
}


fn main() {
    let contents = fs::read_to_string("data/input").unwrap();
    let orbits_vector = get_orbits_vector(&contents);

    println!("Part 1: {}", get_orbit_count(orbits_vector));
}
