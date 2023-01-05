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

    let padding = 1;

    println!("Original image:");
    lib::print_image(&s, &img);

    for n in 0..2 {
        // Create a padded image and corresponding shape as we are dealing with an "infinite" image
        // and we need to be able to check the neighbours of the pixels on the edge of the image
        let (p_shape, p_img) = lib::pad(&s, &img, padding, '.');

        println!("Padded image, iteration {}", n + 1);
        lib::print_image(&p_shape, &p_img);

        // Create a vector to store the output of this iteration
        let mut out = vec!['.'; p_img.len()];

        for i in 0..p_img.len() {
            let mut bits = lib::neighbours(&p_shape, i, true)
                .iter()
                .map(|k| p_img[*k] == '#')
                .collect::<Vec<bool>>();

            // Check if we are on the edge of the image and dealing with a padding
            if bits.len() != 8 {
                continue;
            }

            // Insert the current pixel in the middle of the bits
            bits.insert(4, p_img[i] == '#');

            // Calculate the output for this pixel
            out[i] = iea[lib::bits_to_usize(&bits)];
        }

        // Overwrite the original shape with the padded shape
        s = p_shape;

        println!("Output image, iteration {}", n + 1);
        lib::print_image(&s, &out);

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

        // Padding of 3
        // assert!(day20::puzzle1("src/y2021/day20/input.txt") < 6548);

        // // Padding of 2
        // assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5990);

        // // Padding of 1
        // assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5303);

        // assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5436);

        // assert_eq!(day20::puzzle1("src/y2021/day20/input.txt"), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(day20::puzzle2("src/y2021/day20/test.txt"), 1);
        assert_eq!(day20::puzzle2("src/y2021/day20/input.txt"), 1);
    }
}
