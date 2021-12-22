use crate::lib;

pub fn puzzle1(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);

    input[0].chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    input[0]
        .chars()
        .enumerate()
        .fold((0, 0), |(i, acc), c| {
            if acc < 0 {
                (i, acc)
            } else {
                match c.1 {
                    '(' => (c.0 + 1, acc + 1),
                    ')' => (c.0 + 1, acc - 1),
                    _ => (i, acc),
                }
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use crate::y2015::day01;

    #[test]
    fn test1() {
        assert_eq!(day01::puzzle1("src/y2015/day01/test.txt"), -3);
        assert_eq!(day01::puzzle1("src/y2015/day01/input.txt"), 138);
    }

    #[test]
    fn test2() {
        assert_eq!(day01::puzzle2("src/y2015/day01/test2.txt"), 5);
        assert_eq!(day01::puzzle2("src/y2015/day01/input.txt"), 1771);
    }
}
