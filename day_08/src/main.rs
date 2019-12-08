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
    let layer_with_least_zeros = charvec.chunks(size)
        .min_by_key(|layer|
            layer.iter()
            .filter(|&ch| *ch=='0')
            .count()
        )
        .unwrap();

    ['1', '2'].iter()
        .map(|s|
            layer_with_least_zeros.iter()
            .filter(|&c| *c==*s)
            .count()
        )
        .product()
}

fn pixel_value(image: &char, layer: &char) -> char {
    if *image=='2' {
        match *layer {
            '0' => ' ',
            '1' => '#',
            _ => *layer
        }
    } else {
        *image
    }
}

fn part_two(input: &str, width: usize, height: usize) {
    let size = width * height;
    let charvec: Vec<char> = input.chars().collect();
    let layers: Vec<&[char]> = charvec.chunks(size).collect();

    let image: Vec<char> = layers.iter().fold(
        vec!['2'; size],
        |im, layer| im.iter()
            .zip(layer.iter())
            .map(|(im_px, lay_px)| pixel_value(im_px, lay_px))
            .collect()
        );

    // Visualize as string
    let output = image.chunks(width)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", output);
}

fn main() {
    let contents = fs::read_to_string("data/input").unwrap();
    let contents = contents.trim();

    println!("Part 1: {}", part_one(&contents, 25, 6));
    println!("Part 2:");
    part_two(&contents, 25, 6);
}
