use crate::lib;

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let iea = &input[0].chars().collect::<Vec<char>>();
    let mut img = input
        .iter()
        .skip(2)
        .filter(|s| **s != "")
        .flat_map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let mut s = lib::Shape::new(input[2].len(), img.len() / input[2].len());

    for n in 0..2 {
        let padder = match n {
            // On first iteration, pad with '.'
            0 => '.',
            // On the second iteration, pad depends on what is in iea[0]
            1 => iea[0],
            _ => panic!("Invalid iteration"),
        };

        // Create a padded shape and image, pad by 1 pixel in all directions
        let (p_shape, p_img) = lib::pad(&s, &img, 1, padder);

        // Create a vector to store the output of this iterations
        let mut out: Vec<char> = vec![];

        for i in 0..p_img.len() {
            // Get the neighbours of the current pixel and convert them to bools.
            // 'neighbours_2' returns a vector of Option<usize>, of indexes of the neighbours.
            // The Option is None when we are looking at a pixel outside the shape.
            let mut bits = lib::neighbours_2(&p_shape, i, true)
                .iter()
                .map(|j| match j {
                    Some(j) => p_img[*j] == '#',
                    None => padder == '#',
                })
                .collect::<Vec<bool>>();

            // Insert the current pixel in the middle of the bits
            bits.insert(4, p_img[i] == '#');

            // Calculate the output for this pixel
            out.push(iea[lib::bits_to_usize(&bits)]);
        }

        // Overwrite the original shape with the padded shape
        s = p_shape;

        // Overwrite the input image with the output of this iteration
        img = out;
    }

    img.iter().filter(|c| **c == '#').fold(0, |acc, _| acc + 1)
}

pub fn puzzle2(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);
    0
}

#[cfg(test)]
mod tests {
    use crate::y2021::day20;

    #[test]
    fn test1() {
        assert_eq!(day20::puzzle1("src/y2021/day20/test.txt"), 35);

        assert!(day20::puzzle1("src/y2021/day20/input.txt") < 6548);

        assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5990);
        assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5303);
        assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5436);
        assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5092);
        assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5570);
        assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 6112);

        assert_eq!(day20::puzzle1("src/y2021/day20/input.txt"), 5425);
    }

    #[test]
    fn test2() {
        assert_eq!(day20::puzzle2("src/y2021/day20/test.txt"), 1);
        assert_eq!(day20::puzzle2("src/y2021/day20/input.txt"), 1);
    }
}
