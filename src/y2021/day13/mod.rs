use std::collections::HashSet;

use crate::lib;

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let mut dots: Vec<lib::Point> = input
        .iter()
        .map(|s| {
            s.split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .filter_map(|v: Vec<usize>| {
            if v.len() == 2 {
                Some(lib::Point::new(v[0], v[1]))
            } else {
                None
            }
        })
        .collect();

    let folds: Vec<lib::Point> = input
        .iter()
        .filter(|s| s.starts_with("fold along"))
        .map(|s| s.replace("fold along ", ""))
        .map(|s| {
            let ss: Vec<&str> = s.split("=").collect();
            let v = ss[1].parse::<usize>().unwrap();
            if ss[0] == "x" {
                vec![v, 0]
            } else {
                vec![0, v]
            }
        })
        .filter_map(|v: Vec<usize>| {
            if v.len() == 2 {
                Some(lib::Point::new(v[0], v[1]))
            } else {
                None
            }
        })
        .collect();

    for fold in folds.iter().take(1) {
        if fold.x > 0 {
            dots = dots
                .into_iter()
                .map(|d| {
                    if d.x > fold.x {
                        // fold along y (change x)
                        let new_x = fold.x - (d.x - fold.x);
                        lib::Point::new(new_x, d.y)
                    } else {
                        d
                    }
                })
                .collect()
        } else if fold.y > 0 {
            dots = dots
                .into_iter()
                .map(|d| {
                    if d.y > fold.y {
                        // fold along x (change y)
                        let new_y = fold.y - (d.y - fold.y);
                        lib::Point::new(d.x, new_y)
                    } else {
                        d
                    }
                })
                .collect()
        }
    }

    let mut r: HashSet<lib::Point> = HashSet::new();
    for d in dots.iter() {
        r.insert(*d);
    }

    r.len()
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let mut dots: Vec<lib::Point> = input
        .iter()
        .map(|s| {
            s.split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .filter_map(|v: Vec<usize>| {
            if v.len() == 2 {
                Some(lib::Point::new(v[0], v[1]))
            } else {
                None
            }
        })
        .collect();

    let folds: Vec<lib::Point> = input
        .iter()
        .filter(|s| s.starts_with("fold along"))
        .map(|s| s.replace("fold along ", ""))
        .map(|s| {
            let ss: Vec<&str> = s.split("=").collect();
            let v = ss[1].parse::<usize>().unwrap();
            if ss[0] == "x" {
                vec![v, 0]
            } else {
                vec![0, v]
            }
        })
        .filter_map(|v: Vec<usize>| {
            if v.len() == 2 {
                Some(lib::Point::new(v[0], v[1]))
            } else {
                None
            }
        })
        .collect();

    for fold in folds.iter() {
        if fold.x > 0 {
            dots = dots
                .into_iter()
                .map(|d| {
                    if d.x > fold.x {
                        // fold along y (change x)
                        lib::Point::new(fold.x - (d.x - fold.x), d.y)
                    } else {
                        d
                    }
                })
                .collect()
        } else if fold.y > 0 {
            dots = dots
                .into_iter()
                .map(|d| {
                    if d.y > fold.y {
                        // fold along x (change y)
                        lib::Point::new(d.x, fold.y - (d.y - fold.y))
                    } else {
                        d
                    }
                })
                .collect()
        }
    }

    let mut r: HashSet<lib::Point> = HashSet::new();
    for d in dots.iter() {
        r.insert(*d);
    }

    let s = lib::Shape::new_from_points(&dots);

    for y in 0..s.h {
        for x in 0..s.w {
            if dots.iter().find(|d| d.x == x && d.y == y).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    r.len()
}

#[cfg(test)]
mod tests {
    use crate::y2021::day13;

    #[test]
    fn test1() {
        assert_eq!(day13::puzzle1("src/y2021/day13/test.txt"), 17);
        assert_ne!(day13::puzzle1("src/y2021/day13/input.txt"), 102);
    }

    #[test]
    fn test2() {
        assert_eq!(day13::puzzle2("src/y2021/day13/test.txt"), 16);
        // assert_eq!(day13::puzzle2("src/y2021/day13/input.txt"), 1);
    }
}
