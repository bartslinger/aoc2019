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

    let chars: Vec<char> = input.chars().collect();
    let chunks: Vec<_> = chars.chunks(size)
                        .map(|ch| (ch, ch.iter().filter(|&ch| *ch=='0').count()))
                        .collect();

    // Can't figure out yet how to get the answer completely from iterators
    let mut best_chunk = chunks[0].0;
    let mut best_zerocount = chunks[0].1;
    for (chunk,zerocount) in chunks {
        if zerocount < best_zerocount {
            best_zerocount = zerocount;
            best_chunk = chunk;
        }
    }
    let ones = best_chunk.iter().filter(|&x| *x == '1').count();
    let twos = best_chunk.iter().filter(|&x| *x == '2').count();
    ones * twos
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
