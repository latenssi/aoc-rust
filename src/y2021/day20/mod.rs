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

    let padding = 3;

    lib::print_image(&s, &img);

    for _ in 0..2 {
        let (p_shape, p_img) = lib::pad(&s, &img, padding, '.');

        let mut out = vec!['.'; p_img.len()];

        for i in 0..p_img.len() {
            let mut bits = lib::neighbours(&p_shape, i, true)
                .iter()
                .map(|k| p_img[*k] == '#')
                .collect::<Vec<bool>>();

            if bits.len() == 8 {
                // Insert the current "pixel"
                bits.insert(4, p_img[i] == '#');
                out[i] = iea[lib::bits_to_usize(&bits)];
            }
        }

        s = p_shape;
        img = out;

        lib::print_image(&s, &img);
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
        assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 6548);
        assert_ne!(day20::puzzle1("src/y2021/day20/input.txt"), 5436);
        assert_eq!(day20::puzzle1("src/y2021/day20/input.txt"), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(day20::puzzle2("src/y2021/day20/test.txt"), 1);
        assert_eq!(day20::puzzle2("src/y2021/day20/input.txt"), 1);
    }
}
