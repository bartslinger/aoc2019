use std::collections::{HashSet, HashMap};
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_input() {
        let input = ".#..#
        .....
        #####
        ....#
        ...##";

        let asteroids = get_asteroid_map(input);
        let best = find_best_location(&asteroids);
        assert_eq!(((3,4),8), best);
    }

    #[test]
    fn test_other_inputs() {
        let input = "......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####";
        let asteroids = get_asteroid_map(input);
        let best = find_best_location(&asteroids);
        assert_eq!(((5,8),33), best);

        let input = "#.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.";
        let asteroids = get_asteroid_map(input);
        let best = find_best_location(&asteroids);
        assert_eq!(((1,2),35), best);

        let input = ".#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..";
        let asteroids = get_asteroid_map(input);
        let best = find_best_location(&asteroids);
        assert_eq!(((6,3),41), best);

    }

    #[test]
    fn test_big_input() {
        let input = ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";
        let asteroids = get_asteroid_map(input);
        let best = find_best_location(&asteroids);
        assert_eq!(((11,13),210), best);

        let visible = get_visible_asteroids((11,13), &asteroids);
        assert!(!&visible.contains(&(11,13)));
        assert_eq!(210, visible.len());

        let laser_vaporize_order = super_sick_laser_vaporation_order((11,13), &asteroids);
        assert_eq!((11,12), laser_vaporize_order[0]);
        assert_eq!((12,1), laser_vaporize_order[1]);
        assert_eq!((12,2), laser_vaporize_order[2]);
        assert_eq!((12,8), laser_vaporize_order[10-1]);
        assert_eq!((16,0), laser_vaporize_order[20-1]);
        assert_eq!((16,9), laser_vaporize_order[50-1]);
        assert_eq!((10,16), laser_vaporize_order[100-1]);
        assert_eq!((9,6), laser_vaporize_order[199-1]);
        assert_eq!((8,2), laser_vaporize_order[200-1]);
        assert_eq!((10,9), laser_vaporize_order[201-1]);
        assert_eq!((11,1), laser_vaporize_order[299-1]);
    }

    #[test]
    fn test_get_smallest_unit() {
        assert_eq!(((0,0),0), get_smallest_unit((0,0)));
        assert_eq!(((0,1),2), get_smallest_unit((0,2)));
        assert_eq!(((0,1),1), get_smallest_unit((0,1)));
        assert_eq!(((-1,0),4), get_smallest_unit((-4,0)));
        assert_eq!(((1,0),2), get_smallest_unit((2,0)));
        assert_eq!(((1,1),2), get_smallest_unit((2,2)));
        assert_eq!(((2,3),6), get_smallest_unit((12,18)));
        assert_eq!(((-2,3),6), get_smallest_unit((-12,18)));
    }
}

fn get_smallest_unit(input: (i64,i64)) -> ((i64,i64),i64) {
    let mut smallest_unit = ((0,0),0);
    if input.0 == 0 && input.1 == 0 {
        // Nothing
    } else if input.0 == 0 {
        smallest_unit = ((0, input.1/i64::abs(input.1)), i64::abs(input.1));
    } else if input.1 == 0 {
        smallest_unit = ((input.0/i64::abs(input.0), 0), i64::abs(input.0));
    } else {
        for n in (1..=i64::max(i64::abs(input.0), i64::abs(input.1))).rev() {
            if input.0 % n == 0 && input.1 % n == 0 {
                smallest_unit = ((input.0/n, input.1/n),n);
                break;
            }
        }
    }
    smallest_unit
}

fn is_visible(location: (i64,i64), target: (i64,i64), asteroids: &HashSet<(i64,i64)>) -> bool {
    let dx = target.0 - location.0;
    let dy = target.1 - location.1;
    
    if dx == 0 && dy == 0 {
        return false;
    }
    
    let (smallest_unit, n) = get_smallest_unit((dx,dy));

    // Check for every possible location in between if it is occupied
    for i in 1..n {
        let scan_x = location.0 + smallest_unit.0 * i;
        let scan_y = location.1 + smallest_unit.1 * i;

        if asteroids.contains(&(scan_x, scan_y)) {
            return false;
        }
    }
    
    true
}

fn get_visible_asteroids(location: (i64,i64), asteroids: &HashSet<(i64,i64)>) -> HashSet<(i64,i64)> {
    asteroids.iter()
        .filter(|&a| is_visible(location, *a, asteroids))
        .cloned()
        .collect::<HashSet<_>>()
}

fn angle(location: (i64,i64), target: (i64,i64)) -> f64 {
    let dx = target.0 - location.0;
    let dy = target.1 - location.1;

    let mut extra = 0.0;
    if dx < 0 {
        extra = std::f64::consts::PI * 2.0;
    }

    f64::atan2(dx as f64, -dy as f64) + extra
}

fn sort_asteroids_by_angle(location: (i64, i64), asteroids: &HashSet<(i64,i64)>) -> Vec<(i64,i64)> {
    let mut asteroids_with_angles = asteroids.iter()
        .map(|&a| (a, angle(location, a)))
        .collect::<HashMap<_,_>>();

    let mut sorted_asteroids: Vec<(i64,i64)> = Vec::new();

    while asteroids_with_angles.len() > 0 {
        let next_asteroid = asteroids_with_angles.iter()
        .min_by(|&x, &y| (x.1).partial_cmp(y.1).unwrap()).unwrap();
        let target = *next_asteroid.0;

        sorted_asteroids.push(target);
        asteroids_with_angles.remove(&target);
    }

    sorted_asteroids
}

fn super_sick_laser_vaporation_order(location: (i64,i64), asteroids: &HashSet<(i64,i64)>) -> Vec<(i64,i64)> {
    let mut order = Vec::new();

    let mut asteroids = asteroids.clone();
    while asteroids.len() > 1 {
        // get visible asteroids
        let visible = get_visible_asteroids(location, &asteroids);
        let mut sorted = sort_asteroids_by_angle(location, &visible);
        order.append(&mut sorted);
        for a in visible {
            asteroids.remove(&a);
        }
    }
    
    order
}

fn find_best_location(asteroids: &HashSet<(i64,i64)>) -> ((i64,i64),i64) {
    asteroids.iter()
        .map(|a| (*a, get_visible_asteroids(*a, asteroids).len() as i64))
        .max_by_key(|&x| x.1).unwrap()
}

fn get_asteroid_map(input: &str) -> HashSet<(i64, i64)> {
    input.lines()
        .enumerate()
        .flat_map(|(i,line)|
            line.trim()
            .chars()
            .enumerate()
            .filter(|(_j, x)|  *x == '#')
            .map(move |(j,_x)| (j as i64,i as i64))
        )
        .collect::<HashSet<_>>()
}

fn main() {
    let contents = fs::read_to_string("data/input").unwrap();
    let input = contents.trim();

    let asteroids = get_asteroid_map(input);
    let best = find_best_location(&asteroids);
    println!("Part 1: {:?}", best);

    let laser_vaporize_order = super_sick_laser_vaporation_order(best.0, &asteroids);
    let nr200 = laser_vaporize_order[200-1];
    println!("Part 2: {:?}", nr200.0*100+nr200.1);
}
