use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from("123456789012");
        assert_eq!(1, part_one(&input, 3, 2));
    }
}

fn part_one(input: &str, width: usize, height: usize) -> usize {
    let size = width * height;
    let charvec: Vec<char> = input.chars().collect();
    let layers: Vec<&[char]> = charvec.chunks(size).collect();
    let layer_with_least_zeros = layers.iter().min_by_key(|layer| layer.iter().filter(|&ch| *ch=='0').count()).unwrap();

    ['1', '2'].iter().map(|s| layer_with_least_zeros.iter().filter(|&c| *c==*s).count()).product()
}

fn part_two(input: &str, width: usize, height: usize) {
    let size = width * height;

    let charvec: Vec<char> = input.chars().collect();
    let layers: Vec<&[char]> = charvec.chunks(size).collect();

    let pixel_value = |x:(&char,&char)| -> char {
        if *x.0=='2' {
            match *x.1 {
                '0' => ' ',
                '1' => '#',
                _ => *x.1
            }
        } else {
            *x.0
        }
    };

    let mut image: Vec<char> = vec!['2'; size];
    for layer in layers {
        // Zip the two iterators to produce single output
        image = image.iter().zip(layer).map(|x| pixel_value(x)).collect();
    }

    for line in image.chunks(width) {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let contents = fs::read_to_string("data/input").unwrap();
    let contents = contents.trim();

    println!("Part 1: {}", part_one(&contents, 25, 6));
    println!("Part 2:");
    part_two(&contents, 25, 6);
}
