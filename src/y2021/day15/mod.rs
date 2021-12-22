use crate::lib;

fn mapped_value(values: &Vec<u8>, orig_shape: &lib::Shape, shape: &lib::Shape, i: usize) -> u8 {
    let scaled_p = lib::i_to_point(shape, i);
    let orig_p = lib::Point::new(scaled_p.x % orig_shape.w, scaled_p.y % orig_shape.h);

    let v = values[lib::point_to_i(orig_shape, &orig_p)];

    if orig_p == scaled_p {
        return v;
    }

    let c = lib::Point::new(scaled_p.x / orig_shape.w, scaled_p.y / orig_shape.h);

    (v - 1 + (c.x + c.y) as u8) % 9 + 1
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let shape = lib::Shape::new(input[0].len(), input.len());

    let values: Vec<u8> = input
        .iter()
        .flat_map(|s| s.chars())
        .map(|c: char| c.to_digit(10).unwrap() as u8)
        .collect();

    *lib::dijkstra(&shape, 0, shape.len() - 1, |v| values[v] as usize)
        .last() // target is the last in the array
        .unwrap()
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let shape = lib::Shape::new(input[0].len(), input.len());

    let values: Vec<u8> = input
        .iter()
        .flat_map(|s| s.chars())
        .map(|c: char| c.to_digit(10).unwrap() as u8)
        .collect();

    let scaled_shape = shape * 5;

    *lib::dijkstra(&scaled_shape, 0, scaled_shape.len() - 1, |v| {
        mapped_value(&values, &shape, &scaled_shape, v) as usize
    })
    .last() // target is the last in the array
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::y2021::day15;

    #[test]
    fn test1() {
        assert_eq!(day15::puzzle1("src/y2021/day15/test.txt"), 40);
        assert_eq!(day15::puzzle1("src/y2021/day15/input.txt"), 398);
    }

    #[test]
    fn test2() {
        assert_eq!(day15::puzzle2("src/y2021/day15/test.txt"), 315);
        assert_eq!(day15::puzzle2("src/y2021/day15/input.txt"), 2817);
    }
}
