use crate::lib;

fn string_to_bin_vec(s: &String) -> Vec<bool> {
    s.chars()
        .map(|c| match c {
            '1' => true,
            _ => false,
        })
        .collect()
}

fn count_ones(mut state: Vec<usize>, current: &Vec<bool>) -> Vec<usize> {
    if state.len() < current.len() {
        state.resize(current.len(), 0)
    }

    for (i, b) in current.iter().enumerate() {
        if *b {
            state[i] += 1
        }
    }

    state
}

fn threshold(src: &Vec<usize>, limit: usize, remainder: usize) -> Vec<bool> {
    src.iter().fold(vec![], |mut s, v| {
        s.push(*v >= limit + remainder);
        s
    })
}

fn bin_vec_to_dec(vec: &Vec<bool>) -> usize {
    let s: String = vec
        .iter()
        .map(|x| char::from_digit(*x as u32, 2).expect("error"))
        .collect();
    usize::from_str_radix(&s, 2).unwrap_or(0)
}

pub fn puzzle1(input_filename: &str) -> usize {
    let parsed: Vec<Vec<bool>> = lib::read_lines(input_filename)
        .iter()
        .map(string_to_bin_vec)
        .collect();

    let ones = parsed.iter().fold(vec![], count_ones);

    let thresh = threshold(&ones, parsed.len() / 2, parsed.len() % 2);
    let thresh_inv = thresh.iter().map(|b| !*b).collect();

    let gamma = bin_vec_to_dec(&thresh);
    let epsilon = bin_vec_to_dec(&thresh_inv);

    gamma * epsilon
}

pub fn puzzle2(input_filename: &str) -> usize {
    let parsed: Vec<Vec<bool>> = lib::read_lines(input_filename)
        .iter()
        .map(string_to_bin_vec)
        .collect();

    let mut tmp = parsed.clone();

    for i in 0..=parsed[0].len() {
        let ones = tmp.iter().fold(vec![], count_ones);
        let thresh = threshold(&ones, tmp.len() / 2, tmp.len() % 2);
        tmp = tmp.into_iter().filter(|v| v[i] == thresh[i]).collect();
        if tmp.len() <= 1 {
            break;
        }
    }

    let oxygen = bin_vec_to_dec(&tmp[0]);

    let mut tmp = parsed.clone();

    for i in 0..=parsed[0].len() {
        let ones = tmp.iter().fold(vec![], count_ones);
        let thresh = threshold(&ones, tmp.len() / 2, tmp.len() % 2);
        tmp = tmp.into_iter().filter(|v| v[i] != thresh[i]).collect();
        if tmp.len() <= 1 {
            break;
        }
    }

    let co2 = bin_vec_to_dec(&tmp[0]);

    oxygen * co2
}

#[cfg(test)]
mod tests {
    use crate::y2021::day03;

    #[test]
    fn test1() {
        assert_eq!(day03::puzzle1("src/y2021/day03/test.txt"), 198);
        assert_eq!(day03::puzzle1("src/y2021/day03/input.txt"), 1131506);
    }

    #[test]
    fn test2() {
        assert_eq!(day03::puzzle2("src/y2021/day03/test.txt"), 230);
        assert_eq!(day03::puzzle2("src/y2021/day03/input.txt"), 7863147);
    }
}
