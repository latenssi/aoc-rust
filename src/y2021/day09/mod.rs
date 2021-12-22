use crate::lib;
use std::collections::HashSet;

fn depth_first_search(values: &Vec<u8>, shape: &lib::Shape, i: usize) -> HashSet<usize> {
    let mut discovered: HashSet<usize> = HashSet::new();
    let mut stack: Vec<usize> = vec![];
    stack.push(i);
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        if discovered.insert(v) {
            for n in lib::neighbours(shape, v, false)
                .into_iter()
                .filter(|j| values[*j] != 9)
            {
                stack.push(n);
            }
        }
    }
    discovered
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let shape = lib::Shape::new(input[0].len(), input.len());

    let values: Vec<u8> = input
        .iter()
        .flat_map(|s| s.chars())
        .map(|c: char| c.to_digit(10).unwrap() as u8)
        .collect();

    values
        .iter()
        .enumerate()
        .filter(|(i, v)| {
            lib::neighbours(&shape, *i, false)
                .into_iter()
                .map(|j| values[j])
                .fold(true, |acc, cur| acc && *v < &cur)
        })
        .map(|(_, v)| *v)
        .fold(0, |acc, v| acc + v as usize + 1)
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let shape = lib::Shape::new(input[0].len(), input.len());

    let values: Vec<u8> = input
        .iter()
        .flat_map(|s| s.chars())
        .map(|c: char| c.to_digit(10).unwrap() as u8)
        .collect();

    let low_points: Vec<usize> = values
        .iter()
        .enumerate()
        .filter(|(i, v)| {
            lib::neighbours(&shape, *i, false)
                .into_iter()
                .map(|j| values[j])
                .fold(true, |acc, cur| acc && *v < &cur)
        })
        .map(|(i, _)| i)
        .collect();

    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|i| depth_first_search(&values, &shape, *i).len())
        .collect();

    basin_sizes.sort();
    basin_sizes.reverse();

    basin_sizes[0..3].iter().fold(1, |prod, v| prod * v)
}

#[cfg(test)]
mod tests {
    use crate::y2021::day09;

    #[test]
    fn test1() {
        assert_eq!(day09::puzzle1("src/y2021/day09/test.txt"), 15);
        assert_eq!(day09::puzzle1("src/y2021/day09/input.txt"), 541);
    }

    #[test]
    fn test2() {
        assert_eq!(day09::puzzle2("src/y2021/day09/test.txt"), 1134);
        assert_eq!(day09::puzzle2("src/y2021/day09/input.txt"), 847504);
    }
}
