use crate::lib;

// (x, y)
fn parse_point(input: String) -> (i32, i32) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let amount: i32 = parts[1].parse().expect("error while parsing");
    match parts[0] {
        "forward" => (amount, 0),
        "up" => (0, amount),
        "down" => (0, -1 * amount),
        _ => (0, 0),
    }
}

fn travel(state: (i32, i32), step: (i32, i32)) -> (i32, i32) {
    // state: (x, y)
    // step: (x, y)
    let x = state.0 + step.0;
    let y = state.1 + step.1;
    (x, y)
}

fn travel_with_aim(state: (i32, i32, i32), step: (i32, i32)) -> (i32, i32, i32) {
    // state: (x, y, aim)
    // step: (x, aim)
    let aim = state.2 + step.1;
    let x = state.0 + step.0;
    let y = state.1 + step.0 * aim;
    (x, y, aim)
}

fn flip_y_2(p: (i32, i32)) -> (i32, i32) {
    (p.0, -1 * p.1)
}

fn flip_y_3(p: (i32, i32, i32)) -> (i32, i32, i32) {
    (p.0, -1 * p.1, p.2)
}

fn multiply_2(p: (i32, i32)) -> i32 {
    p.0 * p.1
}

fn multiply_3(p: (i32, i32, i32)) -> i32 {
    p.0 * p.1
}

pub fn puzzle1(input_filename: &str) -> i32 {
    let pos = lib::read_lines(input_filename)
        .into_iter()
        .map(parse_point)
        .fold((0, 0), travel);

    multiply_2(flip_y_2(pos))
}

pub fn puzzle2(input_filename: &str) -> i32 {
    let pos = lib::read_lines(input_filename)
        .into_iter()
        .map(parse_point)
        .fold((0, 0, 0), travel_with_aim);

    multiply_3(flip_y_3(pos))
}

#[cfg(test)]
mod tests {
    use crate::y2021::day02;

    #[test]
    fn test1() {
        assert_eq!(day02::puzzle1("src/y2021/day02/test.txt"), 150);
        assert_eq!(day02::puzzle1("src/y2021/day02/input.txt"), 1990000);
    }

    #[test]
    fn test2() {
        assert_eq!(day02::puzzle2("src/y2021/day02/test.txt"), 900);
        assert_eq!(day02::puzzle2("src/y2021/day02/input.txt"), 1975421260);
    }
}
