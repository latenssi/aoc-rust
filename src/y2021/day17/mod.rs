use crate::lib;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Area {
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
}

impl Area {
    pub fn new() -> Area {
        Area {
            x1: 0,
            x2: 0,
            y1: 0,
            y2: 0,
        }
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        let Point { x: px, y: py } = p;
        px >= &self.x1 && px <= &self.x2 && py <= &self.y1 && py >= &self.y2
    }

    pub fn point_has_passed(&self, p: &Point) -> bool {
        let Point { x: px, y: py } = p;
        px > &self.x2 || py < &self.y2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct State {
    area: Area,
    point: Point,
    velocity: Point,
}

impl Iterator for State {
    type Item = State;
    fn next(&mut self) -> Option<Self::Item> {
        let x_vel = if self.velocity.x > 0 {
            self.velocity.x - 1
        } else if self.velocity.x < 0 {
            self.velocity.x + 1
        } else {
            self.velocity.x
        };

        let new_state = State {
            area: self.area,
            point: Point {
                x: self.point.x + self.velocity.x,
                y: self.point.y + self.velocity.y,
            },
            velocity: Point {
                x: x_vel,
                y: self.velocity.y - 1,
            },
        };

        if new_state.area.point_has_passed(&new_state.point) {
            None
        } else {
            Some(new_state)
        }
    }
}

fn parse_area(s: &str) -> Area {
    s.replace("target area: ", "")
        .replace("x=", "")
        .replace("y=", "")
        .split(",")
        .flat_map(|s| s.split(".."))
        .filter_map(|s| s.trim().to_string().parse().ok())
        .enumerate()
        .fold(Area::new(), |mut area, (i, c)| {
            match i {
                0 => area.x1 = c,
                1 => area.x2 = c,
                2 => area.y2 = c,
                3 => area.y1 = c,
                _ => {}
            }
            area
        })
}

pub fn puzzle1(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);
    let area = parse_area(&input[0]);

    let mut max_y_pos = 0;

    for x_vel in 0..100 {
        for y_vel in -500..500 {
            let mut state = State {
                area,
                point: Point { x: 0, y: 0 },
                velocity: Point { x: x_vel, y: y_vel },
            };
            let mut hist: Vec<isize> = vec![];
            while let Some(s) = state.next() {
                state = s;
                hist.push(s.point.y);
                if s.area.contains_point(&s.point) {
                    let max_y = hist.into_iter().max().unwrap_or_default();
                    if max_y > max_y_pos {
                        max_y_pos = max_y;
                    }
                    break;
                }
            }
        }
    }

    max_y_pos
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let area = parse_area(&input[0]);
    let start = Point { x: 0, y: 0 };

    let mut valids = 0;

    for x_vel in 0..1000 {
        for y_vel in -500..500 {
            let mut state = State {
                area,
                point: start,
                velocity: Point { x: x_vel, y: y_vel },
            };
            while let Some(s) = state.next() {
                state = s;
                if s.area.contains_point(&s.point) {
                    valids += 1;
                    break;
                }
            }
        }
    }

    valids
}

#[cfg(test)]
mod tests {
    use crate::y2021::day17;

    #[test]
    fn test1() {
        assert_eq!(day17::puzzle1("src/y2021/day17/test.txt"), 45);
        assert_eq!(day17::puzzle1("src/y2021/day17/input.txt"), 3570);
    }

    #[test]
    fn test2() {
        assert_eq!(day17::puzzle2("src/y2021/day17/test.txt"), 112);
        assert_eq!(day17::puzzle2("src/y2021/day17/input.txt"), 1919);
    }
}
