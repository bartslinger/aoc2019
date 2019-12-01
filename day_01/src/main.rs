use aoc;

#[cfg(test)]
mod tests {
    use super::{calculate_fuel, calculate_cumulative_fuel};

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(2, calculate_fuel(&12));
        assert_eq!(2, calculate_fuel(&14));
        assert_eq!(654, calculate_fuel(&1969));
        assert_eq!(33583, calculate_fuel(&100756));
    }

    #[test]
    fn test_cumulative_fuel() {
        assert_eq!(2, calculate_cumulative_fuel(&14));
        assert_eq!(966, calculate_cumulative_fuel(&1969));
        assert_eq!(50346, calculate_cumulative_fuel(&100756));
    }
}

fn calculate_fuel(mass: &u32) -> u32 {
    let fuel = mass / 3;
    if fuel <= 2 {
        return 0
    }
    fuel - 2
}

fn calculate_cumulative_fuel(mass: &u32) -> u32 {
    let mut fuel = calculate_fuel(mass);
    if fuel != 0 {
        fuel += calculate_cumulative_fuel(&fuel);
    }
    fuel
}

fn main() {
    let filename = "data/input";
    let data = aoc::vector_from_file::<u32>(filename).expect("Error getting data");

    println!("Part 1: {}", data.iter().map(calculate_fuel).sum::<u32>());
    println!("Part 2: {}", data.iter().map(calculate_cumulative_fuel).sum::<u32>());
}
