use crate::lib;

pub fn puzzle1(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);
    0
}

pub fn puzzle2(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);
    0
}

#[cfg(test)]
mod tests {
    use crate::y2021::day19;

    #[test]
    fn test1() {
        assert_eq!(day19::puzzle1("src/y2021/day19/test.txt"), 1);
        assert_eq!(day19::puzzle1("src/y2021/day19/input.txt"), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(day19::puzzle2("src/y2021/day19/test.txt"), 1);
        assert_eq!(day19::puzzle2("src/y2021/day19/input.txt"), 1);
    }
}
