use crate::lib;

fn tick(state: [usize; 9]) -> [usize; 9] {
    let mut next_state: [usize; 9] = [0; 9];
    let zeros = state[0];

    for i in 0..8 {
        next_state[i] = state[i + 1];
    }

    next_state[6] += zeros;
    next_state[8] = zeros;

    next_state
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let mut state = input[0].split(",").map(|s| s.parse().unwrap()).fold(
        [0; 9],
        |mut m: [usize; 9], v: usize| {
            m[v] += 1;
            m
        },
    );

    for _ in 0..80 {
        state = tick(state);
    }

    state.iter().fold(0, |mut acc: usize, c| {
        acc += c;
        acc
    })
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let mut state = input[0].split(",").map(|s| s.parse().unwrap()).fold(
        [0; 9],
        |mut m: [usize; 9], v: usize| {
            m[v] += 1;
            m
        },
    );

    for _ in 0..256 {
        state = tick(state);
    }

    state.iter().fold(0, |mut acc: usize, c| {
        acc += c;
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::y2021::day06;

    #[test]
    fn test1() {
        assert_eq!(day06::puzzle1("src/y2021/day06/test.txt"), 5934);
        assert_eq!(day06::puzzle1("src/y2021/day06/input.txt"), 362346);
    }

    #[test]
    fn test2() {
        assert_eq!(day06::puzzle2("src/y2021/day06/test.txt"), 26984457539);
        assert_eq!(day06::puzzle2("src/y2021/day06/input.txt"), 1639643057051);
    }
}
