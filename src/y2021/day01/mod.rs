use std::cmp::PartialOrd;
use std::ops::Add;

use crate::lib;

fn look_behind_1_gt_count<'a, T: PartialOrd>(
    previous: Option<(usize, &'a T)>,
    current: &'a T,
) -> Option<(usize, &'a T)> {
    match previous {
        Some(x) => Some((if current > x.1 { x.0 + 1 } else { x.0 }, current)),
        None => Some((0, current)), // First iteration, no previous
    }
}

fn look_behind_2_sum<T: Default + Add<Output = T> + Copy + Clone>(
    previous: Option<(Vec<T>, T, T)>,
    current: &T,
) -> Option<(Vec<T>, T, T)> {
    match previous {
        Some(mut x) => {
            x.0.push(x.1 + x.2 + *current);
            Some((x.0, x.2, *current))
        }
        None => Some((vec![], T::default(), *current)), // First iteration, no previous
    }
}

pub fn puzzle1(input_filename: &str) -> usize {
    let parsed: Vec<i32> = lib::read_lines(input_filename)
        .iter()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    parsed
        .iter()
        .fold(None, look_behind_1_gt_count)
        .expect("error while counting greater than count")
        .0
}

pub fn puzzle2(input_filename: &str) -> usize {
    let parsed: Vec<i32> = lib::read_lines(input_filename)
        .iter()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    parsed
        .iter()
        .fold(None, look_behind_2_sum)
        .expect("error while counting sliding window sums")
        .0
        .iter()
        .skip(1)
        .fold(None, look_behind_1_gt_count)
        .expect("error while counting greater than count")
        .0
}

#[cfg(test)]
mod tests {
    use crate::y2021::day01;

    #[test]
    fn test1() {
        assert_eq!(day01::puzzle1("src/y2021/day01/test.txt"), 7);
        assert_eq!(day01::puzzle1("src/y2021/day01/input.txt"), 1624);
    }

    #[test]
    fn test2() {
        assert_eq!(day01::puzzle2("src/y2021/day01/test.txt"), 5);
        assert_eq!(day01::puzzle2("src/y2021/day01/input.txt"), 1653);
    }
}
