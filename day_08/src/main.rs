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

    let chars: Vec<char> = input.chars().collect();
    let chunks: Vec<Vec<char>> = chars.chunks(size).map(|x| x.to_vec()).collect();

    // turn into layers
    let mut layers = Vec::new();
    for c in chunks {
        // split in rows and columns
        let layer: Vec<_> = c.chunks(width).map(|x| x.to_vec()).collect();
        layers.push(layer);
    }

    // Initialize transparent image
    let mut image: Vec<Vec<char>> = Vec::new();
    for _ in 0..height {
        image.push(vec!['2';width]);
    }

    // Aaarghh so many nested for loops
    for layer in &layers {
        for i in 0..height {
            for j in 0..width {
                if image[i][j] == '2' && layer[i][j] != '2' {
                    image[i][j] = match layer[i][j] {
                        '1' => '#',
                        '0' => ' ',
                        _ => 'x'
                    };
                }
            }
        }
    }

    for line in &image {
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
