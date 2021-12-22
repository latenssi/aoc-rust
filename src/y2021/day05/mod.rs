use crate::lib;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct LineSegment {
    p1: Point,
    p2: Point,
}

fn string_to_line(s: &String) -> Option<LineSegment> {
    let coords: Vec<usize> = s
        .split(" -> ")
        .flat_map(|s| s.split(","))
        .map(|s| s.parse().unwrap())
        .collect();

    if coords.len() != 4 {
        return None;
    }

    let p1 = Point {
        x: coords[0],
        y: coords[1],
    };

    let p2 = Point {
        x: coords[2],
        y: coords[3],
    };

    Some(LineSegment { p1, p2 })
}

fn line_is_horizontal(l: &LineSegment) -> bool {
    l.p1.y == l.p2.y
}

fn line_is_vertical(l: &LineSegment) -> bool {
    l.p1.x == l.p2.x
}

fn line_is_horizontal_or_vertical(l: &LineSegment) -> bool {
    line_is_horizontal(l) || line_is_vertical(l)
}

fn line_all_points(l: &LineSegment) -> Vec<Point> {
    let min_x = std::cmp::min(l.p1.x, l.p2.x);
    let max_x = std::cmp::max(l.p1.x, l.p2.x);
    let min_y = std::cmp::min(l.p1.y, l.p2.y);
    let max_y = std::cmp::max(l.p1.y, l.p2.y);

    if line_is_horizontal(l) {
        // Horizontal
        return (min_x..max_x + 1)
            .into_iter()
            .map(|x| Point { x, y: l.p1.y })
            .collect();
    } else if line_is_vertical(l) {
        // Vertical
        return (min_y..max_y + 1)
            .into_iter()
            .map(|y| Point { x: l.p1.x, y })
            .collect();
    } else {
        // Diagonal 45 degrees
        let mut xs: Vec<usize> = (min_x..max_x + 1).collect();
        let mut ys: Vec<usize> = (min_y..max_y + 1).collect();

        if l.p1.x > l.p2.x {
            xs.reverse();
        }

        if l.p1.y > l.p2.y {
            ys.reverse()
        }

        return xs
            .into_iter()
            .zip(ys)
            .map(|p| Point { x: p.0, y: p.1 })
            .collect();
    }
}

fn count_overlapping_points(
    mut state: HashMap<Point, usize>,
    current: &Point,
) -> HashMap<Point, usize> {
    *state.entry(*current).or_default() += 1;
    state
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let lines: Vec<LineSegment> = input
        .iter()
        .filter_map(string_to_line)
        .filter(line_is_horizontal_or_vertical)
        .collect();
    let all_points: Vec<Point> = lines.iter().flat_map(line_all_points).collect();

    all_points
        .iter()
        .fold(HashMap::<Point, usize>::new(), count_overlapping_points)
        .into_iter()
        .filter(|(_, v)| *v > 1)
        .count()
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let lines: Vec<LineSegment> = input.iter().filter_map(string_to_line).collect();
    let all_points: Vec<Point> = lines.iter().flat_map(line_all_points).collect();

    all_points
        .iter()
        .fold(HashMap::<Point, usize>::new(), count_overlapping_points)
        .into_iter()
        .filter(|(_, v)| *v > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::y2021::day05;

    #[test]
    fn test1() {
        assert_eq!(day05::puzzle1("src/y2021/day05/test.txt"), 5);
        assert_eq!(day05::puzzle1("src/y2021/day05/input.txt"), 7318);
    }

    #[test]
    fn test2() {
        assert_eq!(day05::puzzle2("src/y2021/day05/test.txt"), 12);
        assert_eq!(day05::puzzle2("src/y2021/day05/input.txt"), 19939);
    }
}
