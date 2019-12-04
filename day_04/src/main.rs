use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_numbers_one() {
        assert!(valid_one(to_byte_vec(111111)));
        assert!(!valid_one(to_byte_vec(223450)));
        assert!(!valid_one(to_byte_vec(123789)));
    }

    #[test]
    fn test_valid_numbers_two() {
        assert!(valid_two(to_byte_vec(112233)));
        assert!(!valid_two(to_byte_vec(123444)));
        assert!(valid_two(to_byte_vec(111122)));
    }

    #[test]
    fn test_find_valid_options() {
        // using puzzle answers
        let answers = find_valid_options(246540, 787419);
        assert_eq!(1063, answers.0);
        assert_eq!(686, answers.1);
    }
}

fn to_byte_vec(number: u32) -> Vec<u8> {
    let n = format!("{}", number);
    assert_eq!(6, n.len());
    n.into_bytes()
}

fn valid_one(b: Vec<u8>) -> bool {
    if !(b[5] >= b[4] &&
        b[4] >= b[3] && 
        b[3] >= b[2] &&
        b[2] >= b[1] &&
        b[1] >= b[0])
    {
        return false;
    }

    if b[5] == b[4] ||
        b[4] == b[3] ||
        b[3] == b[2] ||
        b[2] == b[1] ||
        b[1] == b[0]
    {
        return true;
    }
    false
}

fn valid_two(b: Vec<u8>) -> bool {
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

fn find_valid_options(start: u32, end: u32) -> (usize, usize) {
    let r: Vec<bool> = (start..=end).map(|x| to_byte_vec(x))
        .filter(|x| valid_one(x.to_vec()))
        .map(|x| valid_two(x))
        .collect();
    (r.len(), r.iter().filter(|&x| *x).count())
}

fn main() {
    let contents = fs::read_to_string("data/input").unwrap();
    let numbers = contents.trim().split('-').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    let answers = find_valid_options(numbers[0], numbers[1]);
    println!("Part 1: {}", answers.0);
    println!("Part 2: {}", answers.1);


}
