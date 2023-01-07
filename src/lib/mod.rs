use std::fmt::Display;
use std::fs;
use std::ops::{Div, Mul};
use std::{cmp::Ordering, collections::BinaryHeap};

pub fn read_lines(input_filename: &str) -> Vec<String> {
    let input = fs::read_to_string(input_filename).unwrap();
    let lines = input.lines().map(str::to_string);
    lines.collect()
}

#[derive(Debug, Copy, Clone)]
pub struct Shape {
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Shape {
    pub fn new(w: usize, h: usize) -> Shape {
        Shape { w, h }
    }

    pub fn new_from_points(points: &Vec<Point>) -> Shape {
        points.iter().fold(Shape::new(0, 0), |mut s, d| {
            if d.x + 1 > s.w {
                s.w = d.x + 1;
            }
            if d.y + 1 > s.h {
                s.h = d.y + 1;
            }
            s
        })
    }

    pub fn len(self) -> usize {
        self.w * self.h
    }
}

impl Mul<usize> for Shape {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            w: self.w * rhs,
            h: self.h * rhs,
        }
    }
}

impl Div<usize> for Shape {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            w: self.w / rhs,
            h: self.h / rhs,
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

pub fn left(_: &Shape, p: &Point) -> Option<Point> {
    if p.x > 0 {
        Some(Point { x: p.x - 1, y: p.y })
    } else {
        None
    }
}

pub fn right(s: &Shape, p: &Point) -> Option<Point> {
    if p.x < s.w - 1 {
        Some(Point { x: p.x + 1, y: p.y })
    } else {
        None
    }
}

pub fn top(_: &Shape, p: &Point) -> Option<Point> {
    if p.y > 0 {
        Some(Point { x: p.x, y: p.y - 1 })
    } else {
        None
    }
}

pub fn bottom(s: &Shape, p: &Point) -> Option<Point> {
    if p.y < s.h - 1 {
        Some(Point { x: p.x, y: p.y + 1 })
    } else {
        None
    }
}

pub fn top_left(s: &Shape, p: &Point) -> Option<Point> {
    top(s, &left(s, p)?)
}

pub fn top_right(s: &Shape, p: &Point) -> Option<Point> {
    top(s, &right(s, p)?)
}

pub fn bottom_left(s: &Shape, p: &Point) -> Option<Point> {
    bottom(s, &left(s, p)?)
}

pub fn bottom_right(s: &Shape, p: &Point) -> Option<Point> {
    bottom(s, &right(s, p)?)
}

pub fn point_to_i(s: &Shape, p: &Point) -> usize {
    p.x + s.w * p.y
}

pub fn i_to_point(s: &Shape, i: usize) -> Point {
    Point {
        x: i % s.w,
        y: i / s.w,
    }
}

pub fn i_at_start(_: &Shape, i: usize) -> bool {
    i == 0
}

pub fn i_at_end(s: &Shape, i: usize) -> bool {
    i == s.len() - 1
}

pub fn point_at_start(_: &Shape, p: &Point) -> bool {
    p.x == 0 && p.y == 0
}

pub fn point_at_end(s: &Shape, p: &Point) -> bool {
    p.x == s.w - 1 && p.y == s.h - 1
}

const NEIGHBOURS_NOT_DIAG: [fn(&Shape, &Point) -> Option<Point>; 4] = [top, bottom, left, right];

const NEIGHBOURS_DIAG: [fn(&Shape, &Point) -> Option<Point>; 8] = [
    top_left,
    top,
    top_right,
    left,
    right,
    bottom_left,
    bottom,
    bottom_right,
];

pub fn neighbours(s: &Shape, i: usize, include_diag: bool) -> Vec<usize> {
    let p = i_to_point(s, i);

    (if !include_diag {
        NEIGHBOURS_NOT_DIAG.to_vec()
    } else {
        NEIGHBOURS_DIAG.to_vec()
    })
    .into_iter()
    .filter_map(|f| f(s, &p))
    .map(|p| point_to_i(s, &p))
    .collect()
}

pub fn neighbours_2(s: &Shape, i: usize, include_diag: bool) -> Vec<Option<usize>> {
    (if !include_diag {
        NEIGHBOURS_NOT_DIAG.to_vec()
    } else {
        NEIGHBOURS_DIAG.to_vec()
    })
    .into_iter()
    .map(|f| match f(s, &i_to_point(s, i)) {
        Some(p) => Some(point_to_i(s, &p)),
        None => None,
    })
    .collect()
}

pub fn map_i(from: &Shape, to: &Shape, offset: &Shape, i: usize) -> usize {
    let mut p = i_to_point(from, i);
    p.x += offset.w;
    p.y += offset.h;
    point_to_i(to, &p)
}

pub fn pad<T>(s: &Shape, img: &[T], pad_by: usize, padder: T) -> (Shape, Vec<T>)
where
    T: Clone + Copy,
{
    let new_shape = Shape::new(s.w + 2 * pad_by, s.h + 2 * pad_by);
    let mut new_image: Vec<T> = vec![padder; new_shape.len()];
    let offset = Shape::new(pad_by, pad_by);

    for i in 0..s.len() {
        let j = map_i(s, &new_shape, &offset, i);
        new_image[j] = img[i];
    }

    (new_shape, new_image)
}

pub fn print_image<T>(s: &Shape, img: &[T])
where
    T: Display,
{
    for y in 0..s.h {
        for x in 0..s.w {
            let v = &img[point_to_i(s, &Point { x, y })];
            print!("{}", v);
        }
        println!();
    }
    println!();
}

pub fn bits_to_usize(bits: &[bool]) -> usize {
    bits.iter().fold(0, |acc, &b| acc * 2 + b as usize)
}

// pub fn print_grid(grid: Vec<usize>, s: Shape) {
//     for y in 0..s.h {
//         for x in 0..s.w {
//             let v = grid[point_to_i(s, Point { x, y })];
//             print!("{}", v);
//         }
//         println!();
//     }
//     println!();
// }

#[derive(Copy, Clone, Eq, PartialEq)]
struct DijkstraState {
    cost: usize,
    position: usize,
}

// BinaryHeap for dijkstra requires flipped ordering along the cost axis
impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra<F>(shape: &Shape, start: usize, target: usize, cost_fn: F) -> Vec<usize>
where
    F: Fn(usize) -> usize,
{
    let mut heap: BinaryHeap<DijkstraState> = BinaryHeap::new();
    let mut dist: Vec<_> = (0..shape.len()).map(|_| usize::MAX).collect();

    dist[start] = 0;

    heap.push(DijkstraState {
        cost: 0,
        position: start,
    });

    while let Some(DijkstraState { cost, position: u }) = heap.pop() {
        if u == target {
            break;
        }

        if cost > dist[u] {
            continue;
        }

        for v in neighbours(&shape, u, false) {
            let next = DijkstraState {
                cost: cost + cost_fn(v),
                position: v,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    return dist;
}

pub fn modulo(a: usize, b: usize) -> usize {
    ((a % b) + b) % b
}
