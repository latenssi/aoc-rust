use crate::lib;

pub fn puzzle(input_filename: &str, iterations: usize) -> usize {
    let input = lib::read_lines(input_filename);
    let iea = &input[0].chars().collect::<Vec<char>>();
    let mut img = input
        .iter()
        .skip(2)
        .filter(|s| **s != "")
        .flat_map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let mut shape = lib::Shape::new(input[2].len(), img.len() / input[2].len());

    let mut padder = '.';

    for _ in 0..iterations {
        enhance(iea, &mut padder, &mut img, &mut shape);
    }

    img.iter().filter(|c| **c == '#').fold(0, |acc, _| acc + 1)
}

fn enhance(iea: &Vec<char>, padder: &mut char, img: &mut Vec<char>, shape: &mut lib::Shape) {
    let c_padder = *padder;

    // Create a padded shape and image, pad by 1 pixel in all directions
    let (p_shape, p_img) = lib::pad(shape, img, 1, c_padder);

    // Create a vector to store the output of this iteration
    let mut out: Vec<char> = vec![];

    for i in 0..p_img.len() {
        // Get the neighbours of the current pixel and convert them to bools.
        // 'neighbours_2' returns a vector of Option<usize>, of indexes of the neighbours.
        // The Option is None when we are looking at a pixel outside the shape.
        let mut bits = lib::neighbours_2(&p_shape, i, true)
            .iter()
            .map(|j| match j {
                Some(j) => p_img[*j] == '#',
                None => c_padder == '#',
            })
            .collect::<Vec<bool>>();

        // Insert the current pixel in the middle of the bits
        bits.insert(4, p_img[i] == '#');

        // Calculate the output for this pixel
        out.push(iea[lib::bits_to_usize(&bits)]);
    }

    // Overwrite the original shape with the padded shape
    *shape = p_shape;

    // Overwrite the input image with the output of this iteration
    *img = out;

    // Determine the next padding character
    *padder = match c_padder {
        '#' => iea[511],
        '.' => iea[0],
        _ => panic!("Invalid char"),
    };
}

#[cfg(test)]
mod tests {
    use crate::y2021::day20;

    #[test]
    fn test1() {
        assert_eq!(day20::puzzle("src/y2021/day20/test.txt", 2), 35);

        assert!(day20::puzzle("src/y2021/day20/input.txt", 2) < 6548);

        assert_ne!(day20::puzzle("src/y2021/day20/input.txt", 2), 5990);
        assert_ne!(day20::puzzle("src/y2021/day20/input.txt", 2), 5303);
        assert_ne!(day20::puzzle("src/y2021/day20/input.txt", 2), 5436);
        assert_ne!(day20::puzzle("src/y2021/day20/input.txt", 2), 5092);
        assert_ne!(day20::puzzle("src/y2021/day20/input.txt", 2), 5570);
        assert_ne!(day20::puzzle("src/y2021/day20/input.txt", 2), 6112);

        assert_eq!(day20::puzzle("src/y2021/day20/input.txt", 2), 5425);
    }

    #[test]
    fn test2() {
        assert_eq!(day20::puzzle("src/y2021/day20/test.txt", 50), 3351);
        assert_eq!(day20::puzzle("src/y2021/day20/input.txt", 50), 14052);
    }
}
