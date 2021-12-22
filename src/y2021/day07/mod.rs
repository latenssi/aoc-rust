use crate::lib;

fn binom(n: usize, k: usize) -> usize {
    let mut res = 1;
    for i in 0..k {
        res = (res * (n - i)) / (i + 1);
    }
    res
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let x: Vec<usize> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    let x_min = *x.iter().min().unwrap();
    let x_max = *x.iter().max().unwrap();

    let mut min_err = std::usize::MAX;

    for t in x_min..x_max + 1 {
        let err = x.iter().fold(0, |mut acc: usize, v| {
            acc += (*v as i32 - t as i32).abs() as usize;
            acc
        });

        if err < min_err {
            min_err = err
        }
    }

    min_err
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let x: Vec<usize> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    let x_min = *x.iter().min().unwrap();
    let x_max = *x.iter().max().unwrap();

    let mut min_err = std::usize::MAX;

    for t in x_min..x_max + 1 {
        let err = x.iter().fold(0, |mut acc: usize, v| {
            let steps = (*v as i32 - t as i32).abs() as usize;
            acc += binom(steps + 1, 2);
            acc
        });

        if err < min_err {
            min_err = err
        }
    }

    min_err
}

#[cfg(test)]
mod tests {
    use crate::y2021::day07;

    #[test]
    fn test1() {
        assert_eq!(day07::puzzle1("src/y2021/day07/test.txt"), 37);
        assert_eq!(day07::puzzle1("src/y2021/day07/input.txt"), 364898);
    }

    #[test]
    fn test2() {
        assert_eq!(day07::puzzle2("src/y2021/day07/test.txt"), 168);
        assert_eq!(day07::puzzle2("src/y2021/day07/input.txt"), 104149091);
    }
}
