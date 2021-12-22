use crate::lib;

#[derive(Debug, Copy, Clone)]
enum DIR {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn run_step(state: (DIR, (isize, isize)), step: &str) -> (DIR, (isize, isize)) {
    let step: Vec<char> = step.chars().collect();

    let new_dir = step[0];
    let amount = step[1].to_digit(10).unwrap() as isize;

    let cur_dir = state.0;
    let pos = state.1;

    match cur_dir {
        DIR::UP => match new_dir {
            'R' => (DIR::RIGHT, (pos.0 + amount, pos.1)),
            'L' => (DIR::LEFT, (pos.0 - amount, pos.1)),
            _ => state,
        },
        DIR::DOWN => match new_dir {
            'R' => (DIR::LEFT, (pos.0 - amount, pos.1)),
            'L' => (DIR::RIGHT, (pos.0 + amount, pos.1)),
            _ => state,
        },
        DIR::LEFT => match new_dir {
            'R' => (DIR::UP, (pos.0, pos.1 + amount)),
            'L' => (DIR::DOWN, (pos.0, pos.1 - amount)),
            _ => state,
        },
        DIR::RIGHT => match new_dir {
            'R' => (DIR::DOWN, (pos.0, pos.1 - amount)),
            'L' => (DIR::UP, (pos.0, pos.1 + amount)),
            _ => state,
        },
    }
}

pub fn puzzle1(input_filename: &str) -> isize {
    let input = lib::read_lines(input_filename);

    let state = input[0]
        .split(",")
        .map(|s| s.trim())
        .fold((DIR::UP, (0, 0)), run_step);

    println!("{:?}", state);

    (state.1 .0).abs() + (state.1 .1).abs()
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    0
}

#[cfg(test)]
mod tests {
    use crate::y2016::day01;

    #[test]
    fn test1() {
        assert_eq!(day01::puzzle1("src/y2016/day01/test1.txt"), 5);
        assert_eq!(day01::puzzle1("src/y2016/day01/test2.txt"), 2);
        assert_eq!(day01::puzzle1("src/y2016/day01/test3.txt"), 12);
        assert_eq!(day01::puzzle1("src/y2016/day01/input.txt"), 13);
    }

    #[test]
    fn test2() {
        // assert_eq!(day01::puzzle2("src/y2016/day01/test.txt"), 1);
        // assert_eq!(day01::puzzle2("src/y2016/day01/input.txt"), 1);
    }
}
