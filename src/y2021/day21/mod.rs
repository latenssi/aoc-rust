use crate::lib;

fn parse_pos(input: &str) -> usize {
    input
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn dirac_slow(
    (p1_pos, p1_score): (usize, usize),
    (p2_pos, p2_score): (usize, usize),
    roll_count: usize,
) -> (usize, usize) {
    if p1_score >= 21 {
        (1, 0)
    } else if p2_score >= 21 {
        (0, 1)
    } else {
        let p1_turn = (roll_count / 3) % 2 == 0;
        let turn_idx = roll_count % 3; // 0, 1, 2

        [1, 2, 3]
            .into_iter()
            .map(|d| {
                if p1_turn {
                    let p1_pos = ((p1_pos + d - 1) % 10) + 1;
                    let mut p1_score = p1_score;
                    if turn_idx == 2 {
                        p1_score += p1_pos
                    }
                    dirac_slow((p1_pos, p1_score), (p2_pos, p2_score), roll_count + 1)
                } else {
                    let p2_pos = ((p2_pos + d - 1) % 10) + 1;
                    let mut p2_score = p2_score;
                    if turn_idx == 2 {
                        p2_score += p2_pos
                    }
                    dirac_slow((p1_pos, p1_score), (p2_pos, p2_score), roll_count + 1)
                }
            })
            .fold((0, 0), |acc, c| (acc.0 + c.0, acc.1 + c.1))
    }
}

fn dirac_fast(
    rules: &[(usize, usize)], // ways, total_roll
    p1: (usize, usize),       // pos, score
    p2: (usize, usize),       // pos, score
    ways_so_far: usize,
    p1_turn: bool,
) -> (usize, usize) {
    if p1.1 >= 21 {
        (ways_so_far, 0)
    } else if p2.1 >= 21 {
        (0, ways_so_far)
    } else {
        rules
            .into_iter()
            .map(|(ways, total_roll)| {
                let mut p1 = p1;
                let mut p2 = p2;
                let ways_so_far = ways_so_far * ways;
                if p1_turn {
                    p1.0 = ((p1.0 + total_roll - 1) % 10) + 1;
                    p1.1 += p1.0;
                } else {
                    p2.0 = ((p2.0 + total_roll - 1) % 10) + 1;
                    p2.1 += p2.0;
                }
                dirac_fast(rules, p1, p2, ways_so_far, !p1_turn)
            })
            .fold((0, 0), |acc, c| (acc.0 + c.0, acc.1 + c.1))
    }
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let mut p1_pos = parse_pos(&input[0]);
    let mut p2_pos = parse_pos(&input[1]);

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut d_rolls = 0;
    let mut d = 0;

    loop {
        let mut total = 0;
        for _ in 0..3 {
            d = d % 100 + 1;
            d_rolls += 1;
            total += d;
        }
        if d_rolls % 2 == 1 {
            p1_pos = ((p1_pos + total - 1) % 10) + 1;
            p1_score += p1_pos;
        } else {
            p2_pos = ((p2_pos + total - 1) % 10) + 1;
            p2_score += p2_pos;
        }
        if p1_score >= 1000 || p2_score >= 1000 {
            break;
        }
    }

    ([p1_score, p2_score].iter().min().unwrap() * d_rolls) as usize
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);
    let p1_pos = parse_pos(&input[0]);
    let p2_pos = parse_pos(&input[1]);

    // After 3 dice rolls there are
    // 1 way(s) to get a total of 3
    // 3 way(s) to get a total of 4
    // 6 way(s) to get a total of 5
    // 7 way(s) to get a total of 6
    // 6 way(s) to get a total of 7
    // 3 way(s) to get a total of 8
    // 1 way(s) to get a total of 9
    let d = dirac_fast(
        &[(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)],
        (p1_pos, 0),
        (p2_pos, 0),
        1,
        true,
    );

    d.0.max(d.1)
}

#[cfg(test)]
mod tests {
    use crate::y2021::day21;

    #[test]
    fn test1() {
        assert_eq!(day21::puzzle1("src/y2021/day21/test.txt"), 739785);
        assert_eq!(day21::puzzle1("src/y2021/day21/input.txt"), 713328);
    }

    #[test]
    fn test2() {
        assert_eq!(day21::puzzle2("src/y2021/day21/test.txt"), 444356092776315);
        assert_eq!(day21::puzzle2("src/y2021/day21/input.txt"), 92399285032143);
    }
}
