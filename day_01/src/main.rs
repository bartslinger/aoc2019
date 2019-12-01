use std::fs::File;
use std::io::Read;

#[test]
fn test_calculate_fuel() {
    assert_eq!(2, calculate_fuel(12));
    assert_eq!(2, calculate_fuel(14));
    assert_eq!(654, calculate_fuel(1969));
    assert_eq!(33583, calculate_fuel(100756));
}

#[test]
fn test_cumulative_fuel() {
    assert_eq!(2, calculate_cumulative_fuel(14));
    assert_eq!(966, calculate_cumulative_fuel(1969));
    assert_eq!(50346, calculate_cumulative_fuel(100756));
}

fn calculate_fuel(mass: u32) -> u32 {
    let fuel = mass / 3;
    if fuel <= 2 {
        return 0
    }
    fuel - 2
}

fn calculate_cumulative_fuel(mass: u32) -> u32 {
    let mut fuel = calculate_fuel(mass);
    if fuel != 0 {
        fuel += calculate_cumulative_fuel(fuel);
    }
    fuel
}

fn get_file_contents(filename: String) -> String {
    let mut f = File::open(filename)
        .expect("Failed to open file");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Failed read to string");

    contents
}

fn get_data_vector(contents: String) -> Vec<u32> {
    let split = contents.trim().lines();
    let mut data = Vec::new();
    for line in split {
        let number: u32 = line.parse().expect("Invalid number");
        data.push(number);
    }
    data
}

fn main() {
    let contents = get_file_contents(String::from("data/input"));
    let data = get_data_vector(contents);

    let mut sum = 0;
    for number in &data {
        sum += calculate_fuel(*number);
    }
    println!("Part 1: {}", sum);

    sum = 0;
    for number in &data {
        sum += calculate_cumulative_fuel(*number);
    }
    println!("Part 2: {}", sum);

}
