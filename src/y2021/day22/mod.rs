use std::ops::Sub;

use crate::lib;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Sub for Point3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug, Clone)]
struct Cuboid {
    p1: Point3,
    p2: Point3,
    holes: Vec<Cuboid>,
}

impl Cuboid {
    pub fn new() -> Cuboid {
        Cuboid {
            p1: Point3 { x: 0, y: 0, z: 0 },
            p2: Point3 { x: 0, y: 0, z: 0 },
            holes: vec![],
        }
    }

    pub fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        let p1 = self.p1.max(other.p1);
        let p2 = self.p2.min(other.p2);
        if p2 > p1 {
            Some(Cuboid {
                p1,
                p2: p2,
                holes: vec![],
            })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        let p = self.p1 - self.p2;
        let holes_len: usize = self.holes.iter().map(|c| c.len()).sum();
        (p.x * p.y * p.z) as usize - holes_len
    }
}

#[derive(Debug)]
struct Step {
    on: bool,
    cuboid: Cuboid,
}

impl Step {
    pub fn new() -> Step {
        Step {
            on: false,
            cuboid: Cuboid::new(),
        }
    }

    pub fn from_str(s: &str) -> Step {
        let split = s.split_whitespace().collect::<Vec<&str>>();
        let xyz = split[1]
            .split(",")
            .map(|s| {
                s[2..]
                    .split("..")
                    .filter_map(|s| s.parse::<isize>().ok())
                    .collect()
            })
            .collect::<Vec<Vec<isize>>>();
        Step {
            on: split[0] == "on",
            cuboid: Cuboid {
                p1: Point3 {
                    x: xyz[0][0],
                    y: xyz[1][0],
                    z: xyz[2][0],
                },
                p2: Point3 {
                    x: xyz[0][1],
                    y: xyz[1][1],
                    z: xyz[2][1],
                },
                holes: vec![],
            },
        }
    }
}

pub fn puzzle1(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);
    let mut cuboids_on: Vec<Cuboid> = vec![];
    let steps = input
        .iter()
        .map(|s| Step::from_str(s))
        .collect::<Vec<Step>>();

    let limit = Cuboid {
        p1: Point3 {
            x: -50,
            y: -50,
            z: -50,
        },
        p2: Point3 {
            x: 50,
            y: 50,
            z: 50,
        },
        holes: vec![],
    };

    for step in steps {
        // Limit the cuboids to the given volume
        if let Some(cuboid) = limit.intersect(&step.cuboid) {
            if step.on {
                cuboids_on.push(step.cuboid.clone());
            } else {
                for c in cuboids_on.iter_mut() {
                    if let Some(hole) = c.intersect(&cuboid) {
                        c.holes.push(hole);
                    }
                }
            }
            println!("{:?}, {:?}, {}", step.cuboid, cuboid, cuboid.len());
        }
    }

    for i in 0..cuboids_on.len() {
        for j in i + 1..cuboids_on.len() {
            if i != j {
                if let Some(shared) = cuboids_on[i].intersect(&cuboids_on[j]) {
                    cuboids_on[i].holes.push(shared);
                }
            }
        }
    }

    // println!("{:?}", steps);
    0
}

pub fn puzzle2(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);
    0
}

#[cfg(test)]
mod tests {
    use crate::y2021::day22;

    #[test]
    fn test1() {
        assert_eq!(day22::puzzle1("src/y2021/day22/test.txt"), 1);
        // assert_eq!(day22::puzzle1("src/y2021/day22/input.txt"), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(day22::puzzle2("src/y2021/day22/test.txt"), 1);
        assert_eq!(day22::puzzle2("src/y2021/day22/input.txt"), 1);
    }
}
