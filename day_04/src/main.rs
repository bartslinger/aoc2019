use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_numbers_one() {
        assert!(valid(111111).0);
        assert!(!valid(223450).0);
        assert!(!valid(123789).0);
    }

    #[test]
    fn test_valid_numbers_two() {
        assert!(valid(112233).1);
        assert!(!valid(123444).1);
        assert!(valid(111122).1);
    }

    #[test]
    fn test_find_valid_options() {
        // using puzzle answers
        let answers = find_valid_options(246540, 787419);
        assert_eq!(1063, answers.0);
        assert_eq!(686, answers.1);
    }
}

fn valid(number: u32) -> (bool, bool) {
    let n = format!("{}", number);
    assert_eq!(6, n.len());

    let b = n.as_bytes();

    if !(b[5] >= b[4] &&
        b[4] >= b[3] && 
        b[3] >= b[2] &&
        b[2] >= b[1] &&
        b[1] >= b[0])
    {
        return (false, false);
    }

    if b[5] == b[4] ||
        b[4] == b[3] ||
        b[3] == b[2] ||
        b[2] == b[1] ||
        b[1] == b[0]
    {
        return (true, has_double(b));
    }

    return (false, false);
}

fn has_double(b: &[u8]) -> bool {
    if b[0] == b[1] && b[1] != b[2] {
        return true;
    }
    if b[0] != b[1] && b[1] == b[2] && b[2] != b[3] {
        return true;
    }
    if b[1] != b[2] && b[2] == b[3] && b[3] != b[4] {
        return true;
    }
    if b[2] != b[3] && b[3] == b[4] && b[4] != b[5] {
        return true;
    }
    if b[3] != b[4] && b[4] == b[5] {
        return true;
    }
    false
}

fn find_valid_options(start: u32, end: u32) -> (u32, u32) {

    // Brute force!
    let mut cnt = 0;
    let mut cnt2 = 0;
    for number in start..=end {
        let r = valid(number);
        if r.0 {
            cnt += 1;
        }
        if r.1 {
            cnt2 += 1;
        }
    }

    (cnt,cnt2)
}

fn main() {
    let contents = fs::read_to_string("data/input").unwrap();

    let numbers = contents.trim().split('-').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    let answers = find_valid_options(numbers[0], numbers[1]);
    println!("Part 1: {}", answers.0);
    println!("Part 2: {}", answers.1);


}
