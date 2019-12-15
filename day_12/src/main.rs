use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gravity() {
        let input = "<x=-1, y=0, z=2>
                    <x=2, y=-10, z=-7>
                    <x=4, y=-8, z=8>
                    <x=3, y=5, z=-1>";
        let moons = parse_moons(input);

        let gravity = get_gravity(&moons);
        assert_eq!(vec![3,-1,-1], gravity[0]);
        assert_eq!(vec![1,3,3], gravity[1]);
        assert_eq!(vec![-3,1,-3], gravity[2]);
        assert_eq!(vec![-1,-3,1], gravity[3]);
    }

    #[test]
    fn test_step() {
        let input = "<x=-1, y=0, z=2>
                    <x=2, y=-10, z=-7>
                    <x=4, y=-8, z=8>
                    <x=3, y=5, z=-1>";
        let moons = parse_moons(input);

        let moons = steps(moons, 1);
        assert_eq!((2,-1,1), moons[0].pos);
        assert_eq!((3,-7,-4), moons[1].pos);
        assert_eq!((1,-7,5), moons[2].pos);
        assert_eq!((2,2,0), moons[3].pos);

        let moons = steps(moons, 9);
        assert_eq!((2,1,-3), moons[0].pos);
        assert_eq!((1,-8,0), moons[1].pos);
        assert_eq!((3,-6,1), moons[2].pos);
        assert_eq!((2,0,4), moons[3].pos);
    }

    #[test]
    fn test_energy() {
        let input = "<x=-1, y=0, z=2>
                    <x=2, y=-10, z=-7>
                    <x=4, y=-8, z=8>
                    <x=3, y=5, z=-1>";
        let moons = parse_moons(input);
        let moons = steps(moons, 10);
        assert_eq!(179, get_energy(&moons));
    }

    #[test]
    fn test_find_loop() {
        let input = "<x=-1, y=0, z=2>
                    <x=2, y=-10, z=-7>
                    <x=4, y=-8, z=8>
                    <x=3, y=5, z=-1>";
        let moons = parse_moons(input);

        let delta = find_repeat_periods(&moons);
        assert_eq!(2772, lcm(lcm(delta.0, delta.1), delta.2));
    }

    #[test]
    fn test_find_cycle_example_two() {
        let input = "<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>";

    let moons = parse_moons(input);
    

    let delta = find_repeat_periods(&moons);
    println!("{:?}", delta);

    assert_eq!(4686774924, lcm(lcm(delta.0, delta.1),delta.2));
    }
}

#[derive(Debug, Clone)]
struct Moon {
    pos: (i64,i64,i64),
    vel: (i64,i64,i64)
}

fn lcm(a: u64, b: u64) -> u64 {
    let ap = primes::factors(a);
    let mut bp = primes::factors(b);

    for f in &ap {
        // vector find_item unstable, therefore doing it this way
        let index = bp.iter().position(|&x| x == *f);
        match index {
            Some(v) => {
                bp.remove(v);
            },
            None => ()
        }
    }

    let a: u64 = ap.iter().product();
    let b: u64 = bp.iter().product();
    a * b
}

fn find_repeat_periods(moons: &Vec<Moon>) -> (u64,u64,u64) {
    let mut moons = moons.clone();
    let mut repeat_x = HashMap::new();
    let mut repeat_y = HashMap::new();
    let mut repeat_z = HashMap::new();

    let mut delta = (0, 0, 0);

    for i in 0.. {
        moons = steps(moons, 1);
        let x_state = (moons[0].pos.0, moons[1].pos.0, moons[2].pos.0, moons[3].pos.0, 
                        moons[0].vel.0, moons[1].vel.0, moons[2].vel.0, moons[3].vel.0);

        let y_state = (moons[0].pos.1, moons[1].pos.1, moons[2].pos.1, moons[3].pos.1, 
            moons[0].vel.1, moons[1].vel.1, moons[2].vel.1, moons[3].vel.1);

        let z_state = (moons[0].pos.2, moons[1].pos.2, moons[2].pos.2, moons[3].pos.2, 
            moons[0].vel.2, moons[1].vel.2, moons[2].vel.2, moons[3].vel.2);
        
        match repeat_x.insert(x_state, i) {
            Some(prev) => {
                delta.0 = i-prev;
            },
            _ => ()
        };

        match repeat_y.insert(y_state, i) {
            Some(prev) => {
                delta.1 = i-prev;
            },
            _ => ()
        };

        match repeat_z.insert(z_state, i) {
            Some(prev) => {
                delta.2 = i-prev;
            },
            _ => ()
        };

        if delta.0 != 0 && delta.1 != 0 && delta.2 != 0 {
            break;
        }
    }
    delta
}

