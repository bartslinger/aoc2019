use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_numbers() {
        assert!(valid(111111));
        assert!(!valid(223450));
        assert!(!valid(123789));
    }
}

fn valid(number: u64) -> bool {
    let n = format!("{}", number);
    assert_eq!(6, n.len());

    let b = n.as_bytes();

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
        return has_double(b);
    }

    return false;
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

fn main() {
    let contents = fs::read_to_string("data/input").unwrap();

    let numbers = contents.trim().split('-').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

    // Brute force!
    let mut cnt = 0;
    for number in numbers[0]..=numbers[1] {
        if valid(number) {
            cnt += 1;
        }
    }

    println!("Part 1: {}", cnt);


}
