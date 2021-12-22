use crate::lib;
use std::collections::HashSet;

fn increment_elements(by: u8, v: &Vec<u8>) -> Vec<u8> {
    v.iter().map(|e| e + by).collect()
}

fn step_flashed(values: &mut Vec<u8>, s: &lib::Shape) -> HashSet<usize> {
    let mut flashed: HashSet<usize> = HashSet::new();

    let mut stack: Vec<usize> = values
        .iter()
        .enumerate()
        .filter(|(_, v)| **v > 9)
        .map(|(i, _)| i)
        .collect();

    while !stack.is_empty() {
        let i = stack.pop().unwrap();
        if flashed.insert(i) {
            for j in lib::neighbours(s, i, true) {
                values[j] += 1;
                if values[j] > 9 {
                    stack.push(j);
                }
            }
        }
    }

    flashed
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let s = lib::Shape::new(input[0].len(), input.len());

    let mut values: Vec<u8> = input
        .iter()
        .flat_map(|s| s.chars())
        .map(|c: char| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut flashes = 0;

    for _ in 0..100 {
        // Step 1
        values = increment_elements(1, &values);

        // Step 2
        let flashed: HashSet<usize> = step_flashed(&mut values, &s);

        // Step 3
        for i in flashed.iter() {
            values[*i] = 0;
        }

        flashes += flashed.len();
    }

    flashes
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let s = lib::Shape::new(input[0].len(), input.len());

    let mut values: Vec<u8> = input
        .iter()
        .flat_map(|s| s.chars())
        .map(|c: char| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut step: usize = 1;

    loop {
        // Step 1
        values = increment_elements(1, &values);

        // Step 2
        let flashed: HashSet<usize> = step_flashed(&mut values, &s);

        // Step 3
        for i in flashed.iter() {
            values[*i] = 0;
        }

        if flashed.len() == values.len() {
            break;
        }

        step += 1;
    }

    step
}

#[cfg(test)]
mod tests {
    use crate::y2021::day11;

    #[test]
    fn test1() {
        assert_eq!(day11::puzzle1("src/y2021/day11/test.txt"), 1656);
        assert_eq!(day11::puzzle1("src/y2021/day11/input.txt"), 1644);
    }

    #[test]
    fn test2() {
        assert_eq!(day11::puzzle2("src/y2021/day11/test.txt"), 195);
        assert_eq!(day11::puzzle2("src/y2021/day11/input.txt"), 229);
    }
}