fn reduce(input: i64) -> i64 {
    match input {
        0 => 0,
        _ => input/input.abs()
    }
}

fn get_gravity(moons: &Vec<Moon>) -> Vec<Vec<i64>> {
    let mut gravity: Vec<_> = (0..moons.len()).map(|_x| vec![0,0,0]).collect();
    let combinations: Vec<_> = (0..moons.len()).combinations(2).collect();
    
    // Calculate gravity
    for combo in &combinations {
        let dx = moons[combo[0]].pos.0 - moons[combo[1]].pos.0;
        let dy = moons[combo[0]].pos.1 - moons[combo[1]].pos.1;
        let dz = moons[combo[0]].pos.2 - moons[combo[1]].pos.2;
        gravity[combo[0]][0] -= reduce(dx);
        gravity[combo[1]][0] += reduce(dx);
        gravity[combo[0]][1] -= reduce(dy);
        gravity[combo[1]][1] += reduce(dy);
        gravity[combo[0]][2] -= reduce(dz);
        gravity[combo[1]][2] += reduce(dz);
    }
    gravity
}

fn steps(mut moons: Vec<Moon>, steps: usize) -> Vec<Moon> {
    for _ in 0..steps {
        // Calculate gravity
        let gravity = get_gravity(&moons);
        // Apply gravity
        for m in 0..moons.len() {
            moons[m].vel.0 += gravity[m][0];
            moons[m].vel.1 += gravity[m][1];
            moons[m].vel.2 += gravity[m][2];
        }

        // Apply velocity
        for m in 0..moons.len() {
            moons[m].pos.0 += moons[m].vel.0;
            moons[m].pos.1 += moons[m].vel.1;
            moons[m].pos.2 += moons[m].vel.2;
        }
    }

    moons
}

fn get_energy(moons: &Vec<Moon>) -> i64 {
    moons.iter().map(|m| 
        (m.pos.0.abs() + m.pos.1.abs() + m.pos.2.abs())
        *
        (m.vel.0.abs() + m.vel.1.abs() + m.vel.2.abs())
    )
    .sum()
}

fn part_one(moons: &Vec<Moon>) -> i64 {
    let moons = steps(moons.clone(), 1000);
    get_energy(&moons)
}

fn part_two(moons: &Vec<Moon>) -> u64 {
    let moons = moons.clone();
    let delta = find_repeat_periods(&moons);
    lcm(lcm(delta.0, delta.1), delta.2)
}

fn parse_moons(data: &str) -> Vec<Moon> {
    let mut moons: Vec<Moon> = Vec::new();

    for line in data.lines() {
        let data = line.replace("=", ",");
        let data = data.replace(">", ",");
        let data: Vec<&str> = data.split(",").collect();
        let moon = Moon {
            pos: (data[1].parse().unwrap(),
                data[3].parse().unwrap(),
                data[5].parse().unwrap()),
            vel: (0,0,0)
        };
        moons.push(moon);
    }
    moons
}

fn main() {
    let contents = fs::read_to_string("data/input").unwrap();
    let contents = contents.trim();
    let moons = parse_moons(&contents);

    println!("Part 1: {}", part_one(&moons));
    println!("Part 2: {}", part_two(&moons));
}
