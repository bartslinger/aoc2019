use std::fs;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::{get_orbits_vector, build_galaxy_map, count_all_orbits, get_distance_to_santa};

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
        let galaxy_map = build_galaxy_map(orbits);
        let orbit_count = count_all_orbits(&galaxy_map);
        assert_eq!(42, orbit_count);
    }

    #[test]
    fn test_get_distance_to_santa() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";
        let orbits = get_orbits_vector(input);
        let galaxy_map = build_galaxy_map(orbits);

        assert_eq!(4, get_distance_to_santa(galaxy_map));
    }
}

fn get_distance_to_santa(galaxy_map: HashMap<&str,&str>) -> usize {
    // First, follow the trajectory of santa to COM
    let mut santa_orbit = HashMap::new();
    let mut next: &str = galaxy_map.get("SAN").unwrap();
    let mut dist: usize = 0;
    while next != "COM" {
        santa_orbit.insert(next, dist);
        dist += 1;
        next = galaxy_map.get(next).unwrap();
    }

    // Now follow trajectory from YOU until there is a match
    let mut next: &str = galaxy_map.get("YOU").unwrap();
    let mut dist = 0;
    loop {
        match santa_orbit.get(next) {
            Some(v) => return v + dist,
            None => ()
        }
        dist += 1;
        next = galaxy_map.get(next).unwrap();
    }
}

fn count_all_orbits(galaxy_map: &HashMap<&str, &str>) -> usize {
    let mut sum = 0;
    for orbit in galaxy_map {
        // keep counting till COM is reached
        let mut next: &str = orbit.1;
        while next != "COM" {
            sum += 1;
            next = galaxy_map.get(next).unwrap();
        }
        sum += 1; // Count orbit around COM
    }
    sum
}

fn build_galaxy_map(orbits: Vec<Vec<&str>>) -> HashMap<&str, &str> {

    let mut galaxy = HashMap::new();

    for orbit in orbits {
        galaxy.insert(orbit[1], orbit[0]);
    }

    galaxy
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
    let orbits = get_orbits_vector(&contents);
    let galaxy_map = build_galaxy_map(orbits.clone());

    println!("Part 1: {}", count_all_orbits(&galaxy_map));
    println!("Part 2: {}", get_distance_to_santa(galaxy_map));
}
